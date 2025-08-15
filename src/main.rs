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




fn main() -> Result<(), Error> {

    // first initialize the gamestate
    let mut current_gamestate : state::Gamestate = state::Gamestate{
        resolved: false,
        active_player: true,
        state_array_row_1: [1,1,1],
        state_array_row_2: [1,1,1],
        state_array_row_3: [1,1,1],
    };


    // its used for input checking
    let validator = |input: &usize| if *input > 3 || *input < 1 {
        Ok(Validation::Invalid("Only integers 1-3 are allowed.".into()))
    } else {
        Ok(Validation::Valid)
    };


    // OriolFilter — 5:22 PM i would recommend first showing the empty minefield
    render_cli::render_gamestate(&current_gamestate);

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
                        state::place_x(status_r-1, status_c-1, &mut current_gamestate);
                        state::check_winning(&mut current_gamestate);
                    },
                    (Err(err), .. ) => { println!("Error while publishing your status: {}", err); break; },
                    ( .., Err(err)) => { println!("Error while publishing your status: {}", err); break; },
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
                        state::place_o(status_r-1, status_c-1, &mut current_gamestate);
                        state::check_winning(&mut current_gamestate);
                    },
                    (Err(err), .. ) => { println!("Error while publishing your status: {}", err); break; },
                    ( .., Err(err)) => { println!("Error while publishing your status: {}", err); break; },
                }

            },
        }

        render_cli::render_gamestate(&current_gamestate);
    }
    Ok(())
}


