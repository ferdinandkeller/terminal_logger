use terminal_logger::TerminalLogger;
use std::thread;

fn main() {
  let mut terminal_logger = TerminalLogger::new();
  terminal_logger.start();
  for _ in 0..100 {
    terminal_logger.render("I like potatoes".to_owned());
    thread::sleep(std::time::Duration::from_millis(500));
  }
}
