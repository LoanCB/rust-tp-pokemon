mod pokemon;
mod breeding;
mod utils;

use rand::Rng;
use std::cmp::PartialEq;
use serde::{Serialize, Deserialize};
use std::error::Error;
use crate::breeding::Breeding;
use crate::pokemon::{Gender, Pokemon, PokemonType};
use crate::utils::show_pokemons_list;

fn main() {
    // let mut breeding = Breeding::new();
    //
    // let pikachu = Pokemon::new("Pikachu", PokemonType::Electrik, Gender::Male);
    // let salameche = Pokemon::new("Salamèche", PokemonType::Feu, Gender::Femelle);
    // let bulbizarre = Pokemon::new("Bulbizarre", PokemonType::Plante, Gender::Male);
    // let feunard = Pokemon::new("Feunard", PokemonType::Feu, Gender::Male);
    // let galifeu = Pokemon::new("Galifeu", PokemonType::Feu, Gender::Femelle);
    //
    // breeding.add_pokemon(pikachu);
    // breeding.add_pokemon(salameche);
    // breeding.add_pokemon(bulbizarre);
    // breeding.add_pokemon(feunard);
    // breeding.add_pokemon(galifeu);
    //
    // breeding.show_all();
    // breeding.train_pokemons(1125);
    // breeding.show_all();
    //
    // let feunard_ref = breeding.pokemons[3].clone();
    // let galifeu_ref = breeding.pokemons[4].clone();
    // breeding.try_reproduce(&feunard_ref, &galifeu_ref);
    // breeding.show_all();
    //
    // // save to file
    // let _ = breeding.save_to_file();

    // Load from file
    let mut breeding = Breeding::load_from_file()
        .map(|b| {
            println!("Elevage rechargé !");
            b
        })
        .unwrap_or_else(|e| {
            eprintln!("Erreur au chargement : {}", e);
            Breeding::new()
        });

    // List basic
    breeding.show_all();

    // List by type
    println!("Liste des Pokémons triés par type");
    show_pokemons_list(&breeding.list_by_type());

    // List by level
    println!("Liste des Pokémons triés par niveau");
    show_pokemons_list(&breeding.list_by_level());
}
