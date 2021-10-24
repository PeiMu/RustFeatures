pub(crate) fn enum_test() {
    // basic_test();
    // message_test();
    // option_test();
    // match_test();
    // match_option();
    // match_placeholder();
    if_let_test();
}

/**
 * 1. V4和V6是相同类型，可以作为同类型参数传入函数中
 * 2. 直接在enum中把具体的值类型写到每个枚举类型里，写法更简洁（不必再增加一个结构体或其他代码）
 * 3. 不同的枚举类型可以包含不同类型和数量的值
*/
#[derive(Debug)]
enum IpAddrKind {
    StrV4(String),
    StrV6(String),
    VarV4(u8, u8, u8, u8),
}

fn basic_test() {
    let home = IpAddrKind::StrV4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::StrV6(String::from("::1"));
    let local = IpAddrKind::VarV4(192, 168, 0, 1);
    route(home);
    route(loopback);
    route(local);
}

fn route(ip_kind: IpAddrKind) {
    println!("The Ip address is: {:?}", ip_kind);
}

#[derive(Debug)]
enum Message {
    Quit,                       // has no data associated with it at all.
    Move { x: i32, y: i32 },    // includes an anonymous struct inside it.
    Write(String),              // includes a single String.
    ChangeColor(i32, i32, i32), // includes three i32 values.
    Nested(Box<Message>), // insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `Message` representable
}

impl Message {
    fn previous_message(&self) {
        println!("previous message is: {:?}", self);
    }
}

fn message_test() {
    let mut m = Message::Write(String::from("hello"));
    m.previous_message();
    m = Message::Move { x: 45, y: 33 };
    m.previous_message();
    m = Message::ChangeColor(0, 196, 251);
    m.previous_message();
    m = Message::Quit;
    m.previous_message();
    m = Message::Nested(Box::from(m));
    m.previous_message();
}

fn option_test() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None; // 需要指明类型
    println!("some number: {:?}", some_number);
    println!("some string: {:?}", some_string);
    println!("absent number: {:?}", absent_number);

    let x: i8 = 5;
    let mut y: Option<i8> = None;
    // let sum = x + y; // error: no implementation for `i8 + Option<i8>`
    y = Some(6);
    let sum = x + y.expect("y is null");
    println!("sum: {}", sum);
}

fn match_test() {
    let mut coin = Coin::Penny;
    println!("The value in coin: {}", value_in_cents(coin));

    coin = Coin::Quarter(UsState::Alabama);
    println!("The value in coin: {}", value_in_cents(coin));
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn match_option() {
    let five = Some(5);
    println!("five plus one is: {:?}", plus_one(five));
    println!("None plus one is: {:?}", plus_one(None));
}

fn match_placeholder() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        _ => println!("other"),
    }
}

fn if_let_test() {
    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
