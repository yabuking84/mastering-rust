
struct User {
    // TODO: Something goes here
    name: String,
    age: u8,
}
struct ShopItem {
    name: String,
    quantity: u32,
}


fn main() {
    println!("STRUCT");


    // 1
    // Complete the code by addressing the TODO.
    let user = User {
        name: String::from("Tom Riddle"),
        age: 17u8,
    };
    println!("User's name: {}", user.name);
    println!("User's age: {}", user.age);
    
    let item = create_item("Socks", 200);
    let in_stock = is_in_stock(&item);
    println!("{} is in stock: {in_stock}", item.name);

} 


fn create_item(name: &str, quantity: u32) -> ShopItem {
    ShopItem {
        name: name.to_string(),
        quantity, // notice how struct initializations can be shortened when variable and field have same name
    }
}

fn is_in_stock(item: &ShopItem) -> bool {
    item.quantity > 0
}
 