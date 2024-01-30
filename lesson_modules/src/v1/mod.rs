mod database;
pub mod auth_utils;

pub fn authenticate(credentials: auth_utils::models::Credentials){
    if let database::Status::Connected = database::connect_to_database() {
        auth_utils::login(credentials);
        println!("V1 login success");
    }
}