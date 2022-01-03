use std::{
    ops::{Deref, Index},
    sync::Arc,
};

/// A reference-counted string.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Sequence<T>(Arc<[T]>);

impl<T> Sequence<T> {
    pub fn empty() -> Self {
        Sequence::new(Vec::new())
    }

    pub fn new(s: impl Into<Arc<[T]>>) -> Self {
        Sequence(s.into())
    }
}

impl<T> Deref for Sequence<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Default for Sequence<T> {
    fn default() -> Self {
        Sequence::empty()
    }
}

impl<T, V> From<V> for Sequence<T>
where
    Arc<[T]>: From<V>,
{
    fn from(v: V) -> Self {
        Sequence::new(v)
    }
}

impl<I, T> Index<I> for Sequence<T>
where
    Arc<[T]>: Index<I>,
{
    type Output = <Arc<[T]> as Index<I>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.0.index(index)
    }
}

impl<T, V> PartialEq<V> for Sequence<T>
where
    [T]: PartialEq<V>,
{
    fn eq(&self, other: &V) -> bool {
        self.0.eq(other)
    }
}
