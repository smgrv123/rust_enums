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
    println!("{}", return_val)
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
            println!("The State for the following quarter is {:?}",state);
            25
        }
    }
}
