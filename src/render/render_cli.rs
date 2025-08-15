// ///////////////////////////////////////////////////////////
/* Copyright (C) 2024 Faseeh Irfan - All Rights Reserved
 * You may use, distribute and modify this code under the
 * terms of the CC-BY Creative commons license.
 *
 * You should have received a copy of the CC- license with
 * this file. If not, look it up or something idk.
 */
// ///////////////////////////////////////////////////////////

use crate::engine::state::Gamestate;


/// Returns X if 3 and O if 2. 
/// Thats it. Its only used for pretty writing to CLI.
pub fn convert_to_xoxo(value: &i32) -> char {

    return match value {
        1 => '\u{25A1}',    // □
        2 => '\u{25CB}',    // ○
        3 => '\u{2715}',    // ✕
        _ => '?',           // something is terribly wrong... but i dont want to know that from the renderer
    }
}


pub fn render_gamestate(state: &Gamestate) {

    println!("{} {} {}", convert_to_xoxo(&state.state_array_row_1[0]), convert_to_xoxo(&state.state_array_row_1[1]), convert_to_xoxo(&state.state_array_row_1[2]));
    println!("{} {} {}", convert_to_xoxo(&state.state_array_row_2[0]), convert_to_xoxo(&state.state_array_row_2[1]), convert_to_xoxo(&state.state_array_row_2[2]));
    println!("{} {} {}", convert_to_xoxo(&state.state_array_row_3[0]), convert_to_xoxo(&state.state_array_row_3[1]), convert_to_xoxo(&state.state_array_row_3[2]));

}