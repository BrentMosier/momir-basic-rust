// Read oracle cards jsonl to get all creature cards and write them to a json file
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter};

// Define the exact fields you want to keep.
// Anything omitted here is completely ignored during deserialization.
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

fn main() -> io::Result<()> {
    //change the name of the input path to match your jsonl file name
    //keep jsonl file in root of project
    let input_path = "oracle-cards-20260805090244.jsonl";
    let output_path = "creatures.json";

    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);

    let mut cmc_groups: BTreeMap<String, Vec<CreatureCard>> = BTreeMap::new();

    println!("Reading, filtering, and limiting creature card data");
    // let mut first_line = String::new();
    // reader.read_line(&mut first_line)?;

    // if first_line.trim().is_empty() {
    //     print!("The first line is empty or the file is blank");
    // } else {
    //     println!("\n--- SUCCESS! FIRST LINE READ ---");
    //     println!("{}", first_line.trim());
    //     println!("---------------------------------\n");
    // }

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        //Directly parse the line into our small typed struct
        if let Ok(card) = serde_json::from_str::<CreatureCard>(&line) {
            //perform creature filter
            if card.type_line.contains("Creature") {
                //filter fractional mana costs
                if card.cmc % 1.0 != 0.0 {
                    continue;
                }

                //create the integer key name
                let cmc_key = format!("cmc_{}", card.cmc as u32);

                //save the stripped down card struct into the btreemap
                cmc_groups
                    .entry(cmc_key)
                    .or_insert_with(Vec::new)
                    .push(card);
            }
        }
    }

    println!("Writing filterd cards to {}...", output_path);

    //cards are filtered and stored in btreemap, write them to json file
    let output_file = File::create(output_path)?;
    let writer = BufWriter::new(output_file);
    serde_json::to_writer_pretty(writer, &cmc_groups)
        .expect("Failed to write valid nested JSON structure to disk");
    println!("\nExtraction complete! Summary of entries saved:");
    for (cmc, cards) in &cmc_groups {
        println!(" - {}: {} cards", cmc, cards.len());
    }

    Ok(())
}
