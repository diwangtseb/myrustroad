pub fn longer<'a>(a:&'a str,b:&'a str) -> &'a str {
    if a.len()>b.len(){
        a
    }
    else {
        b
    }
}

pub fn look_life(){
    let r:&str;
    {
        let a = "北京";
        let b = "上海";
        r = longer(a, b)
    }
    println!("{}",r)    
}