#[derive(Debug)]
enum State {
    A,
    B,
    C,
    // random state names
}
enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn main() {
    let return_val = value_in_cents(Coins::Quarter(State::B));
    println!("{}", return_val);

    let five = Some(5);
    let plus_five_res = plus_five(five);
    println!("{:?}", plus_five_res);

    let dice_val = 8;
    match dice_val {
        3 => {
            println!("funny hat => {}", 3);
        }
        7 => {
            println!("NO return funny hat => {}", 7);
        }
        // other => println!("{}", other),
        _ => println!("reroll")
    }
}

fn value_in_cents(coins: Coins) -> u8 {
    match coins {
        Coins::Penny => {
            println!("I reached here");
            1
        }
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quarter(state) => {
            println!("The State for the following quarter is {:?}", state);
            25
        }
    }
}

fn plus_five(val: Option<u32>) -> Option<u32> {
    match val {
        None => None,
        Some(i) => Some(i + 5),
    }
}
