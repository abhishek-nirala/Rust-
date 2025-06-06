use packages_crates_modules::{authenticate, Credentials};



fn main() {
    println!("Start!");
    
    let cred = Credentials {
        username :  String::from("tofu"),
        password : String::from("tofu@random123456")
    };
    
    authenticate(&cred);

    println!("Credentials : {:#?}",cred);


    println!("End!");
}
