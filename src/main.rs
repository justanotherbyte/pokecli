//Declare dependencies
use std::io::stdin;
use reqwest;

fn main() {
	//Declare a mutable input string
    let mut pokemon_name = String::new();
    stdin().read_line(&mut pokemon_name)
    	.ok()
        .expect("Failed to read line");

    let json = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_name));
    println!(json);

    

    
}