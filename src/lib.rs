use std::borrow::Borrow;

#[cfg(feature = "serde")]
use serde::{de, Deserialize, Serialize};

#[derive(Debug )]
#[cfg_attr(feature = "serde", derive(Serialize), serde(transparent))]
#[repr(transparent)]
pub struct LimitedStr<const MAX_LENGTH: usize>(str);

impl<const MAX_LENGTH: usize> Borrow<LimitedStr<MAX_LENGTH>> for LimitedString<MAX_LENGTH> {
    fn borrow(&self) -> &LimitedStr<MAX_LENGTH> {
        unsafe { LimitedStr::<MAX_LENGTH>::from_str_unchecked(self.0.borrow()) }
    }
}

impl<const MAX_LENGTH: usize> LimitedStr<MAX_LENGTH> {
    pub const fn from_str(s: &str) -> Option<&Self> {
        if s.len() <= MAX_LENGTH {
            Some(unsafe { Self::from_str_unchecked(s) })
        } else {
            None
        }
    }

    pub const unsafe fn from_str_unchecked(s: &str) -> &Self {
        union StrRepr<'a, const MAX_LENGTH: usize> {
            normal_str: &'a str,
            limited_str: &'a LimitedStr<MAX_LENGTH>,
        }
        unsafe { StrRepr::<MAX_LENGTH> { normal_str: s }.limited_str }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(transparent))]
#[repr(transparent)]
pub struct LimitedString<const MAX_LENGTH: usize>(String);

impl<const MAX_LENGTH: usize> ToOwned for LimitedStr<MAX_LENGTH> {
    type Owned = LimitedString<MAX_LENGTH>;

    fn to_owned(&self) -> Self::Owned {
        LimitedString(self.0.to_owned())
    }
}

impl<const MAX_LENGTH: usize> LimitedString<MAX_LENGTH> {
    pub fn from_string(s: String) -> Result<Self, String> {
        if s.len() <= MAX_LENGTH {
            Ok(Self(s))
        } else {
            Err(s)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, const MAX_LENGTH: usize> Deserialize<'de> for LimitedString<MAX_LENGTH> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        <String as Deserialize>::deserialize(deserializer).and_then(|s| {
            Self::from_string(s).map_err(|s| {
                de::Error::invalid_length(
                    s.len(),
                    &"a string that has a length less than max length",
                )
            })
        })
    }
}
