pub mod models;

use crate::database::get_user;
pub fn login(_cred: &models::Credentials) {
    //login the user.
    get_user();
}

 
