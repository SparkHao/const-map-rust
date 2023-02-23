use const_map::{set, get};

fn main() {
    println!("Hello, world!");
    let key = String::from("key");
    let value = String::from("value");

    set(key.clone(), value);
    let v = get(key.clone());
    println!("get v: {:?}", v);

    let v = get(key.clone());
    println!("get v: {:?}", v);
}
