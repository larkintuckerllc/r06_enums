fn main() {
    let coin = Coin::Penny;
    let coin2 = coin;
    // println!("The value of coin is: {:?}", coin); // BORROWED AFTER MOVE
    println!("The value of coin2 is: {:?}", coin2); // Penny
    coin2.display();
    let coin3 = Coin::Quarter(2);
    match coin3 {
        Coin::Penny => {
            println!("Lucky penny!");
        },
        Coin::Nickel => {
            println!("Shiny nickel!");
        },
        Coin::Dime => {
            println!("Small dime!");
        },
        Coin::Quarter(value) => {
            println!("Large quarter with value: {}", value); // 2
        },
        _ => ()
    }
    match coin3 {
        Coin::Quarter(value) => {
            println!("Large quarter with value: {}", value); // 2
        },
        _ => ()
    }
    if let Coin::Quarter(value) = coin3 {
        println!("Large quarter with value: {}", value); // 2
    } 

    // OPTION
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(i32),
    Euro,
}

impl Coin {
    fn display(&self) {
        println!("Displayed is: {:?}", self); // Penny
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
