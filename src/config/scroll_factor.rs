use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(try_from = "f64")]
pub struct ScrollFactor(f64);

impl TryFrom<f64> for ScrollFactor {
    type Error = TryFromFloatError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value > 0.0 && value.is_finite() {
            Ok(ScrollFactor(value))
        } else {
            Err(TryFromFloatError())
        }
    }
}

impl ScrollFactor {
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Default for ScrollFactor {
    fn default() -> Self {
        1.0.try_into().expect("should not be error")
    }
}

#[derive(thiserror::Error, Debug, PartialEq)]
#[error("scroll factor must be a positive finite number")]
pub struct TryFromFloatError();

#[derive(Debug, Deserialize, Default, PartialEq)]
/// Scroll speed factor vertical/horizontal pair.
pub struct ScrollFactorPair {
    /// vertical scroll speed factor.
    #[serde(default)]
    pub vertical: ScrollFactor,
    /// horizontal scroll speed factor.
    #[serde(default)]
    pub horizontal: ScrollFactor,
}

#[cfg(test)]
mod tests {
    use core::f64;

    use super::*;
    use serde_test::{Token, assert_de_tokens, assert_de_tokens_error};

    #[test]
    fn test_try_from_float() {
        let err = Err(TryFromFloatError());
        assert_eq!(ScrollFactor::try_from(1.0), Ok(ScrollFactor(1.0)));
        assert_eq!(ScrollFactor::try_from(0.0), err);
        assert_eq!(ScrollFactor::try_from(-1.0), err);
        assert_eq!(ScrollFactor::try_from(f64::NAN), err);
        assert_eq!(ScrollFactor::try_from(f64::INFINITY), err);
        assert_eq!(ScrollFactor::try_from(f64::NEG_INFINITY), err);
    }

    #[test]
    fn test_de() {
        let error = "scroll factor must be a positive finite number";
        assert_de_tokens(&ScrollFactor(1.0), &[Token::F64(1.0)]);
        assert_de_tokens_error::<ScrollFactor>(&[Token::F64(0.0)], error);
        assert_de_tokens_error::<ScrollFactor>(&[Token::F64(-1.0)], error);
        assert_de_tokens_error::<ScrollFactor>(&[Token::F64(f64::NAN)], error);
        assert_de_tokens_error::<ScrollFactor>(&[Token::F64(f64::INFINITY)], error);
        assert_de_tokens_error::<ScrollFactor>(&[Token::F64(f64::NEG_INFINITY)], error);
    }
}
