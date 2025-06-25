#[derive(Debug)]
struct User<'a> {
    name: &'a String,
}

pub fn this_main() {
    let first_name = String::from("tofu");
    let user = User { name: &first_name };
    println!("{user:?}");
}
