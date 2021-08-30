use std::env;
use std::any::Any;
use std::rc::Rc;
use std::cell::{RefCell};
use std::collections::{VecDeque, HashMap};
use std::fmt;

pub use value_reader::*;

pub type Value = Rc<RefCell<Box<Any>>>;

pub struct StringValue {
    value: Value
}

pub struct NumericalValue {
    value: Value
}

///////////////////////
pub enum ValueReaderStatus {
    Pending,
    Ready
}

pub struct ValueReader {
    pub create: fn() -> Box<Any>,
    pub next: fn(&mut Box<Any>, &str) -> ValueReaderStatus,
    pub result: fn(Box<Any>) -> Box<Any>
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

    pub fn register_string(
        &mut self, key: &str, default: &str, desc: &str) -> StringValue {
        let value = self.register_with_reader(key, Box::new(default.to_string()), desc, ValueReader{
            create: StringValueReader::create,
            next: StringValueReader::next,
            result: StringValueReader::result
        });

        StringValue{
            value: value
        }
    }

    pub fn parse(&mut self) {
        let args: Vec<String> = env::args().collect();

        let mut index = 0;
        loop {
            if index == args.len() - 1 {
                break;
            }

            if &args[index] == &self.help_key {
                self.print_help();
                self.exit();
            }

            let item = match self.keys.get(&args[index]) {
                Some(item) => {
                    index += 1;
                    item
                },
                None => {
                    println!("{} unregister", &args[index]);
                    self.exit();
                    break;
                }
            };

            let mut object = (item.reader.create)();
            loop {
                match (item.reader.next)(&mut object, &args[index]) {
                    ValueReaderStatus::Pending => {
                        index += 1;
                    },
                    ValueReaderStatus::Ready => {
                        index += 1;
                        *item.value.borrow_mut() = (item.reader.result)(object);
                        break;
                    }
                }
            }
        }
    }

    fn print_help(&self) {
        println!("help:");
        for (key, value) in self.keys.iter() {
            println!("\t{}\n\t\tdefault: {:?}\n\t\tdesc: {}", key, value.value, &value.desc);
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

pub mod value_reader;

