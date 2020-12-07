use std::env;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::any::{Any};
use std::fmt::Display;

type FromString<V: Any> = fn(value: String) -> V;

struct Item<V: Any + Display> {
    value: Rc<RefCell<V>>,
    from: FromString<V>,
    desc: String,
    is: bool
}

pub struct Flag<V: Any + Display> {
    help: String,
    keys: HashMap<String, Item<V>>
}

impl<V: Any + Display> Flag<V> {
    pub fn register(&mut self, key: String, default: V
        , from: FromString<V>) -> Rc<RefCell<V>> {
        self.register_with_desc(key, default, from, String::from(""))
    }

    pub fn register_with_desc(&mut self, key: String, default: V
        , from: FromString<V>, desc: String) -> Rc<RefCell<V>> {
        let r = Rc::new(RefCell::new(default));
        self.keys.insert(key.to_string(), Item{
            value: r.clone(),
            from: from,
            desc: desc.to_string(),
            is: false
        });
        r
    }

    pub fn has(&self, key: &str) -> bool {
        let v = match self.keys.get(key) {
            Some(v) => v,
            None => {
                return false;
            }
        };
        v.is
    }

    pub fn parse(&mut self) {
        let args = env::args();
        let args_len = args.len();
        let mut is_find = false;
        let mut last_key = "".to_string();
        for (index, arg) in args.enumerate() {
            if arg == self.help {
                self.printHelp();
                self.exit();
            }
            match self.keys.get(&arg) {
                Some(field) => {
                    is_find = true;
                    last_key = arg;
                    if let Some(r) = self.keys.get_mut(&last_key) {
                        r.is = true;
                    };
                },
                None => {
                    if is_find == true {
                        if let Some(r) = self.keys.get_mut(&last_key) {
                            *(*r.value).borrow_mut() = (r.from)(arg);
                        }
                    }
                    is_find = false;
                }
            }
        }
    }

    fn printHelp(&self) {
        println!("help:");
        for (key, value) in self.keys.iter() {
            println!("\t{}\n\t\tdefault: {}\n\t\tdesc: {}", key, *value.value.borrow(), &value.desc);
        }
    }

    fn exit(&self) {
        if cfg!(target_os="windows") {
            std::process::exit(0);
        } else {
            std::process::exit(0);
        }
    }

    pub fn new() -> Self {
        Self {
            help: "--help".to_string(),
            keys: HashMap::new()
        }
    }
}

