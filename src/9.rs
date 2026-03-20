use std::collections::HashMap;
use std::hash::Hash;

/// Обобщённый счётчик значений типа T.
struct Counter<T> {
    values: HashMap<T, u64>,
}

impl<T: Eq + Hash> Counter<T> {
    /// Создаёт новый счётчик.
    fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    /// Увеличивает счётчик для данного значения.
    fn count(&mut self, value: T) {
        *self.values.entry(value).or_insert(0) += 1;
    }

    /// Возвращает количество появлений заданного значения.
    fn times_seen(&self, value: &T) -> u64 {
        self.values.get(value).copied().unwrap_or_default()
    }
}

fn main() {
    let mut ctr = Counter::new();
    ctr.count(13);
    ctr.count(14);
    ctr.count(16);
    ctr.count(14);
    ctr.count(14);
    ctr.count(11);

    for i in 10..20 {
        println!("Значение {} видели {} раз", i, ctr.times_seen(&i));
    }

    let mut strctr = Counter::new();
    strctr.count("apple");
    strctr.count("orange");
    strctr.count("apple");

    println!("Получили {} яблок", strctr.times_seen(&"apple"));
}