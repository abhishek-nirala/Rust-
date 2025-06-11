pub fn strings(){
    println!("in string");

    let mut a = String::new();
    a.push_str("Здравствуйте");
    println!("a : {}",a.len());

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("s : {s}");
    
    let hello = "नमस्ते";
    // let ans = &hello[1..4];
    // println!("ans : {}",ans);
    // for i in hello.bytes() {     //applies Copy trait at `hello`. hence value is copied;
    //     println!("{i}");
    // }
 for i in hello.chars() {     //chars() returns letters & bytes returns unicode scalar values.
        println!("{i}");
    }
    // println!("{hello}");

}