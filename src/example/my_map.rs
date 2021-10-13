use std::collections::HashMap;

pub fn init_map() {
    let mut mymap = HashMap::new();
    mymap.insert("color","red");
    mymap.insert("color","yellow");
    println!("{}",mymap.get("color").unwrap());
}