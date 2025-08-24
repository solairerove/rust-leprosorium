fn main() {
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Nickel));
    println!("{}", value_in_cents(Coin::Dime));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    let mut describe_quarter = describe_state_quarter(Coin::Quarter(UsState::Alabama));
    match describe_quarter {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }

    describe_quarter = describe_state_quarter(Coin::Quarter(UsState::Alaska));
    match describe_quarter {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }

    describe_quarter = describe_state_quarter(Coin::Penny);
    match describe_quarter {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
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
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
