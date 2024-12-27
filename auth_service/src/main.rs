use auth_service::authenticate;
use auth_service::auth_utils::models::Credentials;
fn main() {
    let Cred = Credentials {
        username: String::from("Punit Savlesha"),
        password: String::from("punpun123")
    };

    authenticate(Cred);
    //or
    //auth_service::authenticate(cred);
}
