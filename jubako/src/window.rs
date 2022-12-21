//! Window hosted by server and related types.

use std::fmt::Debug;

pub mod simple_window;

pub trait Message: 'static + Debug + Clone + Send + Sync {}
impl<T: 'static + Debug + Clone + Send + Sync> Message for T {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirtyFlag {
    ShouldRender,
    None,
}
