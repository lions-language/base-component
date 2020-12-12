use std::env;
use std::rc::Rc;
use std::cell::{RefCell};
use std::collections::HashMap;
use std::fmt;

enum Opt {
    Single(String),
    Multi(Vec<String>)
}

impl fmt::Display for Opt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Opt::Single(v) => {
                v.fmt(f)
            },
            Opt::Multi(v) => {
                let mut r = String::new();
                for item in v {
                    r.push_str(item);
                    r.push(' ');
                }
                write!(f, "{}", r)
            }
        }
    }
}

type RcValue = Rc<RefCell<Opt>>;

struct Item {
    value: RcValue,
    desc: String,
    is: bool,
}

pub struct Flag {
    help: String,
    keys: HashMap<String, Item>,
    is_warning: bool
}

pub struct Value {
    v: RcValue
}

impl Value {
    fn new(v: RcValue) -> Self {
        Self {
            v: v
        }
    }
}

enum ReadStatus {
    Processing,
    Finish
}

struct Reader {
    value: RcValue
}

impl Reader {
    fn process(&mut self, arg: String) -> ReadStatus {
        *self.value.borrow_mut() = Opt::Single(arg);
        ReadStatus::Finish
    }

    fn new(value: RcValue) -> Self {
        Self {
            value: value
        }
    }
}

impl Flag {
    pub fn register(&mut self, key: String, default: Opt) -> Value {
        self.register_with_desc(key, default, String::from(""))
    }

    pub fn register_with_desc(&mut self, key: String, default: Opt
        , desc: String) -> Value {
        let r = Rc::new(RefCell::new(default));
        self.keys.insert(key.to_string(), Item{
            value: r.clone(),
            desc: desc,
            is: false
        });
        Value::new(r)
    }

    pub fn reg_string(&mut self, key: String, default: String, desc: String) -> Value {
        self.register_with_desc(key, Opt::Single(default), desc)
    }

    pub fn reg_u32(&mut self, key: String, default: u32, desc: String) -> Value {
        self.register_with_desc(key, Opt::Single(default.to_string()), desc)
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
        let mut reader: Option<Reader> = None;
        let mut read_status = ReadStatus::Processing;
        for (i, arg) in args.enumerate() {
            if arg == self.help {
                self.print_help();
                self.exit();
            }
            match &mut reader {
                Some(r) => {
                    read_status = r.process(arg);
                    if let ReadStatus::Finish = &read_status {
                        reader = None;
                    }
                },
                None => {
                    match self.keys.get(&arg) {
                        Some(item) => {
                            if let ReadStatus::Processing = &read_status {
                                self.panic(format!(
                                        "the parameters before the {} parameter are not matched"
                                        , arg));
                            }
                            reader = Some(Reader::new(item.value.clone()));
                            continue;
                        },
                        None => {
                        }
                    }
                }
            }
        }
    }

    fn panic<T: std::fmt::Display>(&self, msg: T) {
        println!("{}", msg);
        std::process::exit(0);
    }

    fn warning<T: std::fmt::Display>(&self, msg: T) {
        if self.is_warning {
            println!("[Warning] {}", msg);
        }
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

    pub fn set_help(&mut self, help: String) {
        *&mut self.help = help;
    }

    pub fn set_warning(&mut self) {
        self.is_warning = true;
    }

    pub fn set_nowarning(&mut self) {
        self.is_warning = false;
    }

    pub fn new() -> Self {
        Self {
            help: "--help".to_string(),
            keys: HashMap::new(),
            is_warning: true
        }
    }
}

#[macro_export]
macro_rules! read {
    ($v:ident, $typ:ident) => {
        match $v.v.borrow().parse::<$typ>() {
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
        &*$v.v.borrow()
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

