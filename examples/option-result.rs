fn main() {
    println!("OPTION");
    let username = get_username(1);
    // if let Some(name) = username {
    //     println!("{name}");
    // }

    println!("USERNAME: {:?}", username);

    match username {
        Some(name) => println!("{name}"),
        None => println!("No username"),
    }
}

fn get_username(user_id: u32) -> Option<String> {
    let query = format!("GET username FROM users WHERE id={user_id}");
    let db_result = query_db(query);
    // db_result.err() // if you want to return an error
    db_result.ok()
}

fn query_db(query: String) -> Result<String, String> {
    if query.is_empty() {
        Err(String::from("Query string is empty!"))
    } else {
        Ok(String::from("Ferris"))
    }
}
