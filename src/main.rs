mod my_var;
mod example;
use example::example::User;


fn main() {
    let u  = User::new(String::from("test"),1);
    println!("{:#?}",u);
}
