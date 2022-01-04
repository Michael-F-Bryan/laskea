use std::{
    borrow::Borrow,
    fmt::{self, Display, Formatter},
    ops::Deref,
    sync::Arc,
};

/// A reference-counted string.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Text(Arc<str>);

impl Text {
    fn new(s: impl Into<Arc<str>>) -> Self {
        Text(s.into())
    }
}

impl Deref for Text {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for Text {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Default for Text {
    fn default() -> Self {
        Text::new("")
    }
}

impl<'a> From<&'a str> for Text {
    fn from(s: &'a str) -> Self {
        Text::new(s)
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        Text::new(s)
    }
}

impl<T> Borrow<T> for Text
where
    Arc<str>: Borrow<T>,
    T: ?Sized,
{
    fn borrow(&self) -> &T {
        self.0.borrow()
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}
