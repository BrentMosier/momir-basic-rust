# Run project
```zsh
cargo run
```

# What is this project?
A rust implementation of momir basic to run on a raspberry pi with a thermal printer

# Todo
- [x] Download card data from scryfall
- [ ] Download card art from scryfall (?)
- [ ] Given specified mana value, Fetch card and art
- [ ] Run project on raspberry pi
- [ ] Connect to hardware components

# Steps
- Download Oracle cards from [Scryfall](https://scryfall.com/docs/api/bulk-data) to get all unique card objects. These are downloaded as a jsonl file (as explained on the scryfall page). Extract the jsonl file and place it in the root of your project, noting the file name.
- Update the `input_path` value in `src/bin/setup.rs` to match the name of your oracle card file
- Run the following command to extract and filter out the creature card data:
```console 
cargo run --bin setup 
```
