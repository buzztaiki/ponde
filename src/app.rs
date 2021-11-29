use std::io;
use std::os::unix::prelude::*;
use std::path::Path;

use input::{Libinput, LibinputInterface};

use crate::default_libinput_interface::DefaultLibinputInterface;
use crate::device_fd::{DeviceFd, DeviceFdMap};

pub struct App {
    libinput: input::Libinput,
}

impl App {
    pub fn new() -> Self {
        let mut libinput = Libinput::new_with_udev(AppLibinputInterface::default());
        libinput
            .udev_assign_seat("seat0")
            .expect("failed to assign seat");
        Self { libinput }
    }

    pub fn event_loop(&mut self) -> io::Result<()> {
        loop {
            self.libinput.dispatch().unwrap();
            for event in &mut self.libinput {
                eprintln!("{:?}", event);
            }
        }
    }
}

#[derive(Default)]
struct AppLibinputInterface {
    iface: DefaultLibinputInterface,
    device_fd_map: DeviceFdMap,
}

impl LibinputInterface for AppLibinputInterface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<RawFd, i32> {
        let fd = self.iface.open_restricted(path, flags)?;
        self.device_fd_map.insert(DeviceFd::new(fd, path));
        Ok(fd)
    }

    fn close_restricted(&mut self, fd: RawFd) {
        self.iface.close_restricted(fd);
        self.device_fd_map.remove_by_fd(fd);
    }
}
