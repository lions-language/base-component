#[macro_use]
extern crate command_option;
use command_option::flag::{Flag, ItemValue};

fn main() {
    let mut flag = Flag::new();
    let host = flag.reg_string(String::from("-h"), String::from("localhost")
	, String::from("host"));
    let port = flag.reg_u32(String::from("-p"), 80
	, String::from("port"));
    let address = flag.reg_lengthen_str_vec(String::from("-address")
        , vecdeque!["a".to_string(), "b".to_string(), "c".to_string()]
        , String::from("address"));
    let packages = flag.reg_fixed_str_vec(String::from("-packages")
        , vecdeque!["libmath".to_string(), "../third".to_string()]
        , String::from("packages"));
    flag.parse();
    println!("h: {}", read_string!(host));
    println!("p: {}", read_i32!(port));
    for item in read_vector!(address) {
        println!("address: {}", read_string_item!(item));
    }
    for item in read_vector!(packages) {
        println!("packages: {}", read_string_item!(item));
    }
}
