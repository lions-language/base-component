use std::env;
use std::any::Any;
use std::rc::Rc;
use std::cell::{RefCell};
use std::collections::{VecDeque, HashMap};
use std::fmt;

pub type Value = Rc<RefCell<Box<Any>>>;

pub enum ValueReaderStatus {
    Pending,
    Ready
}

pub trait ValueReader {
    fn next(&self) -> ValueReaderStatus;
    fn to_any(self) -> Box<Any>;
}

struct Item {
    desc: String,
    value: Value
}

pub struct Command {
    keys: HashMap<String, Item>
}

impl Command {
    pub fn register<T: ValueReader>(
        &mut self, key: &str, default: T, desc: &str) -> Value {

        let value = Rc::new(RefCell::new(default.to_any()));

        self.keys.insert(key.to_string(), Item{
            desc: desc.to_string(),
            value: value.clone()
        });

        value
    }

    pub fn new() -> Self {
        Self {
            keys: HashMap::new()
        }
    }
}

