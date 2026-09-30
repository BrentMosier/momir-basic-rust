use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter};

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

fn main() {
    println!("Hello, world!");
}

//pass in cmc number
//given number, get random creature from json file
