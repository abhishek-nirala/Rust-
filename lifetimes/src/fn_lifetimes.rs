
fn this_main() {
    println!("Start!");

    let ans: &String;
    let str1 = String::from("small");
    {
        let str2 = String::from("longer");
        ans = longest(&str1, &str2);
    }
    println!("{}", ans);

    println!("End!");
}

fn longest<'a>(str1: &'a String, str2: &'a String) -> &'a String {
    if str1.len() > str2.len() { str1 } else { str2 }
}
