use std::str::FromStr;

use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Button(evdev::Key);

impl<'de> Deserialize<'de> for Button {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        evdev::Key::from_str(&s)
            .ok()
            .filter(|_| s.starts_with("BTN_"))
            .map(Self)
            .ok_or_else(|| serde::de::Error::custom(format!("unexpected button value {}", s)))
    }
}

impl Button {
    pub fn code(&self) -> u16 {
        self.0.code()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_de_tokens, assert_de_tokens_error, Token};

    #[test]
    fn test_de_button() {
        assert_de_tokens(&Button(evdev::Key::BTN_LEFT), &[Token::Str("BTN_LEFT")]);
    }

    #[test]
    fn test_de_key() {
        assert_de_tokens_error::<Button>(&[Token::Str("KEY_A")], "unexpected button value KEY_A");
    }

    #[test]
    fn test_de_invalid_button() {
        assert_de_tokens_error::<Button>(
            &[Token::Str("BTN_BAD")],
            "unexpected button value BTN_BAD",
        );
    }
}
