pub fn divide(x:u8, y:u8) -> Result<u8, String>{
    println!("----------------------------in divide-----------------");
    if y == 0 {
        Err(String::from("Please enter a number above 0"))
    } else {
        Ok(x / y)
    }
}