mod functions;
use crate::functions::all_functions;
pub fn start() {
    println!("Welcome to Blackjack");
    println!("Blackjack pays 3/2");
    println!("--------------------------\n");
    println!("Are you 21? Y/N");

    let mut answer: String = String::new();
    std::io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    let answer: String = answer.to_uppercase();
    let answer: &str = answer.trim_end();

    if answer == "Y" {
        println!("Great! Let's play!");
    } else {
        println!("Sorry :( You need to be 21 to play.");
        std::process::exit(0);
    }
}

pub fn game() {
    let bet: f32 = all_functions::bet();
    println!("Your bet: {:?}", bet);
    println!("-----------------");
    println!("Dealer's Hand");
    let dh: i32 = all_functions::dealer_hand();
    print!("{:?}", dh);
    print!("\t x \n");
    println!("Your hand:");
    let hand: i32 = all_functions::hand();
    println!("{:?}", hand);
    println!("What would you like to do?");
    println!("\t h - Hit");
    println!("\t d - Double");
    println!("\t s - Stand");
    println!("Your choose: ");
    let mut answer: String = String::new();
    std::io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    let answer: &str = answer.trim_end();
    let choice: i32 = choice(answer);
    let hand: i32 = choice + &hand;
    if hand > 21 {
        println!("{:?}", hand);
        println!("bust :(");
        std::process::exit(0);
    }
    if hand == 21 {
        println!("{:?}", hand);
        let black_jack: f32 = black_jack(bet);
        println!("Blackjack! Your winnings ${black_jack}")
    }
    println!("Your hand: {hand}");
    println!("--------------------------");
    println!("Dealer's hand");
    let dh: i32 = &dh + all_functions::dealer_hand();
    println!("{:?}", dh);
    println!("--------------------------");
    println!("Dealer's hand after action");
    println!("--------------------------");
    let dh: i32 = all_functions::dealer_action(dh) + &dh;
    println!("{:?}", dh);
    if dh > 21 {
        let winnings: f32 = bet * 2.0;
        println!("Dealer busts!");
        println!("Win! :) Your winnings ${winnings}");
    } else if hand > dh {
        let winnings: f32 = bet * 2.0;
        println!("Your hand: {hand} Dealer's Hand: {dh}");
        println!("Win! :) Your winnings ${winnings}");
    } else if hand == dh {
        println!("Your hand: {hand} Dealer's Hand: {dh}");
        println!("Push :/");
    } else {
        println!("Your hand: {hand} Dealer's Hand: {dh}");
        println!("Dealer wins :(");
    }
}
fn choice(x: &str) -> i32 {
    if x == "h" {
        let hand: i32 = all_functions::hit();
        hand
    } else if x == "d" {
        let hand: i32 = all_functions::hit();
        hand
    } else if x == "s" {
        0
    } else {
        println!("Error! Wrong key pressed. Defaulting to stand.");
        0
    }
}
fn black_jack(bet: f32) -> f32 {
    let bet: f32 = bet;
    let winnings: f32 = bet * 1.5;
    winnings
}
