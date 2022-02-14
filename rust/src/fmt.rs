#[macro_use]
extern crate lazy_static;

use colored::*;

const NAME: &str = "Name"; // Change name here
const NLEN: u32 = const_str::to_char_array!(NAME).len() as u32; // name length
lazy_static! { // Change colors here
    static ref AT: String = "@".red().to_string(); // colored filler symbol
    static ref VNAME: String = NAME.green().to_string() + if NLEN % 2 == 0 {&AT} else {""}; // colored name
}
const AT_LEN: u32 = (11 - NLEN) / 2; // to center the name in the heart

fn main() {
    let charset: [&str; 3] = [" ", &AT, &VNAME];
    let heartmodl: Vec<Vec<(usize, u32)>> = vec![
        vec![(0, 1), (1, 5), (0, 3), (1, 5)],
        vec![(1, 7), (0, 1), (1, 7)],
        vec![(0, 1), (1, 13)],
        vec![(0, 2), (1, AT_LEN), (2, 1), (1, AT_LEN)],
        vec![(0, 3), (1, 9)],
        vec![(0, 4), (1, 7)],
        vec![(0, 6), (1, 3)],
    ];
    for l in heartmodl {
        for sq in l {
            for _ in 0..sq.1 {
                print!("{}", charset[sq.0]);
            }
        }
        println!();
    }
}
