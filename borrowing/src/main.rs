fn main() {
    println!("Start!");
    
    let mut s = String::from("Tofu");
    let len = str_length(&mut s);
    println!("size of {:?} is {len}",s);

    println!("Finish!")
}


fn str_length(s:&mut String) -> usize {
    s.push_str(" soya sauce!");
    s.len()
}
 