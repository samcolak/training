

fn hello_friend(name1: String) -> String {

    let mut _name2 = "Bob".to_string();

    _name2 = _name2.to_uppercase();

    println!("Lets go Rust! Hello {} - From {}" , _name2, name1);    

    name1.to_lowercase()

}


fn main() {

    let mut _name1 = "Sam".to_string();

    _name1 = hello_friend(_name1);

    println!("Hello, world! Welcome {}" , _name1);

}
