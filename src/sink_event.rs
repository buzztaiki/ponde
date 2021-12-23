use evdev::{AbsoluteAxisType, EventType, InputEvent, RelativeAxisType};
use input::event::pointer::{Axis, ButtonState, PointerScrollEvent};
use input::event::PointerEvent;

use crate::errors::{self, Error};

#[derive(Debug)]
pub struct SinkEvent(Vec<InputEvent>);

impl TryFrom<&PointerEvent> for SinkEvent {
    type Error = Error;

    fn try_from(event: &PointerEvent) -> Result<Self, Self::Error> {
        match event {
            PointerEvent::Motion(ev) => Ok(Self(vec![
                InputEvent::new(
                    EventType::RELATIVE,
                    RelativeAxisType::REL_X.0,
                    ev.dx() as i32,
                ),
                InputEvent::new(
                    EventType::RELATIVE,
                    RelativeAxisType::REL_Y.0,
                    ev.dy() as i32,
                ),
            ])),
            PointerEvent::MotionAbsolute(ev) => Ok(Self(vec![
                InputEvent::new(
                    EventType::ABSOLUTE,
                    AbsoluteAxisType::ABS_X.0,
                    ev.absolute_x() as i32,
                ),
                InputEvent::new(
                    EventType::ABSOLUTE,
                    AbsoluteAxisType::ABS_Y.0,
                    ev.absolute_y() as i32,
                ),
            ])),
            PointerEvent::Button(ev) => Ok(Self(vec![InputEvent::new(
                EventType::KEY,
                ev.button() as u16,
                match ev.button_state() {
                    ButtonState::Pressed => 1,
                    ButtonState::Released => 0,
                },
            )])),
            PointerEvent::ScrollWheel(ev) => {
                let mut res = Vec::new();
                if ev.has_axis(Axis::Vertical) {
                    res.push(InputEvent::new(
                        EventType::RELATIVE,
                        RelativeAxisType::REL_WHEEL.0,
                        ev.scroll_value(Axis::Vertical) as i32,
                    ));
                    res.push(InputEvent::new(
                        EventType::RELATIVE,
                        RelativeAxisType::REL_WHEEL_HI_RES.0,
                        ev.scroll_value_v120(Axis::Vertical) as i32,
                    ));
                }
                if ev.has_axis(Axis::Horizontal) {
                    res.push(InputEvent::new(
                        EventType::RELATIVE,
                        RelativeAxisType::REL_HWHEEL.0,
                        ev.scroll_value(Axis::Horizontal) as i32,
                    ));
                    res.push(InputEvent::new(
                        EventType::RELATIVE,
                        RelativeAxisType::REL_HWHEEL_HI_RES.0,
                        ev.scroll_value_v120(Axis::Horizontal) as i32,
                    ));
                }
                Ok(Self(res))
            }
            PointerEvent::ScrollFinger(ev) => {
                let mut res = Vec::new();
                if ev.has_axis(Axis::Vertical) {
                    res.push(InputEvent::new(
                        EventType::RELATIVE,
                        RelativeAxisType::REL_WHEEL.0,
                        ev.scroll_value(Axis::Vertical) as i32,
                    ));
                }
                if ev.has_axis(Axis::Horizontal) {
                    res.push(InputEvent::new(
                        EventType::RELATIVE,
                        RelativeAxisType::REL_HWHEEL.0,
                        ev.scroll_value(Axis::Horizontal) as i32,
                    ));
                }
                Ok(Self(res))
            }
            PointerEvent::ScrollContinuous(ev) => {
                let mut res = Vec::new();
                if ev.has_axis(Axis::Vertical) {
                    res.push(InputEvent::new(
                        EventType::RELATIVE,
                        RelativeAxisType::REL_WHEEL.0,
                        ev.scroll_value(Axis::Vertical) as i32,
                    ));
                }
                if ev.has_axis(Axis::Horizontal) {
                    res.push(InputEvent::new(
                        EventType::RELATIVE,
                        RelativeAxisType::REL_HWHEEL.0,
                        ev.scroll_value(Axis::Horizontal) as i32,
                    ));
                }
                Ok(Self(res))
            }
            // PointerEvent::Axis(_) => todo!(),
            _ => Err(errors::Error::Error(format!(
                "unexpected pointer event: {:?}",
                event
            ))),
        }
    }
}
