use std::io;
use std::os::unix::prelude::RawFd;
use std::path::Path;

use nix::ioctl_write_int;

use crate::errors::Error;

// see /usr/include/linux/input.h
ioctl_write_int!(eviocgrab, b'E', 0x90);

#[derive(Debug, PartialEq, Eq)]
pub struct DeviceFd {
    fd: RawFd,
    path: Box<Path>,
    name: String,
}

impl DeviceFd {
    pub fn new(fd: RawFd, path: &Path) -> Option<Self> {
        path.file_name().and_then(|x| x.to_str()).map(|x| Self {
            fd,
            path: path.into(),
            name: x.to_string(),
        })
    }

    pub fn grab(&mut self) -> Result<(), Error> {
        unsafe { eviocgrab(self.fd, 1) }.map_err(io::Error::from)?;
        Ok(())
    }
}

#[derive(Default)]
pub struct DeviceFdMap {
    values: Vec<DeviceFd>,
}

impl DeviceFdMap {
    pub fn insert(&mut self, device_fd: DeviceFd) {
        let mut i = 0;
        while i < self.values.len() {
            let value = &self.values[i];
            if value.fd == device_fd.fd || value.name == device_fd.name {
                self.values.remove(i);
            } else {
                i += 1;
            }
        }
        self.values.push(device_fd);
    }

    #[allow(dead_code)]
    pub fn get_by_name(&self, name: &str) -> Option<&DeviceFd> {
        self.values.iter().find(|x| x.name == name)
    }

    pub fn get_by_name_mut(&mut self, name: &str) -> Option<&mut DeviceFd> {
        self.values.iter_mut().find(|x| x.name == name)
    }

    pub fn remove_by_fd(&mut self, fd: RawFd) -> Option<DeviceFd> {
        for i in 0..self.values.len() {
            if self.values[i].fd == fd {
                return Some(self.values.remove(i));
            }
        }
        None
    }

    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.values.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_device_fd(fd: RawFd, path: &Path) -> DeviceFd {
        DeviceFd::new(fd, path).unwrap()
    }

    #[test]
    fn test_insert() {
        let mut map = DeviceFdMap::default();
        map.insert(new_device_fd(1, Path::new("/dev/f1")));
        map.insert(new_device_fd(2, Path::new("/dev/f2")));
        assert_eq!(map.len(), 2);
        assert_eq!(
            map.get_by_name("f1"),
            Some(&new_device_fd(1, Path::new("/dev/f1")))
        );

        // name should be a key
        map.insert(new_device_fd(3, Path::new("/dev/f1")));
        assert_eq!(map.len(), 2);
        assert_eq!(
            map.get_by_name("f1"),
            Some(&new_device_fd(3, Path::new("/dev/f1")))
        );

        // fd should also be a key
        map.insert(new_device_fd(3, Path::new("/dev/f3")));
        assert_eq!(map.len(), 2);
        assert_eq!(
            map.get_by_name("f3"),
            Some(&new_device_fd(3, Path::new("/dev/f3")))
        );

        // fd and path matches different entries, should remove both entries
        map.insert(new_device_fd(3, Path::new("/dev/f2")));
        assert_eq!(map.len(), 1);
        assert_eq!(
            map.get_by_name("f2"),
            Some(&new_device_fd(3, Path::new("/dev/f2")))
        );
    }

    #[test]
    fn test_get_by_name() {
        let mut map = DeviceFdMap::default();
        map.insert(new_device_fd(1, Path::new("f1/f1")));
        assert_eq!(map.len(), 1);
        assert_eq!(
            map.get_by_name("f1"),
            Some(&new_device_fd(1, Path::new("f1/f1")))
        );
        assert_eq!(map.get_by_name("f2"), None);
    }

    #[test]
    fn test_get_by_name_mut() {
        let mut map = DeviceFdMap::default();
        map.insert(new_device_fd(1, Path::new("/dev/f1")));
        assert_eq!(map.len(), 1);
        assert_eq!(
            map.get_by_name_mut("f1"),
            Some(&mut new_device_fd(1, Path::new("/dev/f1")))
        );
    }

    #[test]
    fn test_remove_by_fd() {
        let mut map = DeviceFdMap::default();
        map.insert(new_device_fd(1, Path::new("/dev/f1")));
        map.insert(new_device_fd(2, Path::new("/dev/f2")));
        assert_eq!(
            map.remove_by_fd(1),
            Some(new_device_fd(1, Path::new("/dev/f1")))
        );
        assert_eq!(map.remove_by_fd(1), None);
        assert_eq!(map.len(), 1);
    }
}
