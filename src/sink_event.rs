use evdev::{AbsoluteAxisType, EventType, InputEvent, RelativeAxisType};
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
                new_relative_event(RelativeAxisType::REL_X, ev.dx()),
                new_relative_event(RelativeAxisType::REL_Y, ev.dy()),
            ])),
            PointerEvent::MotionAbsolute(ev) => Ok(Self(vec![
                new_absolute_event(AbsoluteAxisType::ABS_X, ev.absolute_x()),
                new_absolute_event(AbsoluteAxisType::ABS_Y, ev.absolute_y()),
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
            _ => Err(errors::Error::Error(format!(
                "unexpected pointer event: {:?}",
                event
            ))),
        }
    }
}

fn new_relative_event(axis_type: RelativeAxisType, value: f64) -> InputEvent {
    InputEvent::new(EventType::RELATIVE, axis_type.0, value as i32)
}

fn new_absolute_event(axis_type: AbsoluteAxisType, value: f64) -> InputEvent {
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
    f: impl Fn(Axis) -> InputEvent,
) -> Vec<InputEvent> {
    let mut res = Vec::new();
    if ev.has_axis(Axis::Vertical) {
        let x = f(Axis::Vertical);
        res.push(InputEvent::new(x.event_type(), x.code(), -x.value()));
    }
    if ev.has_axis(Axis::Horizontal) {
        res.push(f(Axis::Horizontal));
    }
    res
}

fn convert_scroll_event(ev: &impl PointerScrollEvent) -> Vec<InputEvent> {
    dispatch_scroll_event(ev, |axis| match axis {
        Axis::Vertical => new_relative_event(RelativeAxisType::REL_WHEEL, ev.scroll_value(axis)),
        Axis::Horizontal => new_relative_event(RelativeAxisType::REL_HWHEEL, ev.scroll_value(axis)),
    })
}

fn convert_wheel_event(ev: &PointerScrollWheelEvent) -> Vec<InputEvent> {
    let mut res = convert_scroll_event(ev);
    res.append(&mut dispatch_scroll_event(ev, |axis| match axis {
        Axis::Vertical => new_relative_event(
            RelativeAxisType::REL_WHEEL_HI_RES,
            ev.scroll_value_v120(axis),
        ),
        Axis::Horizontal => new_relative_event(
            RelativeAxisType::REL_HWHEEL_HI_RES,
            ev.scroll_value_v120(axis),
        ),
    }));
    res
}

fn convert_button(ev: &PointerButtonEvent, cfg: &config::Device) -> Vec<InputEvent> {
    let source_button = config::Button::from_code(ev.button() as u16);
    let button = cfg.map_button(source_button);
    vec![new_button_event(button.code(), ev.button_state())]
}
