use reqwest::blocking::Client;
use reqwest::Url;
use scraper::{Html, Selector};
use std::collections::{HashSet, VecDeque};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
use thiserror::Error;

const MAX_URLS: usize = 100;
const WORKERS: usize = 8;

#[derive(Error, Debug)]
enum CrawlError {
    #[error("ошибка запроса: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("неправильный http ответ: {0}")]
    BadResponse(String),
}

#[derive(Debug, Clone)]
struct CrawlCommand {
    url: Url,
    extract_links: bool,
}

#[derive(Debug, Default)]
struct CrawlStats {
    processed_pages: usize,
    successful_pages: usize,
    failed_pages: usize,
    total_found_links: usize,
    queued_urls: usize,
}

#[derive(Debug)]
struct SharedState {
    queue: VecDeque<CrawlCommand>,
    visited: HashSet<String>,
    active_workers: usize,
    finished: bool,
    stats: CrawlStats,
}

impl SharedState {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            visited: HashSet::new(),
            active_workers: 0,
            finished: false,
            stats: CrawlStats::default(),
        }
    }
}

fn visit_page(client: &Client, command: &CrawlCommand) -> Result<Vec<Url>, CrawlError> {
    println!("Проверяем {}", command.url);

    let response = client.get(command.url.clone()).send()?;
    if !response.status().is_success() {
        return Err(CrawlError::BadResponse(response.status().to_string()));
    }

    if !command.extract_links {
        return Ok(Vec::new());
    }

    let base_url = response.url().to_owned();
    let body_text = response.text()?;
    let document = Html::parse_document(&body_text);

    let selector = Selector::parse("a").unwrap();
    let href_values = document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"));

    let mut link_urls = Vec::new();

    for href in href_values {
        match base_url.join(href) {
            Ok(link_url) => link_urls.push(link_url),
            Err(err) => {
                println!(
                    "Ссылку на странице {} невозможно разобрать {:?}: {}",
                    base_url, href, err
                );
            }
        }
    }

    println!("На странице найдено ссылок: {}", link_urls.len());

    Ok(link_urls)
}

fn is_same_domain(url: &Url, domain: &str) -> bool {
    match url.domain() {
        Some(d) => d == domain || d.ends_with(&format!(".{domain}")),
        None => false,
    }
}

fn normalize_url(url: &Url) -> String {
    let mut normalized = url.clone();
    normalized.set_fragment(None);
    normalized.to_string()
}

fn worker(
    id: usize,
    client: Client,
    domain: String,
    shared: Arc<(Mutex<SharedState>, Condvar)>,
) {
    loop {
        let command = {
            let (lock, cvar) = &*shared;
            let mut state = lock.lock().unwrap();

            loop {
                if state.finished {
                    return;
                }

                if let Some(cmd) = state.queue.pop_front() {
                    state.active_workers += 1;
                    break cmd;
                }

                if state.active_workers == 0 {
                    state.finished = true;
                    cvar.notify_all();
                    return;
                }

                state = cvar.wait(state).unwrap();
            }
        };

        let result = visit_page(&client, &command);

        let (lock, cvar) = &*shared;
        let mut state = lock.lock().unwrap();

        state.stats.processed_pages += 1;

        match result {
            Ok(links) => {
                state.stats.successful_pages += 1;
                state.stats.total_found_links += links.len();

                for link in links {
                    if state.visited.len() >= MAX_URLS {
                        break;
                    }

                    if !is_same_domain(&link, &domain) {
                        continue;
                    }

                    let normalized = normalize_url(&link);
                    if state.visited.insert(normalized) {
                        state.queue.push_back(CrawlCommand {
                            url: link,
                            extract_links: true,
                        });
                        state.stats.queued_urls += 1;
                    }
                }
            }
            Err(err) => {
                state.stats.failed_pages += 1;
                println!("Поток {id}: ошибка при обработке {}: {}", command.url, err);
            }
        }

        state.active_workers -= 1;
        cvar.notify_all();
    }
}

fn main() {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0 Safari/537.36")
        .build()
        .unwrap();

    let start_url = Url::parse("https://metanit.com").unwrap();
    let domain = start_url.domain().unwrap().to_string();

    let shared = Arc::new((Mutex::new(SharedState::new()), Condvar::new()));

    {
        let (lock, _) = &*shared;
        let mut state = lock.lock().unwrap();

        let normalized = normalize_url(&start_url);
        state.visited.insert(normalized);
        state.queue.push_back(CrawlCommand {
            url: start_url,
            extract_links: true,
        });
        state.stats.queued_urls = 1;
    }

    let mut handles = Vec::new();

    for id in 0..WORKERS {
        let client = client.clone();
        let domain = domain.clone();
        let shared = Arc::clone(&shared);

        handles.push(thread::spawn(move || {
            worker(id, client, domain, shared);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let (lock, _) = &*shared;
    let state = lock.lock().unwrap();

    println!();
    println!("Список ссылок");
    for url in &state.visited {
        println!("{url}");
    }

    println!();
    println!("Обработано страниц: {}", state.stats.processed_pages);
    println!("Успешно открыто страниц: {}", state.stats.successful_pages);
    println!("Ошибок при обработке: {}", state.stats.failed_pages);
    println!("Всего найдено ссылок на страницах: {}", state.stats.total_found_links);
    println!("Уникальных ссылок сохранено: {}", state.visited.len());
    println!("Всего URL поставлено в очередь: {}", state.stats.queued_urls);
}