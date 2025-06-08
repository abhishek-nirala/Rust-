
pub enum DatabaseStatus {
    Connected,
    _Interrupted,
}
pub fn connect_to_database() -> DatabaseStatus {
    //logic for database connection.
    DatabaseStatus::Connected
}
pub fn get_user() {
    //fetch the details of the user from the database.
}
