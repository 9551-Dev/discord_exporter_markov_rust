use markov::Chain;
use std::fs;
use serde::Deserialize;
use serde_json;
use std::env;

#[derive(Deserialize)]
struct AuthorInfo {
    id: String,
}

#[derive(Deserialize)]
struct Message {
    author: AuthorInfo,
    content: String,
}

#[derive(Deserialize)]
struct MessageList {
    messages: Vec<Message>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Loading markov data from {}",&args[1]);

    let file_data = fs::read_to_string(&args[1]).expect("you fucking retard");
    let json_data: MessageList = serde_json::from_str(&file_data).unwrap();

    let mut markov = Chain::of_order(args[3].parse::<usize>().unwrap());

    for msg in json_data.messages.iter() {
        if args.len() <= 4 || msg.author.id == args[4] {
            markov.feed_str(&msg.content);
        }
    }

    let mut result = String::from("");
    for _ in 0..args[2].parse::<i32>().unwrap() {
        result += markov.generate_str().as_str();
        result += " ";
    }

    println!("\n{}",result);
}