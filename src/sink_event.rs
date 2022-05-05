use crate::config;
use crate::errors::{self, Error};
use evdev::{AbsoluteAxisType, EventType, InputEvent, RelativeAxisType};
use input::event::pointer::{Axis, ButtonState, PointerButtonEvent};
use input::event::PointerEvent;

#[cfg(not(feature = "libinput_1_19"))]
use input::event::pointer::PointerAxisEvent;
#[cfg(feature = "libinput_1_19")]
use input::event::pointer::{
    PointerScrollContinuousEvent, PointerScrollEvent, PointerScrollFingerEvent,
    PointerScrollWheelEvent,
};

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

            #[cfg(feature = "libinput_1_19")]
            PointerEvent::ScrollWheel(ev) => Ok(Self(ScrollEvent::Wheel(ev).convert())),
            #[cfg(feature = "libinput_1_19")]
            PointerEvent::ScrollFinger(ev) => Ok(Self(ScrollEvent::Finger(ev).convert())),
            #[cfg(feature = "libinput_1_19")]
            PointerEvent::ScrollContinuous(ev) => Ok(Self(ScrollEvent::Continuous(ev).convert())),

            #[cfg(feature = "libinput_1_19")]
            #[allow(deprecated)]
            PointerEvent::Axis(_) => {
                // We should ignore axis event when to handle scroll events.
                // see LIBINPUT_EVENT_POINTER_AXIS in https://wayland.freedesktop.org/libinput/doc/latest/api/group__base.html
                Ok(Self(Vec::new()))
            }
            #[cfg(not(feature = "libinput_1_19"))]
            PointerEvent::Axis(ev) => Ok(Self(ScrollEvent::Axis(ev).convert())),

            _ => Err(errors::Error::Message(format!(
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

fn convert_button(ev: &PointerButtonEvent, cfg: &config::Device) -> Vec<InputEvent> {
    let source_button = config::Button::from_code(ev.button() as u16);
    let button = cfg.map_button(source_button);
    vec![new_button_event(button.code(), ev.button_state())]
}

enum ScrollEvent<'a> {
    #[cfg(feature = "libinput_1_19")]
    Wheel(&'a PointerScrollWheelEvent),
    #[cfg(feature = "libinput_1_19")]
    Finger(&'a PointerScrollFingerEvent),
    #[cfg(feature = "libinput_1_19")]
    Continuous(&'a PointerScrollContinuousEvent),
    #[cfg(not(feature = "libinput_1_19"))]
    Axis(&'a PointerAxisEvent),
}

impl<'a> ScrollEvent<'a> {
    fn has_axis(&self, axis: Axis) -> bool {
        match self {
            #[cfg(feature = "libinput_1_19")]
            ScrollEvent::Wheel(ev) => ev.has_axis(axis),
            #[cfg(feature = "libinput_1_19")]
            ScrollEvent::Finger(ev) => ev.has_axis(axis),
            #[cfg(feature = "libinput_1_19")]
            ScrollEvent::Continuous(ev) => ev.has_axis(axis),
            #[cfg(not(feature = "libinput_1_19"))]
            ScrollEvent::Axis(ev) => ev.has_axis(axis),
        }
    }

    fn scroll_value(&self, axis: Axis) -> f64 {
        match self {
            #[cfg(feature = "libinput_1_19")]
            ScrollEvent::Wheel(ev) => ev.scroll_value(axis),
            #[cfg(feature = "libinput_1_19")]
            ScrollEvent::Finger(ev) => ev.scroll_value(axis),
            #[cfg(feature = "libinput_1_19")]
            ScrollEvent::Continuous(ev) => ev.scroll_value(axis),
            #[cfg(not(feature = "libinput_1_19"))]
            ScrollEvent::Axis(ev) => ev.axis_value(axis),
        }
    }

    fn scroll_value_v120(&self, axis: Axis) -> f64 {
        match self {
            #[cfg(feature = "libinput_1_19")]
            ScrollEvent::Wheel(ev) => ev.scroll_value_v120(axis),
            _ => self.scroll_value(axis) * 120.0,
        }
    }

    fn convert(&self) -> Vec<InputEvent> {
        let mut res = Vec::new();

        if self.has_axis(Axis::Vertical) {
            res.push(new_relative_event(
                RelativeAxisType::REL_WHEEL,
                -self.scroll_value(Axis::Vertical),
            ));
            res.push(new_relative_event(
                RelativeAxisType::REL_WHEEL_HI_RES,
                -self.scroll_value_v120(Axis::Vertical),
            ));
        }
        if self.has_axis(Axis::Horizontal) {
            res.push(new_relative_event(
                RelativeAxisType::REL_HWHEEL,
                self.scroll_value(Axis::Horizontal),
            ));
            res.push(new_relative_event(
                RelativeAxisType::REL_HWHEEL_HI_RES,
                self.scroll_value_v120(Axis::Horizontal),
            ));
        }
        res
    }
}
