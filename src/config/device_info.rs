#[derive(Debug, PartialEq, Eq)]
pub struct DeviceInfo {
    pub name: String,
    pub pointer: bool,
    pub gesture: bool,
}

impl DeviceInfo {
    #[allow(dead_code)]
    pub fn of_mouse(name: &str) -> Self {
        Self {
            name: name.to_string(),
            pointer: true,
            gesture: false,
        }
    }

    pub fn is_mouse(&self) -> bool {
        self.pointer && !self.gesture
    }
}

impl From<&input::Device> for DeviceInfo {
    fn from(x: &input::Device) -> Self {
        DeviceInfo {
            name: x.name().to_string(),
            pointer: x.has_capability(input::DeviceCapability::Pointer),
            gesture: x.has_capability(input::DeviceCapability::Gesture),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mouse() {
        let device_info = DeviceInfo {
            name: "moo".to_string(),
            pointer: true,
            gesture: false,
        };
        assert!(device_info.is_mouse());
    }

    #[test]
    fn test_non_pointer_device() {
        let device_info = DeviceInfo {
            name: "moo".to_string(),
            pointer: false,
            gesture: false,
        };
        assert!(!device_info.is_mouse())
    }

    #[test]
    fn test_touchpad() {
        let device_info = DeviceInfo {
            name: "moo".to_string(),
            pointer: true,
            gesture: true,
        };
        assert!(!device_info.is_mouse())
    }
}
