enum IpAddrKind {
    V4,
    V6,
}

/*
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
*/

// こういう書き方もできる
enum IpAddr {
    V4(String),
    V6(String),
}

// こういう引数が異なる形にすることもできる…
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

impl Message {
    fn call(&self) {
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // .. etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    Half,
    Full,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("From {:?}", state);
            25
        },
        // それ以外でヒットさせる
        _ => 0,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    /*
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    */

    {
        let home = IpAddr::V4(String::from("127.0.0.1"));
        let loopback = IpAddr::V6(String::from("::1"));
    }

    {
        let home = IpAddr2::V4(127, 0, 0, 1);
        let loopback = IpAddr2::V6(String::from("::1"));
    }

    let m = Message::Write(String::from("ドドンガドンドンドン"));
    m.call();

    {
        let some_number = Some(5);
        let some_string = Some("a string");

        let absent_number: Option<i32> = None;
    }

    {
        let x: i8 = 5;
        let y: Option<i8> = Some(5);

        // これはだめ x + None の場合があるため
        // Option<T> -> T の変換が必要
        // let sum = x + y;

        // 例えばこう unwrap する
        let sum = x + y.unwrap_or(0);
    }

    println!("cent is {}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    {
        // Optionalな戻り値の時、 {:?} みたいな形式にする必要あり
        println!("Five is {:?}", plus_one(Some(5)));
        println!("None is {:?}", plus_one(None));
    }

    {
        let some_u8_value = Some(0u8);
        // この時有効
        // let some_u8_value = Some(3u8);
        match some_u8_value {
            Some(3) => println!("three"),
            _ => (),
        }
        if let Some(3) = some_u8_value {
            println!("three");
        }
    }
}

fn route(ip_type: IpAddrKind) {

}