use std::{
  thread,
  sync::mpsc
};

use crate::renderer::Renderer;
use crate::utils::Command;

#[derive(Debug, Default)]
pub struct TerminalLogger {
  started: bool,
  sending_channel: Option<mpsc::Sender<Command>>,
  drawing_thread_handle: Option<thread::JoinHandle<()>>,
}

impl TerminalLogger {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn start(&mut self) {
    self.started = true;

    let (sending_channel, receiving_channel) = mpsc::channel();

    let handle = thread::spawn(move || {
      Renderer::new(receiving_channel).start();
    });

    self.sending_channel = Some(sending_channel);
    self.drawing_thread_handle = Some(handle);
  }

  pub fn is_started(&self) -> bool {
    self.started
  }

  pub fn display(&self, message: String) {
    if self.started {
      if let Some(sending_channel) = &self.sending_channel {
        sending_channel.send(Command::Display(message)).unwrap();
      }
    }
  }

  pub fn stop(&mut self) {
    if self.started {
      if let Some(sending_channel) = &self.sending_channel {
        sending_channel.send(Command::Stop).unwrap();
      }

      if self.drawing_thread_handle.is_some() {
        self.drawing_thread_handle.take().unwrap().join().unwrap();
      }

      self.started = false;
    }
  }
}
