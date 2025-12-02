use std::os::unix::prelude::*;
use std::path::Path;

use input::LibinputInterface;
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;

#[derive(Default)]
pub struct DefaultLibinputInterface {}

impl LibinputInterface for DefaultLibinputInterface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        open(path, OFlag::from_bits_truncate(flags), Mode::empty()).map_err(|e| e as i32)
    }

    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(fd);
    }
}
