pub mod models;
pub fn login(cred: models::Credentials) {
    // try login to the user 
    crate::database::get_user();
    // or 
    // super::database::get_user();
}
