// external crate imports
use inquire::{CustomType, validator::{Validation}};

// local imports
// mod utils;
// use crate::utils::math;

mod engine;
use crate::engine::state;

mod render;
use crate::render::render_cli;


// setup for CLI argument parser
/// Simple program to greet a person


fn main() {

    // first initialize the gamestate
    let mut current_gamestate : state::Gamestate = state::Gamestate{
        resolved: false,
        active_player: true,
        state_array_row_1: [1,1,1],
        state_array_row_2: [1,1,1],
        state_array_row_3: [1,1,1],
    };


    let validator = |input_r: &usize| if *input_r > 2 {
        Ok(Validation::Invalid("Only values up to 2 are allowed.".into()))
    } else {
        Ok(Validation::Valid)
    };

    /////////////////////////////
    // main turn loop begins here
    while !(&current_gamestate.resolved) {

        let active_player = &current_gamestate.active_player;

        match active_player {
            // its X's turn
            true => {

                let status_r = CustomType::<usize>::new("Player 1 plays. Where do you place an X (row)?")
                    .with_validator(validator)
                    .prompt();

                let status_c = CustomType::<usize>::new("Player 1 plays. Where do you place an X (column)?")
                    .with_validator(validator)
                    .prompt();

                let status = (status_r, status_c);

                match status {
                    (Ok(status_r), Ok(status_c)) => {
                        println!("Updating the gamestate...");
                        state::place_x(status_r, status_c, &mut current_gamestate);
                        state::check_winning(&mut current_gamestate);
                    },
                    (Err(err), .. ) => println!("Error while publishing your status: {}", err),
                    ( .., Err(err)) => println!("Error while publishing your status: {}", err),
                }
            },
            // its O's turn
            false => {

                let status_r = CustomType::<usize>::new("Player 2 plays. Where do you place an O (row)??")
                    .with_validator(validator)
                    .prompt();

                let status_c = CustomType::<usize>::new("Player 2 plays. Where do you place an O (column)??")
                    .with_validator(validator)
                    .prompt();

                let status = (status_r, status_c);

                match status {
                    (Ok(status_r), Ok(status_c)) => {
                        println!("Updating the gamestate...");
                        state::place_o(status_r, status_c, &mut current_gamestate);
                        state::check_winning(&mut current_gamestate);
                    },
                    (Err(err), .. ) => println!("Error while publishing your status: {}", err),
                    ( .., Err(err)) => println!("Error while publishing your status: {}", err),
                }

            },
        }

        // rendering time TODO: dump this all in one dedicated function
        println!("{} {} {}", render_cli::convert_to_xoxo(&current_gamestate.state_array_row_1[0]), render_cli::convert_to_xoxo(&current_gamestate.state_array_row_1[1]), render_cli::convert_to_xoxo(&current_gamestate.state_array_row_1[2]));
        println!("{} {} {}", render_cli::convert_to_xoxo(&current_gamestate.state_array_row_2[0]), render_cli::convert_to_xoxo(&current_gamestate.state_array_row_2[1]), render_cli::convert_to_xoxo(&current_gamestate.state_array_row_2[2]));
        println!("{} {} {}", render_cli::convert_to_xoxo(&current_gamestate.state_array_row_3[0]), render_cli::convert_to_xoxo(&current_gamestate.state_array_row_3[1]), render_cli::convert_to_xoxo(&current_gamestate.state_array_row_3[2]));

    }
}


