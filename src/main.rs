use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self};
// use std::io::{self, BufRead, BufReader, BufWriter};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CreatureCard {
    name: String,
    mana_cost: Option<String>,
    cmc: f64,
    type_line: String,
    oracle_text: Option<String>,
    power: Option<String>,
    toughness: Option<String>,
    // Nested objects work too!
    #[serde(default)]
    image_uris: Option<ImageUris>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ImageUris {
    small: Option<String>,
    normal: Option<String>,
    png: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_cmc = "cmc_3";

    // read json file content to a string
    let data = fs::read_to_string("creatures.json")?;

    // Parse the JSON directly into a HashMap of String to Vec<CreatureCard>
    let catalog: HashMap<String, Vec<CreatureCard>> = serde_json::from_str(&data)?;

    //get the cards at that cmc
    if let Some(cards) = catalog.get(target_cmc) {
        println!("There are {} creatures at {}", cards.len(), target_cmc);
        //pick a random card and print its name
        let mut rng = rand::rng();
        if let Some(card) = cards.choose(&mut rng) {
            println!("Chosen creature: {}", card.name)
        } else {
            println!("Error choosing card")
        }
    } else {
        eprintln!("Error: Key {} is missing from the JSON data", target_cmc);
    }

    Ok(())
}

//pass in cmc number
//given number, get random creature from json file
