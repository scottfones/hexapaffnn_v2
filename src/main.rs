#[macro_use]
extern crate peroxide;

pub mod game;

fn main() {
    let g0 = game::new_game();
    let l = game::Location{m: 0, n: 0};
    println!("{}", g0.board);

    let g1 = g0.advance(l);
    println!("{}", g1.board);


}
