mod my_var;
mod example;
use example::example::User;

use my_var::Books;


fn main() {
    let u  = User::new(String::from("test"),1);
    println!("{:#?}",u);
    let b = Books{ id: 1, content: "a".to_string()};
    println!("{:#?}",b.new());
}
