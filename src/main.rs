use prettytable::{Table, row};
use rand::Rng;
use std::cmp::PartialEq;
use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq)]
enum PokemonType {
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
enum Gender {
    Male,
    Femelle,
}

#[derive(Clone, Serialize, Deserialize)]
struct Pokemon {
    name: String,
    level: u8,
    pokemon_type: PokemonType,
    experience: u32,
    gender: Gender,
}

impl Pokemon {
    fn new(name: &str, pokemon_type: PokemonType, gender: Gender) -> Self {
        Pokemon {
            name: name.to_string(),
            level: 1,
            pokemon_type,
            experience: 0,
            gender,
        }
    }

    fn win_xp(&mut self, gain: u32) {
        self.experience += gain;
        while self.experience >= 100 {
            self.experience -= 100;
            self.level += 1;
            println!("{} passe au niveau {} !", self.name, self.level);
        }
    }

    fn show(&self) {
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

    fn reproduce(first_pokemon: &Pokemon, second_pokemon: &Pokemon) -> Option<Pokemon> {
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

fn show_pokemons_list(pokemons: &Vec<Pokemon>) {
    let mut table = Table::new();
    table.add_row(row!["Nom", "Niveau", "Type", "Expérience", "Genre"]);

    for pokemon in pokemons {
        table.add_row(row![
            pokemon.name,
            pokemon.level,
            format!("{:?}", pokemon.pokemon_type),
            pokemon.experience,
            format!("{:?}", pokemon.gender),
        ]);
    }

    table.printstd();
}

#[derive(Serialize, Deserialize)]
struct Breeding {
    pokemons: Vec<Pokemon>,
}

impl Breeding {
    fn new() -> Self {
        Breeding {
            pokemons: Vec::new(),
        }
    }

    fn add_pokemon(&mut self, pokemon: Pokemon) {
        println!("Ajout de {} à l'élevage", pokemon.name);
        self.pokemons.push(pokemon);
    }

    fn show_all(&self) {
        show_pokemons_list(&self.pokemons);
    }

    fn train_pokemons(&mut self, gain: u32) {
        for pokemon in self.pokemons.iter_mut() {
            pokemon.win_xp(gain);
        }
    }

    fn try_reproduce(&mut self, first_pokemon: &Pokemon, second_pokemon: &Pokemon) {
        match Pokemon::reproduce(first_pokemon, second_pokemon) {
            Some(child) => {
                println!("Un nouveau bébé est né !");
                child.show();
                self.pokemons.push(child);
            }
            None => {}
        }
    }

    fn save_to_file(&self) -> Result<(), Box<dyn Error>> {
        let file = File::create("pokemons.json")?;
        serde_json::to_writer_pretty(file, &self)?;
        println!("Sauvegarde effectuée avec succès");
        Ok(())
    }

    fn load_from_file() -> Result<Breeding, Box<dyn Error>> {
        let file = File::open("pokemons.json")?;
        let reader = BufReader::new(file);
        let breeding: Breeding = serde_json::from_reader(reader)?;
        println!("Chargement effectué avec succès");
        Ok(breeding)
    }

    fn list_by_type(&self) -> Vec<Pokemon> {
        let mut sorted = self.pokemons.clone();
        sorted.sort_by(|a, b| a.pokemon_type.cmp(&b.pokemon_type));
        sorted
    }
}

fn main() {
    let mut breeding = Breeding::new();

    let pikachu = Pokemon::new("Pikachu", PokemonType::Electrik, Gender::Male);
    let salameche = Pokemon::new("Salamèche", PokemonType::Feu, Gender::Femelle);
    let bulbizarre = Pokemon::new("Bulbizarre", PokemonType::Plante, Gender::Male);
    let feunard = Pokemon::new("Feunard", PokemonType::Feu, Gender::Male);
    let galifeu = Pokemon::new("Galifeu", PokemonType::Feu, Gender::Femelle);

    breeding.add_pokemon(pikachu);
    breeding.add_pokemon(salameche);
    breeding.add_pokemon(bulbizarre);
    breeding.add_pokemon(feunard);
    breeding.add_pokemon(galifeu);

    breeding.show_all();
    breeding.train_pokemons(1125);
    breeding.show_all();

    let feunard_ref = breeding.pokemons[3].clone();
    let galifeu_ref = breeding.pokemons[4].clone();
    breeding.try_reproduce(&feunard_ref, &galifeu_ref);
    breeding.show_all();

    // save to file
    let _ = breeding.save_to_file();

    // Load from file
    // match Breeding::load_from_file() {
    //     Ok(reloaded) => {
    //         println!("Elevage rechargé !");
    //         reloaded.show_all();
    //     }
    //     Err(e) => eprintln!("Erreur au chargement : {}", e),
    // }

    println!("Liste des Pokémons triés par type");
    show_pokemons_list(&breeding.list_by_type());
}
