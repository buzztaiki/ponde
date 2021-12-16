use std::os::unix::prelude::RawFd;
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub struct DeviceFd {
    fd: RawFd,
    path: Box<Path>,
}

impl DeviceFd {
    pub fn new(fd: RawFd, path: &Path) -> Self {
        Self {
            fd,
            path: path.into(),
        }
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
            if value.fd == device_fd.fd || value.path == device_fd.path {
                self.values.remove(i);
            } else {
                i += 1;
            }
        }
        self.values.push(device_fd);
    }

    pub fn get_by_path(&self, path: &Path) -> Option<&DeviceFd> {
        self.values.iter().find(|x| *x.path == *path)
    }

    pub fn remove_by_fd(&mut self, fd: RawFd) -> Option<DeviceFd> {
        for i in 0..self.values.len() {
            if self.values[i].fd == fd {
                return Some(self.values.remove(i));
            }
        }
        None
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.values.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut map = DeviceFdMap::default();
        map.insert(DeviceFd::new(1, Path::new("p1")));
        map.insert(DeviceFd::new(2, Path::new("p2")));
        assert_eq!(map.len(), 2);
        assert_eq!(
            map.get_by_path(Path::new("p1")),
            Some(&DeviceFd::new(1, Path::new("p1")))
        );

        map.insert(DeviceFd::new(3, Path::new("p1")));
        assert_eq!(map.len(), 2);
        assert_eq!(
            map.get_by_path(Path::new("p1")),
            Some(&DeviceFd::new(3, Path::new("p1")))
        );

        map.insert(DeviceFd::new(3, Path::new("p3")));
        assert_eq!(map.len(), 2);
        assert_eq!(
            map.get_by_path(Path::new("p3")),
            Some(&DeviceFd::new(3, Path::new("p3")))
        );

        map.insert(DeviceFd::new(3, Path::new("p2")));
        assert_eq!(map.len(), 1);
        assert_eq!(
            map.get_by_path(Path::new("p2")),
            Some(&DeviceFd::new(3, Path::new("p2")))
        );
    }

    #[test]
    fn test_get_by_path() {
        let mut map = DeviceFdMap::default();
        map.insert(DeviceFd::new(1, Path::new("p1")));
        assert_eq!(map.len(), 1);
        assert_eq!(
            map.get_by_path(Path::new("p1")),
            Some(&DeviceFd::new(1, Path::new("p1")))
        );
        assert_eq!(map.get_by_path(Path::new("p2")), None);
    }

    #[test]
    fn test_remove_by_fd() {
        let mut map = DeviceFdMap::default();
        map.insert(DeviceFd::new(1, Path::new("p1")));
        map.insert(DeviceFd::new(2, Path::new("p2")));
        assert_eq!(map.remove_by_fd(1), Some(DeviceFd::new(1, Path::new("p1"))));
        assert_eq!(map.remove_by_fd(1), None);
        assert_eq!(map.len(), 1);
    }
}
