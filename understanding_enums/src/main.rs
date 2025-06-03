#[derive(Debug)]
enum IPAddrKind {
    IPv4(u8, u8, u8, u8), //IPv4(String),
    IPv6(String),
}
//struct IPAddr {
//    kind : IPAddrKind,
//    data: String
//}


enum Role {
    User(String),
    Admin(String)
}
//enums can also have impl's.
impl Role {
    fn auth (&self){
        //
    }
}

fn main() {
    println!("Start!");

    //let home = IPAddr {
    //    kind : IPAddrKind::IPv6,
    //    data : String::from("::1")
    //};
    //shortcut for acheiving the above home;
    let home = IPAddrKind::IPv6(String::from("::1"));
    let loopback = IPAddrKind::IPv4(127, 0, 0, 1);

    println!("home : {:#?}", loopback);


    let char_enum = Some('e');
    let num_enum:Option<u32> = Some(45);
    println!("num_enum : {:#?}",num_enum);

    println!("End!");
}

//fn user_type(user_kind:Role){
//    if Role::User {
//        println!("you're not allowed to do changes");
//    } else {
//        println!("you are allowed to do changes");
//    }
//    //println!()
//}
