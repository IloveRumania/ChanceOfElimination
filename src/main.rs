mod athlete;
mod parsing;
mod simulation;

fn main() {
    let athletes_toml = parsing::parse_athletes_toml_file("data/athletes.toml").unwrap();
    let simulation_info = parsing::get_simulation_info(&athletes_toml).unwrap();
    let athletes = parsing::get_athletes(&athletes_toml).unwrap();

    println!("{:#?}", athletes);
}
