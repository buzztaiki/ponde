use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AccelProfile {
    Adaptive,
    Flat,
}

impl From<AccelProfile> for input::AccelProfile {
    fn from(x: AccelProfile) -> Self {
        match x {
            AccelProfile::Adaptive => input::AccelProfile::Adaptive,
            AccelProfile::Flat => input::AccelProfile::Flat,
        }
    }
}
