use crate::errors::Error;
use crate::sink_event::SinkEvent;

#[derive(Debug)]
pub struct SinkDevice {}

impl SinkDevice {
    pub fn create(_name: &str) -> Self {
        Self {}
    }

    pub fn send_event(&mut self, event: &SinkEvent) -> Result<(), Error> {
        eprintln!("todo: send_event: {:?}", event);
        Ok(())
    }
}
