use std::io;
use std::os::unix::prelude::*;
use std::path::Path;
use std::sync::{Arc, Mutex};

use input::event::{DeviceEvent, EventTrait};
use input::Event;
use input::{Libinput, LibinputInterface};

use crate::config::Config;
use crate::default_libinput_interface::DefaultLibinputInterface;
use crate::device_fd::{DeviceFd, DeviceFdMap};

type DeviceFdMapPtr = Arc<Mutex<DeviceFdMap>>;

pub struct App<'a> {
    config: &'a Config,
    device_fd_map: DeviceFdMapPtr,
}

impl<'a> App<'a> {
    pub fn new(config: &'a Config) -> Self {
        let device_fd_map = Arc::new(Mutex::new(DeviceFdMap::default()));
        Self {
            config,
            device_fd_map,
        }
    }

    pub fn event_loop(&mut self) -> io::Result<()> {
        let mut libinput =
            Libinput::new_with_udev(AppLibinputInterface::new(self.device_fd_map.clone()));
        libinput
            .udev_assign_seat("seat0")
            .expect("failed to assign seat");

        loop {
            libinput.dispatch().unwrap();
            for event in &mut libinput {
                self.handle_event(&event)?;
            }
        }
    }

    fn handle_event(&mut self, event: &Event) -> io::Result<()> {
        let device = &event.device();
        match event {
            Event::Device(DeviceEvent::Added(_)) => {
                let _device_config = match self.config.matched_device(&device.into()) {
                    Some(x) => x,
                    None => {
                        eprintln!("unmatched device: {}", device.name());
                        return Ok(());
                    }
                };

                let udev_device = match udev_device(device) {
                    Some(x) => x,
                    None => return Ok(()),
                };

                let devnode = match udev_device.devnode() {
                    Some(x) => x,
                    None => return Ok(()),
                };

                {
                    let map = self.device_fd_map.lock().unwrap();
                    eprintln!("device_fd: {:?}", map.get_by_path(devnode));
                }
            }
            _ => eprintln!("unexpected event: {:?}", event),
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
