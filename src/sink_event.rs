use evdev::{AbsoluteAxisCode, EventType, InputEvent, RelativeAxisCode};
use input::event::pointer::{
    Axis, ButtonState, PointerButtonEvent, PointerScrollEvent, PointerScrollWheelEvent,
};
use input::event::PointerEvent;

use crate::config;
use crate::errors::{self, Error};

#[derive(Debug)]
pub struct SinkEvent(Vec<InputEvent>);

impl AsRef<Vec<InputEvent>> for SinkEvent {
    fn as_ref(&self) -> &Vec<InputEvent> {
        &self.0
    }
}

impl SinkEvent {
    pub fn from_pointer_event(
        event: &PointerEvent,
        device_config: &config::Device,
    ) -> Result<Self, Error> {
        match event {
            PointerEvent::Motion(ev) => Ok(Self(vec![
                new_relative_event(RelativeAxisCode::REL_X, ev.dx()),
                new_relative_event(RelativeAxisCode::REL_Y, ev.dy()),
            ])),
            PointerEvent::MotionAbsolute(ev) => Ok(Self(vec![
                new_absolute_event(AbsoluteAxisCode::ABS_X, ev.absolute_x()),
                new_absolute_event(AbsoluteAxisCode::ABS_Y, ev.absolute_y()),
            ])),
            PointerEvent::Button(ev) => Ok(Self(convert_button(ev, device_config))),
            PointerEvent::ScrollWheel(ev) => Ok(Self(convert_wheel_event(ev))),
            PointerEvent::ScrollFinger(ev) => Ok(Self(convert_scroll_event(ev))),
            PointerEvent::ScrollContinuous(ev) => Ok(Self(convert_scroll_event(ev))),
            #[allow(deprecated)]
            PointerEvent::Axis(_) => {
                // We should ignore axis event when to handle scroll events.
                // see LIBINPUT_EVENT_POINTER_AXIS in https://wayland.freedesktop.org/libinput/doc/latest/api/group__base.html
                Ok(Self(Vec::new()))
            }
            _ => Err(errors::Error::Message(format!(
                "unexpected pointer event: {:?}",
                event
            ))),
        }
    }
}

fn new_relative_event(axis_type: RelativeAxisCode, value: f64) -> InputEvent {
    InputEvent::new(EventType::RELATIVE, axis_type.0, value as i32)
}

fn new_absolute_event(axis_type: AbsoluteAxisCode, value: f64) -> InputEvent {
    InputEvent::new(EventType::ABSOLUTE, axis_type.0, value as i32)
}

fn new_button_event(button: u16, state: ButtonState) -> InputEvent {
    InputEvent::new(
        EventType::KEY,
        button,
        match state {
            ButtonState::Pressed => 1,
            ButtonState::Released => 0,
        },
    )
}

fn dispatch_scroll_event(
    ev: &impl PointerScrollEvent,
    f: impl Fn(Axis) -> (f64, f64),
) -> Vec<InputEvent> {
    let mut res = Vec::new();
    if ev.has_axis(Axis::Vertical) {
        let (v, v120) = f(Axis::Vertical);
        res.push(new_relative_event(RelativeAxisCode::REL_WHEEL, -v));
        res.push(new_relative_event(
            RelativeAxisCode::REL_WHEEL_HI_RES,
            -v120,
        ));
    }
    if ev.has_axis(Axis::Horizontal) {
        let (v, v120) = f(Axis::Horizontal);
        res.push(new_relative_event(RelativeAxisCode::REL_HWHEEL, v));
        res.push(new_relative_event(
            RelativeAxisCode::REL_HWHEEL_HI_RES,
            v120,
        ));
    }
    res
}

fn convert_scroll_event(ev: &impl PointerScrollEvent) -> Vec<InputEvent> {
    dispatch_scroll_event(ev, |axis| {
        (ev.scroll_value(axis), ev.scroll_value(axis) * 120.0)
    })
}

fn convert_wheel_event(ev: &PointerScrollWheelEvent) -> Vec<InputEvent> {
    dispatch_scroll_event(ev, |axis| {
        (ev.scroll_value(axis), ev.scroll_value_v120(axis))
    })
}

fn convert_button(ev: &PointerButtonEvent, cfg: &config::Device) -> Vec<InputEvent> {
    let source_button = config::Button::from_code(ev.button() as u16);
    let button = cfg.map_button(source_button);
    vec![new_button_event(button.code(), ev.button_state())]
}
