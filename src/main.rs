mod athlete;
mod parsing;
mod simulation;

use std::io::{self, prelude::*};

use num_format::{Locale, ToFormattedString};
use rand::Rng;

use athlete::Athlete;

fn main() {
    let athletes_toml = parsing::parse_athletes_toml_file("data/athletes.toml").unwrap();
    let mut simulation_info = parsing::get_simulation_info(&athletes_toml).unwrap();
    simulation_info
        .original_scores
        .sort_by(|lhs, rhs| rhs.cmp(lhs));

    let mut athletes = parsing::get_athletes(&athletes_toml).unwrap();
    athletes.sort_by(|lhs, rhs| rhs.cmp(lhs));

    let athlete_count = athletes.len();
    let multiply_factor = simulation_info.per_day_multiplier.powi(
        (simulation_info.current_event - simulation_info.completed_non_elimination_event_count)
            as i32,
    );

    let mut loss_counts = vec![0; athlete_count];

    for i in 1..=simulation_info.simulation_count {
        let mut shuffled_athletes = vec![None; athlete_count];
        let mut taken_placings = vec![false; athlete_count];

        let mut current_athletes = athletes.clone();

        for athlete_index in 0..athlete_count {
            while shuffled_athletes[athlete_index].is_none()
                || taken_placings[shuffled_athletes[athlete_index].unwrap()]
            {
                shuffled_athletes[athlete_index] =
                    Some(rand::thread_rng().gen_range(0..athlete_count));
            }

            taken_placings[shuffled_athletes[athlete_index].unwrap()] = true;
            current_athletes[athlete_index].score += ((simulation_info.original_scores
                [shuffled_athletes[athlete_index].unwrap()]
                as f32)
                * multiply_factor)
                .round() as u32;
        }

        let mut lowest_score = u32::max_value();
        let mut eliminated_athlete_index = 0;

        for (athlete_index, athlete) in current_athletes.iter().enumerate() {
            if athlete.score < lowest_score {
                lowest_score = athlete.score;
                eliminated_athlete_index = athlete_index;
            }
        }

        loss_counts[eliminated_athlete_index] += 1;

        if i % (simulation_info.simulation_count / 20) == 0 {
            println!(
                "{:.0}% done ({}/{}).",
                (i as f32) / simulation_info.simulation_count as f32 * 100.0,
                i.to_formatted_string(&Locale::en),
                simulation_info
                    .simulation_count
                    .to_formatted_string(&Locale::en),
            );
        }
    }

    display_results(&athletes, &loss_counts, simulation_info.simulation_count);
    pause_console();
}

fn display_results(athletes: &[Athlete], loss_counts: &[u32], simulation_count: u32) {
    assert_eq!(athletes.len(), loss_counts.len());

    let athlete_count = athletes.len();

    println!("\n---------------------------------\n");

    for athlete_index in 0..athlete_count {
        print!("{}'s percentage: ", athletes[athlete_index].name);

        let percentage =
            (loss_counts[athlete_index] as f32) / (simulation_count as f32) * 100.0;

        if percentage < 0.1 && percentage != 0.0 {
            println!("<0.1%");
        } else {
            println!("{:.1}%", percentage);
        }
    }
}

fn pause_console() {
    print!("Press enter to exit...");
    io::stdout().flush().unwrap();

    io::stdin().read(&mut [0]).unwrap();
}
