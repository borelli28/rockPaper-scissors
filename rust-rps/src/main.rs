use rand::Rng;
use std::io;


fn bot_pick() -> &'static str {
    // Generates random number
    let mut rng = rand::thread_rng();
    let random_number: usize = rng.gen_range(0..=2);

    let picks: [&str; 3] = ["Rock", "Paper", "Scissor"];
    let pick: &str = picks[random_number];
    return pick;
}

fn user_pick() -> &'static str {
    println!("Rock[R], Paper[P], or Scissors[S]: ");

    let mut input_raw = String::new();
    io::stdin()
        .read_line(&mut input_raw)
        .expect("Failed to read line");

    let input_raw = input_raw.trim().to_lowercase();
    let input: &str = input_raw.as_str();

    if input == "r" {
        return "Rock"
    } else if input == "p" {
        return "Paper"
    } else if input == "s" {
        return "Scissor"
    } else {
        return "None"
    }
}

fn calculate_winner(bot: &str, user: &str) -> &'static str {
    println!("Bot picked: {}, You picked: {}", bot, user);

    if bot == user {
        return "Both"
    } else if 
    (bot == "Rock" && user == "Scissor") || 
    (bot == "Paper" && user == "Rock") || 
    (bot == "Scissor" && user == "Paper") { 
        return "Bot"
    } else {
        return "User"
    }
}

fn main() {
    loop {
        let user_pick = user_pick();

        if user_pick == "None" {
            eprintln!("Invalid Input");
            break
        }

        let winner = calculate_winner(bot_pick(), user_pick);
        println!("Winner: {}", winner);
    }
}
