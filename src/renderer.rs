use std::{
  thread,
  sync::mpsc,
  time
};

use crate::utils::Command;

pub struct Renderer {
  keep_rendering: bool,
  rendering_loop_duration: time::Duration,
  receiving_channel: mpsc::Receiver<Command>,
}

impl Renderer {
  pub fn new(receiving_channel: mpsc::Receiver<Command>) -> Self {
    Self {
      keep_rendering: true,
      rendering_loop_duration: time::Duration::from_millis(1_000),
      receiving_channel
    }
  }

  pub fn start(&mut self) {
    // keep rendering until receiving a call to stop
    while self.keep_rendering {
      // register when the loop started
      let loop_start_time = time::Instant::now();

      // do stuff
      self.render();

      // sleep until the loop duration has passed, taking into account the execution time
      let loop_end_time = time::Instant::now();
      let loop_execution_duration = loop_end_time - loop_start_time;
      let sleep_duration = self.rendering_loop_duration - loop_execution_duration;
      thread::sleep(sleep_duration);
    }
  }

  pub fn render(&mut self) {
    // as long as there are commands in the channel, keep rendering
    while let Ok(command) = self.receiving_channel.try_recv() {
      // do stuff
      println!("renderer is running");
    }
  }
}
