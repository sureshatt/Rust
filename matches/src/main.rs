enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ..
}


fn main() {
    let dime = Coin::Dime;
    println!("value: {} cents", value_in_cents(&dime));

    let quater = Coin::Quater(UsState::Alaska);
    println!("value: {} cents", value_in_cents(&quater));

    let penny = Coin::Penny;

    built_with_copper(&penny);
    built_with_copper(&quater);

    // Options<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    print_options(six);
    print_options(none);

}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("UsSate: {:#?}", state);
            25
        },
    }
}

fn built_with_copper(coin: &Coin) -> bool {
    match coin {
        Coin::Penny => {
            println!("Penny is made with Copper");
            true
        },
        _ => { // this catch everything else. Or use 'other'
            println!("Nope. No Copper");
            return false;
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn print_options(x: Option<i32>) {
    match x {
        None => {
            println!("Value is None")
        },
        Some(i) => {
            println!("value is: {}",i);
        }
    }
}
