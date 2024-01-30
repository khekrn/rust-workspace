pub struct Credentials{
    username: String,
    password: String,
}

impl Credentials{
    pub fn new(username: String, password: String) -> Self {
        Credentials { username, password }
    }
}