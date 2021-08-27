use std::env;
use std::rc::Rc;
use std::cell::{RefCell};
use std::collections::{VecDeque, HashMap};
use std::fmt;

pub type Value = Rc<RefCell<Any>>;

// impl fmt::Display for ItemValue {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             v.borrow().fmt(f)
//         }
//     }
// }

struct Item {
    value: Value,
    desc: String,
    is: bool,
    value_len: isize
}

pub struct Flag {
    help: String,
    keys: HashMap<String, Item>,
    is_warning: bool
}

pub struct Value {
    pub v: ItemValue
}

impl Value {
    fn new(v: ItemValue) -> Self {
        Self {
            v: v
        }
    }
}

enum ReadStatus {
    Processing,
    Finish,
    Error(String)
}

struct Reader {
    value: ItemValue,
    value_len: isize,
    index: usize
}

impl Reader {
    fn process(&mut self, arg: String) -> ReadStatus {
        match &mut self.value {
            ItemValue::Single(v) => {
                *v.borrow_mut() = arg;
            },
            ItemValue::Multi(v) => {
                if self.index < v.borrow().len() {
                    *v.borrow_mut()[self.index].borrow_mut() = arg;
                } else {
                    v.borrow_mut().push_back(Rc::new(RefCell::new(arg)));
                }
            }
        }
        self.index += 1;
        if self.value_len < 0 {
            ReadStatus::Processing
        } else {
            if self.index == self.value_len as usize {
                ReadStatus::Finish
            } else if self.index > self.value_len as usize {
                ReadStatus::Error(format!("fixed param, but specity lengthen"))
            } else {
                ReadStatus::Processing
            }
        }
    }

    fn next_key(&self) -> ReadStatus {
        if self.value_len < 0 {
            ReadStatus::Finish
        } else {
            if self.index != self.value_len as usize {
                ReadStatus::Processing
            } else {
                ReadStatus::Finish
            }
        }
    }

    fn new(value: ItemValue, value_len: isize) -> Self {
        Self {
            value: value,
            value_len: value_len,
            index: 0
        }
    }
}

