use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize, Serialize};
use crate::pokemon::Pokemon;
use crate::utils::show_pokemons_list;

#[derive(Serialize, Deserialize)]
pub struct Breeding {
    pub(crate) pokemons: Vec<Pokemon>,
}

impl Breeding {
    pub(crate) fn new() -> Self {
        Breeding {
            pokemons: Vec::new(),
        }
    }

    pub(crate) fn add_pokemon(&mut self, pokemon: Pokemon) {
        println!("Ajout de {} à l'élevage", pokemon.name);
        self.pokemons.push(pokemon);
    }

    pub(crate) fn show_all(&self) {
        show_pokemons_list(&self.pokemons);
    }

    pub(crate) fn train_pokemons(&mut self, gain: u32) {
        for pokemon in self.pokemons.iter_mut() {
            pokemon.win_xp(gain);
        }
    }

    pub(crate) fn try_reproduce(&mut self, first_pokemon: &Pokemon, second_pokemon: &Pokemon) {
        match Pokemon::reproduce(first_pokemon, second_pokemon) {
            Some(child) => {
                println!("Un nouveau bébé est né !");
                child.show();
                self.pokemons.push(child);
            }
            None => {}
        }
    }

    pub(crate) fn save_to_file(&self) -> Result<(), Box<dyn Error>> {
        let file = File::create("pokemons.json")?;
        serde_json::to_writer_pretty(file, &self)?;
        println!("Sauvegarde effectuée avec succès");
        Ok(())
    }

    pub(crate) fn load_from_file() -> Result<Breeding, Box<dyn Error>> {
        let file = File::open("pokemons.json")?;
        let reader = BufReader::new(file);
        let breeding: Breeding = serde_json::from_reader(reader)?;
        println!("Chargement effectué avec succès");
        Ok(breeding)
    }

    pub(crate) fn list_by_type(&self) -> Vec<Pokemon> {
        let mut sorted = self.pokemons.clone();
        sorted.sort_by(|a, b| a.pokemon_type.cmp(&b.pokemon_type));
        sorted
    }

    pub(crate) fn list_by_level(&self) -> Vec<Pokemon> {
        let mut sorted = self.pokemons.clone();
        sorted.sort_by(|a, b| a.level.cmp(&b.level));
        sorted
    }
}