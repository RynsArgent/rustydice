extern crate chrono;
extern crate rustydice;

use chrono::Local;
use rustydice::{dice::Dice, dice::Die::*, logger};

fn main() {
    let result = Dice::new(D20).modifier(3).execute();
    println!("{}", logger::build_log(&result, &Local::now()));

    let result = Dice::new(D4).number_of_rolls(8).modifier(-5).execute();
    println!("{}", logger::build_log(&result, &Local::now()));

    let mut my_dice = Dice::new(D100);
    let my_dice = &mut my_dice.number_of_rolls(2);
    let result = my_dice.execute();
    println!("{}", logger::build_log(&result, &Local::now()));
}