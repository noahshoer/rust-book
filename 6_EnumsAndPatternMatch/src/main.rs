fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    let m2 = Message::Quit;
    m2.call();

    let al_q = Coin::Quarter(UsState::Alabama);
    value_in_cents(&al_q);
    if let Some(desc) = describe_state_quarter(&al_q) {
            println!("{desc}");
    }

    let five = Some(5);
    let six = plus_one(five);
    print_num(&six);
    let none = plus_one(None);
    print_num(&none);

}

fn print_num(num: &Option<i32>) {
    if let Some(num) = num {
        println!("The value of the number is {num}");
    } else {
        println!("Nothing dawg");
    }
}

enum Message {
    Quit,
    Write(String),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Write(msg) => println!("Calling to say {msg}"),
            _ => return,
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama
}

#[allow(unused)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819
        }
    }
}

fn describe_state_quarter(coin: &Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
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