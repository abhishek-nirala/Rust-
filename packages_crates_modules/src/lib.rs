#[derive(Debug)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

enum DatabaseStatus {
    Connected,
    Interrupted,
}

fn connect_to_database() -> DatabaseStatus {
    //logic for database connection.
    DatabaseStatus::Connected
}

fn get_user(){
    //fetch the details of the user from the database.
}

fn login(cred: &Credentials){
    //login the user.
    get_user();
}

pub fn authenticate(cred: &Credentials) {
     if let DatabaseStatus :: Connected = connect_to_database() {
         login(&cred);
     }
    println!("Start!");}
