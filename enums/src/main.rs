enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(u8, u8, u8, u8),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let localhost = IpAddrKind::V4(127, 0, 0, 1); 

    let x: i8 = 5;
    let y : Option<i8> = Some(5);

    let sum: i8 = x + y.unwrap_or(0);
    println!("{}", sum);

    let quarter = Coin::Quarter(UsState::California);
    println!("{}", value_in_cents(quarter));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("three!"); 
    };
}

// fn route(ip_kind: IpAddrKind) {}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(y) => Some(y + 1),
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California
}

enum Coin {
    Penny, 
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { 
            println!("State quarter from: {:?}", state);
            return 25;
        }
    }
}
