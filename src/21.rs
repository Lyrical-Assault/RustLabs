use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::Sender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();

        thread::sleep(Duration::from_millis(10));
    }

    fn eat(&self) {
        let _left = self.left_fork.lock().unwrap();
        let _right = self.right_fork.lock().unwrap();

        println!("{} ест ...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] =
    &["Сократ", "Гипатия", "Платон", "Аристотель", "Пифагор"];

fn main() {
    let (tx, rx) = mpsc::channel();

    let forks: Vec<_> = (0..PHILOSOPHERS.len())
        .map(|_| Arc::new(Mutex::new(Fork)))
        .collect();

    let mut philosophers = Vec::new();

    for i in 0..PHILOSOPHERS.len() {
        let left = Arc::clone(&forks[i]);
        let right = Arc::clone(&forks[(i + 1) % PHILOSOPHERS.len()]);

        let (left_fork, right_fork) = if i == PHILOSOPHERS.len() - 1 {
            (right, left)
        } else {
            (left, right)
        };

        philosophers.push(Philosopher {
            name: PHILOSOPHERS[i].to_string(),
            left_fork,
            right_fork,
            thoughts: tx.clone(),
        });
    }

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            thread::spawn(move || {
                for _ in 0..100 {
                    p.eat();
                    p.think();
                }
            })
        })
        .collect();

    drop(tx);

    for handle in handles {
        handle.join().unwrap();
    }

    for thought in rx {
        println!("{thought}");
    }
}