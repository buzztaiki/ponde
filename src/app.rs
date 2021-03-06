use std::cell::RefCell;
use std::os::unix::prelude::*;
use std::path::Path;
use std::rc::Rc;

use input::event::{DeviceEvent, EventTrait};
use input::Event;
use input::{Libinput, LibinputInterface};
use nix::poll::{poll, PollFd, PollFlags};

use crate::config::Config;
use crate::default_libinput_interface::DefaultLibinputInterface;
use crate::device_fd::{DeviceFd, DeviceFdMap};
use crate::errors::Error;
use crate::sink_device::SinkDevice;
use crate::sink_event::SinkEvent;

type DeviceFdMapPtr = Rc<RefCell<DeviceFdMap>>;

pub struct App<'a> {
    config: &'a Config,
    device_fd_map: DeviceFdMapPtr,
    sink_device: SinkDevice,
}

impl<'a> App<'a> {
    pub fn new(config: &'a Config, sink_device: SinkDevice) -> Self {
        let device_fd_map = Rc::new(RefCell::new(DeviceFdMap::default()));
        Self {
            config,
            device_fd_map,
            sink_device,
        }
    }

    pub fn main_loop(&mut self) -> Result<(), Error> {
        let mut libinput =
            Libinput::new_with_udev(AppLibinputInterface::new(self.device_fd_map.clone()));
        libinput
            .udev_assign_seat("seat0")
            .expect("failed to assign seat");

        let mut poll_fds = [PollFd::new(libinput.as_raw_fd(), PollFlags::POLLIN)];
        while poll(&mut poll_fds, -1)? > -1 {
            libinput.dispatch()?;
            for event in &mut libinput {
                self.handle_event(&event)?;
            }
        }
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) -> Result<(), Error> {
        let mut device = event.device();
        let device_config = match self.config.matched_device(&(&device).into()) {
            Some(x) => x,
            None => return Ok(()),
        };

        if device.name() == self.sink_device.name() {
            return Ok(());
        }

        match event {
            Event::Device(DeviceEvent::Added(_)) => {
                eprintln!(
                    "grab matched device: {} ({})",
                    device.sysname(),
                    device.name()
                );

                device_config.apply_to(&mut device)?;
                let mut map = self.device_fd_map.borrow_mut();
                let device_fd = map.get_by_name_mut(device.sysname()).ok_or_else(|| {
                    Error::Message(format!(
                        "failed to get device_fd: {} ({})",
                        device.sysname(),
                        device.name()
                    ))
                })?;
                device_fd.grab()?;
            }
            Event::Pointer(ev) => {
                let sink_event = SinkEvent::from_pointer_event(ev, device_config)?;
                self.sink_device.send_event(&sink_event)?;
            }
            _ => return Err(Error::Message(format!("unexpected event: {:?}", event))),
        }
        Ok(())
    }
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
        if let Some(device_fd) = DeviceFd::new(fd, path) {
            self.device_fd_map.borrow_mut().insert(device_fd);
        }
        Ok(fd)
    }

    fn close_restricted(&mut self, fd: RawFd) {
        self.iface.close_restricted(fd);
        self.device_fd_map.borrow_mut().remove_by_fd(fd);
    }
}
