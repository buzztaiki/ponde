#[derive(Debug, PartialEq, Eq)]
pub struct DeviceInfo {
    pub name: String,
    pub pointer: bool,
    pub gesture: bool,
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
