use packages_crates_modules::authenticate;
use packages_crates_modules::auth_utils::models::Credentials;

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
