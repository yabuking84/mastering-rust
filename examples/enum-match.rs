struct Product {
    name: String,
    category: ProductCategory,
    price: f32,
    in_stock: bool,
}

enum ProductCategory {
    Books,
    Clothing,
    Electrics,
}

#[derive(Debug)]  // this line makes the enum variants printable!
enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace { from: String, to: String },
}

impl Command {
    fn serialize(&self) -> String {
        let mut retVal: String;
        match self {
            Command::Undo => retVal = String::from("{ \"cmd\": \"undo\" }"),
            Self::Redo => retVal = String::from("{ \"cmd\": \"redo\" }"),
            Command::AddText(s) => {
                retVal = format!(
                    "{{ \
                        \"cmd\": \"add_text\", \
                        \"text\": \"{s}\" \
                    }}"
                );
            },
            Command::MoveCursor(line, column) => {
                retVal = format!(
                    "{{ \
                        \"cmd\": \"move_cursor\", \
                        \"line\": {line}, \
                        \"column\": {column} \
                    }}"
                );
            },
            Command::Replace { from, to } => {
                retVal = format!(
                    "{{ \
                        \"cmd\": \"replace\", \
                        \"from\": \"{from}\", \
                        \"to\": \"{to}\", \
                    }}"
                );
            },
        }

        println!("xxx {:?}", self);

        return retVal;

    }
}

fn main() {
    let category = ProductCategory::Electrics;
    let product = Product {
        name: String::from("TV"),
        category,
        price: 200.98,
        in_stock: true,
    };
    let age = 35;

    match age {
        1 => println!("Happy 1st Birthday!"),
        13..=19 => println!("You are a teenager!"),
        x => println!("You are {x} years old!"),
    }
    // println!("xxx {} ",age);
    age;

    let cmd1 = Command::Undo;
    let cmd2 = Command::AddText(String::from("test"));
    let cmd3 = Command::MoveCursor(22, 0);
    let cmd4 = Command::Replace {
        from: String::from("a"),
        to: String::from("b"),
    };

    if let Command::Undo = cmd1 {
        println!("is UNDO");
    }

    println!("{}", cmd1.serialize());
    println!("{}", cmd2.serialize());
    println!("{}", cmd3.serialize());
    println!("{}", cmd4.serialize());

    // tuple structs
    struct Y(i32);
    let Y(bb) = Y(33);
    println!("asd: {bb}"); // asd: 33

    println!("Command: {:?}", cmd4);
}
