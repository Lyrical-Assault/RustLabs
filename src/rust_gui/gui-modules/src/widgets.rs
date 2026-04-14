pub mod button;
pub mod label;
pub mod window;

pub use button::Button;
pub use label::Label;
pub use window::Window;

pub trait Widget {
    /// Ширина self.
    fn width(&self) -> usize;

    /// Прорисовка виджета в буфер.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) -> std::fmt::Result;

    /// Прорисовка виджета.
    fn draw(&self) {
        let mut buffer = String::new();
        if let Err(e) = self.draw_into(&mut buffer) {
            eprintln!("Ошибка при отрисовке: {}", e);
        }
        println!("{}", buffer);
    }
}

