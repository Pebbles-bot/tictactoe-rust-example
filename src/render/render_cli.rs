// ///////////////////////////////////////////////////////////
/* Copyright (C) 2024 Faseeh Irfan - All Rights Reserved
 * You may use, distribute and modify this code under the
 * terms of the CC-BY Creative commons license.
 *
 * You should have received a copy of the CC- license with
 * this file. If not, look it up or something idk.
 */
// ///////////////////////////////////////////////////////////


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
