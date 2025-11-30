use evdev::uinput::{VirtualDevice, VirtualDeviceBuilder};
use evdev::{AttributeSet, KeyCode, RelativeAxisCode};

use crate::errors::Error;
use crate::sink_event::SinkEvent;

pub struct SinkDevice {
    vdevice: VirtualDevice,
    name: String,
}

impl SinkDevice {
    pub fn create(name: &str) -> Result<Self, Error> {
        let mut keys = AttributeSet::<KeyCode>::new();
        // Note: when keyboard keys are enabled, it is not detected as a mouse
        for code in KeyCode::BTN_0.code()..=KeyCode::BTN_THUMBR.code() {
            keys.insert(KeyCode::new(code));
        }

        let mut rel_axes = AttributeSet::<RelativeAxisCode>::new();
        for code in RelativeAxisCode::REL_X.0..=RelativeAxisCode::REL_HWHEEL_HI_RES.0 {
            rel_axes.insert(RelativeAxisCode(code));
        }

        let vdevice = VirtualDeviceBuilder::new()?
            .name(name)
            .with_keys(&keys)?
            .with_relative_axes(&rel_axes)?
            .build()?;
        Ok(Self {
            vdevice,
            name: name.to_string(),
        })
    }

    pub fn send_event(&mut self, event: &SinkEvent) -> Result<(), Error> {
        self.vdevice.emit(event.as_ref())?;
        Ok(())
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
