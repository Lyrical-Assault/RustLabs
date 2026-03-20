pub trait Logger {
    /// Помещает в лог сообщения заданного уровня.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

// Реализация Filter с использованием замыкания
struct Filter<L, F> where F: Fn(u8, &str) -> bool {
    inner: L,
    predicate: F,
}

impl<L, F> Filter<L, F> 
where F: Fn(u8, &str) -> bool {
    fn new(inner: L, predicate: F) -> Self {
        Filter { inner, predicate }
    }
}

impl<L, F> Logger for Filter<L, F> 
where L: Logger, F: Fn(u8, &str) -> bool {
    fn log(&self, verbosity: u8, message: &str) {
        if (self.predicate)(verbosity, message) {
            self.inner.log(verbosity, message);
        }
    }
}

fn main() {
    let logger = Filter::new(StderrLogger, |verbosity, msg| {
        verbosity <= 3 && msg.contains("yikes")
    });
    
    logger.log(5, "FYI");
    logger.log(1, "yikes, something went wrong");
    logger.log(2, "uhoh");
}