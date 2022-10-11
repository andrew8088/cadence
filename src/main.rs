use std::{self, fs};
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let content = fs::read_to_string(file_path);

    let mut new_content = String::new();

    match content {
        Ok(c) => {
            let original_content = c.clone();
            let cards = parse(c);

            for mut card in cards {
                card.score = review(&card);
                new_content = format!("{}{}", new_content, card.to_string());
            }

            match fs::write(file_path, new_content) {
                Ok(_) => {
                    println!("all done!");
                }
                Err(_) => {
                    println!("something went wrong, attempting to roll bac");
                    match fs::write(file_path, original_content) {
                        Ok(_) => {
                            println!("rolled back, maybe try your session again?")
                        }
                        Err(_) => {
                            println!("sorry, no luck.");
                        }
                    }
                }
            }
        }
        Err(err) => println!("error reading file `{}`: {}", file_path, err),
    }
}

#[derive(Debug)]
struct Card {
    prompt: String,
    response: String,
    score: String,
}

impl ToString for Card {
    fn to_string(&self) -> String {
        format!("{} / {} / {}\n", self.prompt, self.response, self.score)
    }
}

fn parse(content: String) -> Vec<Card> {
    let mut v: Vec<Card> = Vec::new();

    for line in content.trim().split("\n") {
        let mut parts = line.split(" / ");

        let prompt = parts.next();
        let response = parts.next();
        let score = parts.next();

        match (prompt, response) {
            (Some(prompt), Some(response)) => {
                v.push(Card {
                    prompt: prompt.to_string(),
                    response: response.to_string(),
                    score: score.or(Some("")).unwrap().to_string(),
                });
            }
            _ => {
                println!("[ERR] line does not include prompt & response: [{}]", line);
            }
        }
    }

    return v;
}

fn review(card: &Card) -> String {
    println!("================================================");
    println!("PROMPT: {} (prev score: {})", card.prompt, card.score);
    println!("");
    let score = get_score().to_string();
    println!("RESPONSE: {}", card.response);

    score
}

fn get_score() -> &'static str {
    println!("This prompt was ");
    println!("j - easy");
    println!("k - medium");
    println!("l - hard");

    let mut buffer = String::new();
    let stdin = io::stdin();
    let result = stdin.read_line(&mut buffer);

    match result {
        Ok(_) => match buffer.trim() {
            "j" => {
                return "easy";
            }
            "k" => {
                return "medium";
            }
            "l" => {
                return "hard";
            }
            _ => {
                return get_score();
            }
        },
        Err(_) => return get_score(),
    }
}
