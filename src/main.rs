mod example;
use example::my_struct::User;
use example::my_var::Books;

use crate::example::{my_lifecycle, my_map, my_unwrap::{my_unwrap_test}, my_vec};


fn main() {
    let u  = User::new(String::from("test"),1);
    println!("{:#?}",u);
    let b = Books{ id: 1, content: "a".to_string()};
    println!("{:#?}",b.new());
    // let temp_str_a = "a北京";
    // let temp_str_b = "b上海";
    // let res  = my_lifecycle::tryreturn(temp_str_a,temp_str_b);
    // println!("{}",res);
    my_lifecycle::look_life();
    my_vec::init_vec();
    my_map::init_map();
    my_unwrap_test();
    closure();
}

fn closure(){
    let _  = (|num| println!("closure,{}",num))("test");
    let _ = || println!("ok");
}
