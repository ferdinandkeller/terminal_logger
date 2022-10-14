use terminal_logger::TerminalLogger;
use std::thread;

fn main() {
  let mut terminal_logger = TerminalLogger::new();
  terminal_logger.start();
  terminal_logger.display("I like potatoes".to_owned());
  thread::sleep(std::time::Duration::from_millis(5000));
  terminal_logger.stop();
  thread::sleep(std::time::Duration::from_millis(1000));
  println!("program ends");
}
