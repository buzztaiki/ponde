use std::os::unix::prelude::*;
use std::path::Path;
use std::sync::{Arc, Mutex};

use input::event::{DeviceEvent, EventTrait};
use input::Event;
use input::{Libinput, LibinputInterface};

use crate::config::Config;
use crate::default_libinput_interface::DefaultLibinputInterface;
use crate::device_fd::{DeviceFd, DeviceFdMap};
use crate::errors::Error;
use crate::sink_device::SinkDevice;

type DeviceFdMapPtr = Arc<Mutex<DeviceFdMap>>;

pub struct App<'a> {
    config: &'a Config,
    device_fd_map: DeviceFdMapPtr,
    sink_device: SinkDevice,
}

impl<'a> App<'a> {
    pub fn new(config: &'a Config, sink_device: SinkDevice) -> Self {
        let device_fd_map = Arc::new(Mutex::new(DeviceFdMap::default()));
        Self {
            config,
            device_fd_map,
            sink_device,
        }
    }

    pub fn event_loop(&mut self) -> Result<(), Error> {
        let mut libinput =
            Libinput::new_with_udev(AppLibinputInterface::new(self.device_fd_map.clone()));
        libinput
            .udev_assign_seat("seat0")
            .expect("failed to assign seat");

        loop {
            libinput.dispatch().unwrap();
            for event in &mut libinput {
                if let Err(e) = self.handle_event(&event) {
                    eprintln!(
                        "failed to handle event: device={}: {}",
                        event.device().name(),
                        e
                    );
                }
            }
        }
    }

    fn handle_event(&mut self, event: &Event) -> Result<(), Error> {
        let device = &event.device();
        let _device_config = match self.config.matched_device(&device.into()) {
            Some(x) => x,
            None => return Ok(()),
        };

        match event {
            Event::Device(DeviceEvent::Added(_)) => {
                let udev_device = udev_device(device)
                    .ok_or_else(|| Error::Error("failed to get udev_device".to_string()))?;

                let devnode = udev_device
                    .devnode()
                    .ok_or_else(|| Error::Error("failed to get devnode".to_string()))?;

                {
                    let map = self.device_fd_map.lock().unwrap();
                    eprintln!("device_fd: {:?}", map.get_by_path(devnode));
                }
            }
            Event::Pointer(ev) => {
                let sink_event = ev.try_into()?;
                self.sink_device.send_event(&sink_event)?;
            }
            _ => return Err(Error::Error(format!("unexpected event: {:?}", event))),
        }
        Ok(())
    }
}

fn udev_device(device: &input::Device) -> Option<udev::Device> {
    unsafe { device.udev_device() }
}

struct AppLibinputInterface {
    iface: DefaultLibinputInterface,
    device_fd_map: DeviceFdMapPtr,
}

impl AppLibinputInterface {
    fn new(device_fd_map: DeviceFdMapPtr) -> Self {
        Self {
            iface: DefaultLibinputInterface::default(),
            device_fd_map,
        }
    }
}

impl LibinputInterface for AppLibinputInterface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<RawFd, i32> {
        let fd = self.iface.open_restricted(path, flags)?;
        let mut map = self.device_fd_map.lock().unwrap();
        map.insert(DeviceFd::new(fd, path));
        Ok(fd)
    }

    fn close_restricted(&mut self, fd: RawFd) {
        self.iface.close_restricted(fd);
        let mut map = self.device_fd_map.lock().unwrap();
        map.remove_by_fd(fd);
    }
}
