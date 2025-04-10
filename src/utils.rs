use prettytable::{row, Table};
use crate::pokemon::Pokemon;

pub fn show_pokemons_list(pokemons: &Vec<Pokemon>) {
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