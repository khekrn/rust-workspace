pub fn login(credentials: models::Credentials){
    crate::v1::database::get_user();
    println!("login success");
}

fn logout(){

}

pub mod models;