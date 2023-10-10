trait Owns {
    fn has_xbox(&self) -> bool;
    fn has_n64(&self) -> bool;
}

#[derive(Debug)]
struct Custom {
    age: usize,
    name: String,
}

impl Owns for Custom {
    fn has_n64(&self) -> bool {
        if self.age > 30 {
            return false;
        }
        return true;
    }
    fn has_xbox(&self) -> bool {
        match self.name.as_str() {
            "John" => true,
            "Takeda" => false,
            _ => false,
        }
    }
}

#[derive(Debug)]
enum Item {
    Number(usize),
    String(String),
    MyCustom(Custom),
}

fn append(items: &mut Vec<Item>) {
    items.push(Item::String("Hello Fem".into()));
}

fn main() {
    let mut items: Vec<Item> = vec![
        Item::Number(1),
        Item::MyCustom(Custom {
            name: String::from("John"),
            age: 33,
        }),
    ];
    append(&mut items);

    println!("vector items");
    for item in items {
        println!("{:?}", item);

        match item {
            Item::Number(num) => println!("{}", num),
            Item::String(str) => println!("{}", str),
            Item::MyCustom(cus) => {
                println!("Hello {}!", cus.name);
                if cus.has_n64() {
                    println!("Nintendo has the best games!")
                }
                if cus.has_xbox() {
                    println!("Xbox is the best amirite?")
                }
            }
        }
    }
}
