use core::panic;

pub fn give_princess(gift :Option<&str>) -> Option<&str>{
    let inside = gift?;
    println!("{}",inside);
    if inside == "snake" 
    {
        panic!("aaaaa A")
    }
    println!("i love the gift");
    Some(inside)
}

pub fn give_commoner(gift :Option<&str>){
    match gift {
        Some(gift) => println!("i am happy!,{}",gift),
        None => println!("anywhere!"),
    }
}

pub fn my_unwrap_test(){
    let gift:Option<&str> = Some("snxake");
    give_commoner(gift);
    let rec_gift = give_princess(gift).unwrap();
    println!("{}",rec_gift);
    let gift_none = None;
    // give_princess(gift_none);
    give_commoner(gift_none);
}