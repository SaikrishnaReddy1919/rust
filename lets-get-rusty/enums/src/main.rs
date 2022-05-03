// jus like structs, methods and associated functions can be defined for enums also
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    // or
    // V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn some_fn() {
        println!("Hello")
    }
}

// abive can also be defined like below as structs...but with enums multiple structs can be grouped together
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;

//     let localhost = IpAddrKind::V4(127, 0, 0, 1);
// }

//-----------------------------------------------------
//Option enum : looks like below
// enum Option<T> {
//     Some(T),
//     None
// }

fn main() {
    let some_number = Some(32);

    //string slice
    let some_string = Some("krishna");

    let no_num: Option<i32> = None;



    //
    let x: i32 = 10;
    let y = Some(20);

    let sum = x + y.unwrap_or(0);


    value_in_cents(Coin::Quarter(UsState::Alaska));
}

// match
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona
    //...
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin : Coin) -> u8 {
    match coin {
        Coin::Dime => 10,
        Coin::Nickel => 5,
        Coin::Quarter(state) => {
            println!("State is : {:?}", state);
            25
        },
        Coin::Penny => 1,
    }
}

