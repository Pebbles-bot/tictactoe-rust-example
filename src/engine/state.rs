// ///////////////////////////////////////////////////////////
/* Copyright (C) 2024 Faseeh Irfan - All Rights Reserved
 * You may use, distribute and modify this code under the
 * terms of the CC-BY Creative commons license.
 *
 * You should have received a copy of the CC- license with
 * this file. If not, look it up or something idk.
 */
// ///////////////////////////////////////////////////////////


/// Struct that contains the gamestate, imagined as a board
/// Specifically, it contains: a bool for whether the game is over: false -> game continues; true-> game is over;
/// a bool for which player is acting
/// three arrays for each row of the board
pub struct Gamestate {
    pub resolved: bool,
    pub active_player: bool,
    pub state_array_row_1: [i32; 3],    // starts with [1,1,1],
    pub state_array_row_2: [i32; 3],    // starts with [1,1,1],
    pub state_array_row_3: [i32; 3],    // starts with [1,1,1],
}


/// checks if the game is over or not
/// logically: a row/col/diag line is represented with a number 1 * 2^x * 3^o, where x is the number of Xes in the line
/// and o is the number of Os in the line. This way Xes and Os can intermix but the game only ends if one of the lines is 
/// 8 or 27
/// then if the product of all the numbers in total is 2^5 * 3^4 =  2592 or 2^4 * 3^5 = 3888, it means every cell is filled and the game must be over
pub fn check_winning(state: &mut Gamestate) -> i128 {

    let mut prods: [i32; 8] = [1; 8];

    // calculate all 8 products for the rows cols and diags
    // if any row is exactly 2^3 = 8 or 3^3 = 27, then the game is over
    prods[0] = state.state_array_row_1.iter().product();
    prods[1] = state.state_array_row_2.iter().product();
    prods[2] = state.state_array_row_3.iter().product();
    prods[3] = state.state_array_row_1[0] * state.state_array_row_2[0] * state.state_array_row_3[0];
    prods[4] = state.state_array_row_1[1] * state.state_array_row_2[1] * state.state_array_row_3[1];
    prods[5] = state.state_array_row_1[2] * state.state_array_row_2[2] * state.state_array_row_3[2];
    prods[6] = state.state_array_row_1[0] * state.state_array_row_2[1] * state.state_array_row_3[2];
    prods[7] = state.state_array_row_1[2] * state.state_array_row_2[1] * state.state_array_row_3[0];

    let total_product : i32 = prods[0] * prods[1] * prods[2];

    for sum in prods.iter() {
        match sum {
            // if any of the products are exactly 3^3=27, X must have won, return immediately
            27 => { 
                println!("Game Over! X wins!");
                state.resolved = true;
                return 1;
            },
            // if any of the products are exactly 2^3, 0 must have won
            8 => {
                println!("Game Over! 0 wins!");
                state.resolved = true;
                return 2;
            },
            // in every other case, the row isnt resolved and checking must continue
            _ => {
                match total_product {
                    2592 | 3888 => {
                            println!("Game Over! Draw!");
                            state.resolved = true;
                            return 0;
                    },
                    _ => {
                        state.resolved = false;
                        continue;
                    }
                }
            },
        }
    }

    return 0;
}


/// simulate the act of placing an X in a board
/// this corresponds to multiplying the appropriate board point by 3
/// also fails to act if the value of the cell is >1, meaning its a filled cell
/// args:
///     row: the index of the row, from 0-2
///     col: the index of the column, from 0-2
///     state: the gamestate instance to modify
pub fn place_x(row: usize, col: usize, state: &mut Gamestate) -> usize {

    match row {
        0 => {
            match state.state_array_row_1[col] {
                1 => {
                    state.state_array_row_1[col] = state.state_array_row_1[col] * 3;
                    state.active_player=!state.active_player;
                    return 1;
                }
                _ => {
                    println!("The cell is already filled. Pick another cell.");
                    return 0;
                }
            }
        },
        1 => {
            match state.state_array_row_2[col] {
                1 => {
                    state.state_array_row_2[col] = state.state_array_row_2[col] * 3;
                    state.active_player=!state.active_player;
                    return 1;
                }
                _ => {
                    println!("The cell is already filled. Pick another cell.");
                    return 0;
                }
            }
        },
        2 => {

            match state.state_array_row_3[col] {
                1 => {
                    state.state_array_row_3[col] = state.state_array_row_3[col] * 3;
                    state.active_player=!state.active_player;
                    return 1;
                }
                _ => {
                    println!("The cell is already filled. Pick another cell.");
                    return 0;
                }
            }
        },
        _ => {
            println!("Invalid row or column number. Try again.");
            return 0;
            // TODO: learn error handling
        }
    }
}


/// simulate the act of placing an X in a board
/// this corresponds to multiplying the appropriate board point by 2
/// also fails to act if the value of the cell is >1, meaning its a filled cell
/// args:
///     row: the index of the row, from 0-2
///     col: the index of the column, from 0-2
///     state: the gamestate instance to modify
pub fn place_o(row: usize, col: usize, state: &mut Gamestate) -> usize {

    match row {
        0 => {
            match state.state_array_row_1[col] {
                1 => {
                    state.state_array_row_1[col] = state.state_array_row_1[col] * 2;
                    state.active_player=!state.active_player;
                    return 1;
                }
                _ => {
                    println!("The cell is already filled. Pick another cell.");
                    return 0;
                }
            }
        },
        1 => {
            match state.state_array_row_2[col] {
                1 => {
                    state.state_array_row_2[col] = state.state_array_row_2[col] * 2;
                    state.active_player=!state.active_player;
                    return 1;
                }
                _ => {
                    println!("The cell is already filled. Pick another cell.");
                    return 0;
                }
            }
        },
        2 => {

            match state.state_array_row_3[col] {
                1 => {
                    state.state_array_row_3[col] = state.state_array_row_3[col] * 2;
                    state.active_player=!state.active_player;
                    return 1;
                }
                _ => {
                    println!("The cell is already filled. Pick another cell.");
                    return 0;
                }
            }
        },
        _ => {
            println!("Invalid row or column number. Try again.");
            return 0;
            // TODO: learn error handling
        }
    }
}