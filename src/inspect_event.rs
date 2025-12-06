use evdev::{EvdevEnum, KeyCode};
use input::event::{
    PointerEvent,
    pointer::{Axis, PointerScrollEvent},
};
pub(crate) use input::{Event, event::EventTrait};

pub fn inspect_event(event: &Event) -> Option<String> {
    let device = event.device();
    let prefix = format!("{} ({})", device.sysname(), device.name());

    #[allow(deprecated)]
    if let Event::Pointer(PointerEvent::Axis(_)) = event {
        return None;
    }

    match event {
        Event::Pointer(ev) => Some(format!(
            "{prefix}: Pointer({})",
            match ev {
                PointerEvent::Motion(e) => format!(
                    "Motion({}/{}, unaccel={}/{})",
                    e.dx(),
                    e.dy(),
                    e.dx_unaccelerated(),
                    e.dy_unaccelerated()
                ),
                PointerEvent::MotionAbsolute(e) =>
                    format!("MotionAbsolute({}/{})", e.absolute_x(), e.absolute_y()),
                PointerEvent::Button(e) => format!(
                    "Button({}({:?}), {:?})",
                    e.button(),
                    KeyCode::from_index(e.button().try_into().unwrap_or_default()),
                    e.button_state()
                ),
                PointerEvent::ScrollWheel(e) => format!(
                    "ScrollWheel({}/{}, v120={}/{})",
                    e.scroll_value(Axis::Horizontal),
                    e.scroll_value(Axis::Vertical),
                    e.scroll_value_v120(Axis::Horizontal),
                    e.scroll_value_v120(Axis::Vertical),
                ),
                PointerEvent::ScrollFinger(e) => format!(
                    "ScrollFinger({}/{})",
                    e.scroll_value(Axis::Horizontal),
                    e.scroll_value(Axis::Vertical)
                ),
                PointerEvent::ScrollContinuous(e) => format!(
                    "ScrollContinuous({}/{})",
                    e.scroll_value(Axis::Horizontal),
                    e.scroll_value(Axis::Vertical)
                ),
                _ => format!("{:?}", ev),
            }
        )),
        _ => Some(format!("{prefix}: {event:?}")),
    }
}
