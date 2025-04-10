use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq)]
pub enum PokemonType {
    Feu,
    Eau,
    Plante,
    Electrik,
    Roche,
    Psy,
    Vol,
    Insecte,
    Normal,
    Combat,
    Poison,
    Spectre,
    Dragon,
    Glace,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Femelle,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Pokemon {
    pub(crate) name: String,
    pub(crate) level: u8,
    pub(crate) pokemon_type: PokemonType,
    pub(crate) experience: u32,
    pub(crate) gender: Gender,
}

impl Pokemon {
    pub(crate) fn new(name: &str, pokemon_type: PokemonType, gender: Gender) -> Self {
        Pokemon {
            name: name.to_string(),
            level: 1,
            pokemon_type,
            experience: 0,
            gender,
        }
    }

    pub(crate) fn win_xp(&mut self, gain: u32) {
        self.experience += gain;
        while self.experience >= 100 {
            self.experience -= 100;
            self.level += 1;
            println!("{} passe au niveau {} !", self.name, self.level);
        }
    }

    pub(crate) fn show(&self) {
        println!("Nom : {}", self.name);
        println!("Niveau : {}", self.level);
        println!("Expérience : {}", self.experience);
        println!("Type : {:?}", self.pokemon_type);
        println!("Genre : {:?}", self.gender)
    }

    fn can_reproduce(&self, other: &Pokemon) -> bool {
        self.pokemon_type == other.pokemon_type
            && self.gender != other.gender
            && self.level >= 10
            && other.level >= 10
    }

    pub(crate) fn reproduce(first_pokemon: &Pokemon, second_pokemon: &Pokemon) -> Option<Pokemon> {
        if first_pokemon.can_reproduce(&second_pokemon) {
            let mut rng = rand::rng();
            let gender = if rng.random_bool(0.5) {
                Gender::Male
            } else {
                Gender::Femelle
            };
            Some(Pokemon::new("Mystere", first_pokemon.clone().pokemon_type, gender))
        } else {
            println!(
                "{} et {} ne peuvent pas se reproduire !",
                first_pokemon.name, second_pokemon.name
            );
            None
        }
    }
}