pub mod all_functions {
    use rand::Rng;

    pub fn bet() -> f32 {
        println!("What is your bet?");
        let mut bet: String = String::new();
        std::io::stdin()
            .read_line(&mut bet)
            .expect("Failed to read line");
        let bet: f32 = match bet.trim().parse() {
            Ok(num) => num,
            Err(_) => 0.0,
        };
        bet
    }
    pub fn hand() -> i32 {
        let card1: i32 = rand::thread_rng().gen_range(1..=11);
        let card2: i32 = rand::thread_rng().gen_range(1..=11);
        let hand: i32 = card1 + card2;
        hand
    }

    pub fn dealer_hand() -> i32 {
        let card: i32 = rand::thread_rng().gen_range(1..=11);
        card
    }

    pub fn hit() -> i32 {
        let card: i32 = rand::thread_rng().gen_range(1..=11);
        print!("\t {card} \n");
        card
    }

    pub fn dealer_action(dh: i32) -> i32 {
        if dh < 17 {
            let card: i32 = rand::thread_rng().gen_range(1..=11);
            card
        } else if dh > 21 {
            println!("Dealer busts!");
            dh
        } else {
            dh
        }
    }
}
