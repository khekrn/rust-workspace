fn main(){
    let username = get_user_name(1);
    match username{
        Some(name) => println!("Name = {name}"),
        None => {}
    }
    // We can write the better concise code using if let because
    // We don't want to consider both the variant of Option only one is required
    // In such cases we can rely on if-let syntax
    if let Some(name) = get_user_name(1){
        println!("Name = {name}");
    }
}

fn get_user_name(id: i32) -> Option<String>{
    let result = String::from("Ferris");
    if id == 1 {
        Some(result)
    }else{
        None
    }
}