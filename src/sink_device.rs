use evdev::uinput::{VirtualDevice, VirtualDeviceBuilder};
use evdev::{AttributeSet, Key, RelativeAxisType};

use crate::errors::Error;
use crate::sink_event::SinkEvent;

pub struct SinkDevice(VirtualDevice);

impl SinkDevice {
    pub fn create(name: &str) -> Result<Self, Error> {
        let mut keys = AttributeSet::<Key>::new();
        // Note: when keyboard keys are enabled, it is not detected as a mouse
        for code in Key::BTN_0.code()..=Key::BTN_THUMBR.code() {
            keys.insert(Key::new(code));
        }

        let mut rel_axes = AttributeSet::<RelativeAxisType>::new();
        for code in RelativeAxisType::REL_X.0..=RelativeAxisType::REL_HWHEEL_HI_RES.0 {
            rel_axes.insert(RelativeAxisType(code));
        }

        let vdevice = VirtualDeviceBuilder::new()?
            .name(name)
            .with_keys(&keys)?
            .with_relative_axes(&rel_axes)?
            .build()?;
        Ok(Self(vdevice))
    }

    pub fn send_event(&mut self, event: &SinkEvent) -> Result<(), Error> {
        self.0.emit(event.as_ref())?;
        Ok(())
    }
}
