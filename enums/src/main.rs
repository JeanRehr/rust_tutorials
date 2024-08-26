enum IpAddr {
    V4(String),
    V6(String),
}

fn route(ip_kind: IpAddr) {}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

/* same as
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
*/

impl Message {
    fn call(&self) {
        /*
        match self {
            Message::Quit => {
                println!("Quit");
            }
            Message::Move { x, y } => {
                println!("Moving to x: {}, y: {}", x, y);
            }
            Message::Write(text) => {
                println!("{}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Changing color to r: {}, g: {}, b: {}", r, g, b);
            }
        } */
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    //states
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}

fn main() {
    let four: IpAddr = IpAddr::V4("111.0.0.1".to_string());
    let six: IpAddr = IpAddr::V6("::2".to_string());
    let home: IpAddr = IpAddr::V4(String::from("127.0.0.1"));
    let loopback: IpAddr = IpAddr::V6(String::from("::1"));
    route(IpAddr::V4("Test".to_string()));

    //let m: Message = Message::Write(String::from("test"));
    //m.call();

    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five);
    let none: Option<i32> = plus_one(None);

    let dice_roll: u8 = 9;
    match dice_roll {
        3 => fancy_hat(),
        7 => remove_fancy_hat(),
        //other => move_player(other), //catch all
        //_ => rerrol(), //catch all pattern
        _ => (), //do nothing (empty tuple type)
    }
}

fn fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn rerrol() {}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
