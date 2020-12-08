#[macro_use]
extern crate command_option;
use command_option::flag::{Flag};

fn main() {
    let mut flag = Flag::new();
    let host = flag.reg_string(String::from("-h"), String::from("localhost")
	, String::from("host"));
    let port = flag.reg_u32(String::from("-p"), 80
	, String::from("port"));
    flag.parse();
    println!("{}", read_string!(host));
    println!("{}", read_i32!(port));
}
