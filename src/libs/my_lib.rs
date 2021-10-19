fn add(i1:u8,i2:u8) -> u8{
    i1+i2
}

#[cfg(test)]
mod test{
    use crate::libs::my_lib::add;

    #[test]
    fn add_test(){
        assert_eq!(add(1,2),3)
    }

}
