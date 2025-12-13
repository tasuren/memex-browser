use std::marker::PhantomData;

use uuid::Uuid;

pub use marker::*;

#[derive(PartialEq, Eq)]
pub struct Id<T> {
    phantom: PhantomData<T>,
    value: sqlx::types::Uuid,
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Id<T> {}

impl<T> std::hash::Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T> std::fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> Default for Id<T> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
            value: Uuid::now_v7(),
        }
    }
}

impl<T> std::ops::Deref for Id<T> {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Id<WorkspaceMarker> {
    pub fn home() -> Self {
        Self {
            phantom: PhantomData,
            value: Uuid::default(),
        }
    }
}

mod marker {
    #[derive(PartialEq, Eq)]
    pub struct WorkspaceMarker;

    #[derive(PartialEq, Eq)]
    pub struct TabMarker;
}
