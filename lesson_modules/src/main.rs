use lesson_modules::v1::{auth_utils, authenticate};
use lesson_modules::v2::auth_utils as v2_auth;
use lesson_modules::v2::authenticate as v2_authenticate;

fn main() {
    let c1 = auth_utils::models::Credentials::new(String::from("c2fun"), String::from("@34324"));
    authenticate(c1);


    let c2 = v2_auth::models::Credentials::new(String::from("c2fun"), String::from("@34324"));
    v2_authenticate(c2);

}
