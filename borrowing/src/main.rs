
/* RUST BORROWING RULES
 * 
 * Only one mutable reference is allowed of a single variable.
 * Any number of immutable reference is allowed.
 * Mutable and immutable reference cannot be allowed simultaneously.
 * Reference is introduced and remains untill it's used, and therefore after value is used another
 * refrence can be taken either mutable or immutable.
 * */


fn main() {
    println!("Start!");

    let mut s = String::from("Tofu");
    //let len = str_length(&mut s);
    let r1 = &s;
    let rs = &r1;
    //rs.push_str("-soya sauce"); 

    println!("rs: {rs}");
    println!("printing r1: {r1}");
    println!("Finish!");
    let r2 = &mut s;
    r2.push_str(" with wasabi");
    println!("r2: {r2}");


    println!("s: {s}");
}

//fn str_length(s:&mut String) -> usize {
//    s.push_str(" soya sauce!");
//    s.len()
//}

