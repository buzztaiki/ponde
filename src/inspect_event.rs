use evdev::{EvdevEnum, KeyCode};
use input::event::{
    PointerEvent,
    pointer::{Axis, PointerScrollEvent, PointerScrollWheelEvent},
};
pub(crate) use input::{Event, event::EventTrait};

fn scroll_value(e: &impl PointerScrollEvent, ax: Axis) -> f64 {
    if e.has_axis(ax) {
        e.scroll_value(ax)
    } else {
        0.0
    }
}

fn scroll_value_v120(e: &PointerScrollWheelEvent, ax: Axis) -> f64 {
    if e.has_axis(ax) {
        e.scroll_value_v120(ax)
    } else {
        0.0
    }
}

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
                    scroll_value(e, Axis::Horizontal),
                    scroll_value(e, Axis::Vertical),
                    scroll_value_v120(e, Axis::Horizontal),
                    scroll_value_v120(e, Axis::Vertical),
                ),
                PointerEvent::ScrollFinger(e) => format!(
                    "ScrollFinger({}/{})",
                    scroll_value(e, Axis::Horizontal),
                    scroll_value(e, Axis::Vertical),
                ),
                PointerEvent::ScrollContinuous(e) => format!(
                    "ScrollContinuous({}/{})",
                    scroll_value(e, Axis::Horizontal),
                    scroll_value(e, Axis::Vertical),
                ),
                _ => format!("{:?}", ev),
            }
        )),
        _ => Some(format!("{prefix}: {event:?}")),
    }
}
