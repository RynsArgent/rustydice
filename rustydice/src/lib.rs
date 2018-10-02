extern crate rand;

use rand::{thread_rng, Rng};
use rand::distributions::Uniform;

pub struct Dice { sides: u32 }

impl Dice {
    pub fn new(sides: u32) -> Dice {
        Dice { sides }
    }

    fn roll(&self) -> u32 {
        let range = Uniform::new_inclusive(1, self.sides);

        thread_rng().sample(range)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_generates_a_number_between_1_and_20() {
        for _ in 0..100 {
            let die = Dice::new(20);
            let roll = die.roll();
            assert!(roll >= 1 && roll <= 20);
        }
    }

    #[test]
    fn it_generates_a_number_between_1_and_12() {
        for _ in 0..100 {
            let die = Dice::new(12);
            let roll = die.roll();
            assert!(roll >= 1 && roll <= 12);
        }
    }

    #[test]
    fn it_generates_a_number_between_1_and_10() {
        for _ in 0..100 {
            let die = Dice::new(10);
            let roll = die.roll();
            assert!(roll >= 1 && roll <= 10);
        }
    }

    #[test]
    fn it_returns_number_between_1_and_8() {
        for _ in 0..100 {
            let die = Dice::new(8);
            let roll = die.roll();
            assert!(roll >= 1 && roll <= 8);
        }
    }

    #[test]
    fn it_returns_number_between_1_and_6() {
        for _ in 0..100 {
            let die = Dice::new(6);
            let roll = die.roll();
            assert!(roll >= 1 && roll <= 6);
        }
    }

    #[test]
    fn it_generates_a_number_between_1_and_4() {
        for _ in 0..100 {
            let die = Dice::new(4);
            let roll = die.roll();
            assert!(roll >= 1 && roll <= 4);
        }
    }
}