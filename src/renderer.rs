use std::{
  thread,
  sync::mpsc,
  time
};
use console::Term;

use crate::utils::Command;

const ANIMATION_CHARS: [char; 8] = ['⡏' ,'⠟', '⠻', '⢹', '⣸', '⣴', '⣦', '⣇'];

pub struct Renderer {
  keep_rendering: bool,
  rendering_loop_duration: time::Duration,
  receiving_channel: mpsc::Receiver<Command>,
  to_render: Vec<String>,
  animation_index: usize,
}

impl Renderer {
  pub fn new(receiving_channel: mpsc::Receiver<Command>) -> Self {
    Self {
      keep_rendering: true,
      rendering_loop_duration: time::Duration::from_millis(100),
      receiving_channel,
      to_render: Vec::new(),
      animation_index: 0,
    }
  }

  pub fn start(&mut self) {
    let term = Term::stdout();
    term.hide_cursor().unwrap();

    // keep rendering until receiving a call to stop
    while self.keep_rendering {
      // register when the loop started
      let loop_start_time = time::Instant::now();

      // re-render the terminal
      self.clear();
      self.render();

      // increment the animation index
      self.animation_index = (self.animation_index + 1) % ANIMATION_CHARS.len();

      // as long as there are commands in the channel, process them
      while let Ok(command) = self.receiving_channel.try_recv() {
        self.process_command(command);
      }

      // sleep until the loop duration has passed, taking into account the execution time
      let loop_end_time = time::Instant::now();
      let loop_execution_duration = loop_end_time - loop_start_time;
      let sleep_duration = self.rendering_loop_duration - loop_execution_duration;
      thread::sleep(sleep_duration);
    }

    
    term.show_cursor().unwrap();
  }

  fn process_command(&mut self, command: Command) {
    match command {
      Command::Stop => self.stop(),
      Command::Display(message) => self.to_render.push(message),
    }
  }

  fn render(&mut self) {
    let term = Term::buffered_stdout();
    for message in &self.to_render {
      term.write_line(&format!("{} {}", ANIMATION_CHARS[self.animation_index],  message)).unwrap();
    }
    term.flush().unwrap();
  }

  fn clear(&mut self) {
    let term = Term::stdout();
    term.clear_last_lines(1).unwrap();
  }

  fn stop(&mut self) {
    self.keep_rendering = false;
    self.clear();
    println!("renderer stopped");
  }
}
