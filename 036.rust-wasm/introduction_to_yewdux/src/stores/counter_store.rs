use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Store)]
pub struct CounterStore {
    pub count: u32,
}
