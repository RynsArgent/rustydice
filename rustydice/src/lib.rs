extern crate rand;

use rand::{thread_rng, Rng};
use rand::distributions::Uniform;

#[derive(Copy, Clone)]
pub enum Die {
    D2 = 2,
    D4 = 4,
    D6 = 6,
    D8 = 8,
    D10 = 10,
    D12 = 12,
    D20 = 20,
    D100 = 100
}

pub struct Dice { 
    die: Die,
    number_of_rolls: u32,
	modifier: i32,
}

pub struct RollResult {
    pub die: Die,
    pub number_of_rolls: u32,
	pub modifier: i32,
    pub rolls: Vec<u32>,
    pub total: i32,
}

impl Dice {
    pub fn new(die: Die) -> Dice {
        Dice { die, number_of_rolls: 1, modifier: 0 }
    }

    pub fn roll_n_times(&self, n: u32) -> Vec<u32> {
        let mut rolls = Vec::new();

        for _ in 0..n {
            rolls.push(self.roll());
        }

        rolls
    }

    pub fn number_of_rolls(&mut self, n: u32) -> &mut Dice {
        self.number_of_rolls = n;
        self
    }

	pub fn modifier(&mut self, m: i32) -> &mut Dice {
        self.modifier = m;
        self
    }

    pub fn execute(&self) -> RollResult {
        let rolls = self.roll_n_times(self.number_of_rolls);
        let sum: u32 = rolls.iter().sum();

        RollResult {
            die: self.die,
            number_of_rolls: self.number_of_rolls,
            modifier: self.modifier,
            total: sum as i32 + self.modifier,
            rolls,
        }
    }

    fn roll(&self) -> u32 {
        let range = Uniform::new_inclusive(1, self.die as u32);

        thread_rng().sample(range)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_number_between_1_and_20() {
        for _ in 0..100 {
            let die = Dice::new(Die::D20);
            let roll = die.roll();
            assert!(roll >= 1 && roll <= 20);
        }
    }

    #[test]
    fn it_generates_a_number_between_1_and_12() {
        for _ in 0..100 {
            let die = Dice::new(Die::D12);
            let roll = die.roll();
            assert!(roll >= 1 && roll <= 12);
        }
    }

    #[test]
    fn it_generates_a_number_between_1_and_10() {
        for _ in 0..100 {
            let die = Dice::new(Die::D10);
            let roll = die.roll();
            assert!(roll >= 1 && roll <= 10);
        }
    }

    #[test]
    fn it_returns_number_between_1_and_8() {
        for _ in 0..100 {
            let die = Dice::new(Die::D8);
            let roll = die.roll();
            assert!(roll >= 1 && roll <= 8);
        }
    }

    #[test]
    fn it_returns_number_between_1_and_6() {
        for _ in 0..100 {
            let die = Dice::new(Die::D6);
            let roll = die.roll();
            assert!(roll >= 1 && roll <= 6);
        }
    }

    #[test]
    fn it_generates_a_number_between_1_and_4() {
        for _ in 0..100 {
            let die = Dice::new(Die::D4);
            let roll = die.roll();
            assert!(roll >= 1 && roll <= 4);
        }
    }

    #[test]
    fn it_returns_a_vector_of_u32_within_range_of_die() {
        let d6 = Dice::new(Die::D6);
        let rolls = d6.roll_n_times(30);

        assert_eq!(rolls.len(), 30);

        for roll in rolls.iter() {
            assert!(*roll >= 1 && *roll <= 6);
        }
    }

    #[test]
    fn it_returns_a_roll_result_with_n() {
        let mut d4 = Dice::new(Die::D4);
        
        // set n
        d4.number_of_rolls(6);

        // execute the roll
        let result = d4.execute();

        assert_eq!(result.number_of_rolls, 6);
    }

	#[test]
    fn it_returns_a_roll_result_with_correct_total_vector() {
        let result = Dice::new(Die::D6).number_of_rolls(6).modifier(2).execute();

        let sum: u32 = result.rolls.iter().sum();
        let total = sum as i32 + result.modifier;

        assert_eq!(result.rolls.len(), 6);
        assert_eq!(result.total, total);
    }
}