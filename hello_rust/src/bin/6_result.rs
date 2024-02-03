fn main(){
    let username = get_user_name(1);
    if let Some(name) = username{
        println!("Name = {name}");
    }
}

fn get_user_name(id: i32) -> Option<String>{
    let query = format!("GET username FROM users where id={id}");
    let db_result = query_db(query);
    db_result.ok()
}

fn query_db(query: String) -> Result<String, String>{
    if query.is_empty(){
        Err(String::from("Query string is empty"))
    }else{
        Ok(String::from("Ferris"))
    }
}