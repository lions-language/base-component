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

pub struct ValueReader {
    pub next: fn(&str) -> ValueReaderStatus
}

struct Item {
    desc: String,
    value: Value,
    reader: ValueReader
}

pub struct Command {
    help_key: String,
    keys: HashMap<String, Item>
}

impl Command {
    pub fn register_with_reader(
        &mut self, key: &str, default: Box<Any>, desc: &str, reader: ValueReader) -> Value {

        let value = Rc::new(RefCell::new(default));

        self.keys.insert(key.to_string(), Item{
            desc: desc.to_string(),
            value: value.clone(),
            reader: reader
        });

        value
    }

    pub fn parse(&mut self) {
        let args = env::args();

        for (_, arg) in args.enumerate() {
            if arg == self.help_key {
                self.print_help();
                self.exit();
            }

            let item = match self.keys.get(&arg) {
                Some(item) => item,
                None => {
                    continue;
                }
            };

            loop {
                match (item.reader.next)(&arg) {
                    ValueReaderStatus::Pending => {
                    },
                    ValueReaderStatus::Ready => {
                        break;
                    }
                }
            }
        }
    }

    fn print_help(&self) {
        println!("help:");
        for (key, value) in self.keys.iter() {
            // println!("\t{}\n\t\tdefault: {}\n\t\tdesc: {}", key, value.value, &value.desc);
        }
    }

    fn exit(&self) {
        std::process::exit(0);
    }

    pub fn set_help_key(&mut self, key: String) {
        *&mut self.help_key = key;
    }

    pub fn new() -> Self {
        Self {
            help_key: String::new(),
            keys: HashMap::new()
        }
    }
}

