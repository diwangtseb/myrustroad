pub(crate) fn print_var() {
    println!("enmmm?")
}

#[derive(Debug)]
pub struct User{
    name:String,
    id:i8
}

impl User {
    pub fn new(name:String,id:i8)->User {
        User{
            name:name,
            id:id
        }
    }
}
