use std::env;
use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;

struct Item {
    value: Rc<RefCell<String>>,
    desc: String,
    is: bool
}

pub struct Flag {
    help: String,
    keys: HashMap<String, Item>
}

impl Flag {
    pub fn register(&mut self, key: String, default: String) -> Rc<RefCell<String>> {
        self.register_with_desc(key, default, String::from(""))
    }

    pub fn register_with_desc(&mut self, key: String, default: String
        , desc: String) -> Rc<RefCell<String>> {
        let r = Rc::new(RefCell::new(default));
        self.keys.insert(key.to_string(), Item{
            value: r.clone(),
            desc: desc,
            is: false
        });
        r
    }

    pub fn reg_string(&mut self, key: String, default: String, desc: String) -> Rc<RefCell<String>> {
        self.register_with_desc(key, default, desc)
    }

    pub fn reg_u32(&mut self, key: String, default: u32, desc: String) -> Rc<RefCell<String>> {
        self.register_with_desc(key, default.to_string(), desc)
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

    pub fn parse2(&mut self) {
        let args = env::args();
        let args_len = args.len();
        let mut is_find = false;
        let mut last_key = "".to_string();
        for (index, arg) in args.enumerate() {
            if arg == self.help {
                self.print_help();
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
                            *r.value.borrow_mut() = arg;
                        }
                    }
                    is_find = false;
                }
            }
        }
    }

    pub fn parse(&mut self) {
        let args = env::args();
        let mut index = 0;
        let mut key_queue = Vec::with_capacity(1);
        for (i, arg) in args.enumerate() {
            if i != index {
                /*
                if key_queue.is_empty() {
                    self.panic(format!("value: {}, cannot be bound to any parameter", arg));
                }
                */
                if key_queue.is_empty() {
                    continue;
                }
                let key = key_queue.remove(0);
                *self.keys.get_mut(&key).unwrap().value.borrow_mut() = arg;
                continue;
            }
            if arg == self.help {
                self.print_help();
                self.exit();
            }
            match self.keys.get(&arg) {
                Some(item) => {
                    index += 1 + 1;
                    key_queue.push(arg);
                    continue;
                },
                None => {
                }
            }
        }
    }

    fn panic<T: std::fmt::Display>(&self, msg: T) {
        println!("{}", msg);
        std::process::exit(0);
    }

    fn print_help(&self) {
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

#[macro_export]
macro_rules! read {
    ($v:ident, $typ:ident) => {
        match $v.borrow().parse::<$typ>() {
            Ok(v) => v,
            Err(_) => {
                println!("[ERROR] file: {}, line: {}, var \"{}\": to {} error"
                    , file!(), line!(), stringify!($v), stringify!($typ));
                std::process::exit(0);
            }
        }
    }
}

#[macro_export]
macro_rules! read_i32 {
    ($v:ident) => {
        read!($v, i32)
    }
}

#[macro_export]
macro_rules! read_u32 {
    ($v:ident) => {
        read!($v, u32)
    }
}

#[macro_export]
macro_rules! read_string {
    ($v:ident) => {
        &*$v.borrow()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn flag_test() {
        let mut flag = Flag::new();
        let host = flag.reg_string(String::from("-h"), String::from("localhost")
            , String::from("host"));
        let port = flag.reg_u32(String::from("-p"), 80
            , String::from("port"));
        flag.parse();
        println!("{}", read_string!(host));
        println!("{}", read_i32!(port));
    }
}

