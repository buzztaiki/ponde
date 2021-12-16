use crate::errors::{self, Error};

#[derive(Debug)]
pub struct SinkEvent {}

impl TryFrom<&input::event::PointerEvent> for SinkEvent {
    type Error = Error;

    fn try_from(ev: &input::event::PointerEvent) -> Result<Self, Self::Error> {
        match ev {
            input::event::PointerEvent::Motion(_) => Ok(Self {}),
            input::event::PointerEvent::MotionAbsolute(_) => Ok(Self {}),
            input::event::PointerEvent::Button(_) => Ok(Self {}),
            input::event::PointerEvent::ScrollWheel(_) => Ok(Self {}),
            input::event::PointerEvent::ScrollFinger(_) => Ok(Self {}),
            input::event::PointerEvent::ScrollContinuous(_) => Ok(Self {}),
            // input::event::PointerEvent::Axis(_) => Ok(Self{}),
            _ => Err(errors::Error::Error(format!(
                "unexpected pointer event: {:?}",
                ev
            ))),
        }
    }
}
