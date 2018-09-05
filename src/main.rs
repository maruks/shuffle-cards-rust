extern crate rand;

use rand::prelude::*;
use std::io;
use std::process::exit;

const NUMBER_OF_CARDS: usize = 52;

fn random_number_k(k: u32) -> u32 {
    thread_rng().gen_range(1, k + 1)
}

fn random_number(k: u32, multiplier: u32, digits: u32, result: u32) -> u32 {
    if digits == 0 {
        result
    } else {
        let rnd = random_number_k(k) - 1;
        random_number(k, multiplier * k, digits - 1, result + rnd * multiplier)
    }
}

fn random_card_index(k: u32, digits: u32, max: u32) -> u32 {
    let result = random_number(k, 1, digits, 0);
    if result > max {
        random_card_index(k, digits, max)
    } else {
        result % NUMBER_OF_CARDS as u32
    }
}

fn find_max_and_digits(k: u32, multiplier: u32, max: u32, digits: u32) -> (u32, u32) {
    if max == NUMBER_OF_CARDS as u32 - 1 {
        (max, digits - 1)
    } else if max > NUMBER_OF_CARDS as u32 - 1 {
        (max - max % (NUMBER_OF_CARDS as u32) - 1, digits - 1)
    } else {
        find_max_and_digits(k, k * multiplier, max + multiplier * (k - 1), digits + 1)
    }
}

fn shuffle_array(array: &mut [i32; NUMBER_OF_CARDS], k: u32) {
    let (max, digits) = find_max_and_digits(k, k, k - 1, 2);
    for i in 0..NUMBER_OF_CARDS {
        let rand_idx = random_card_index(k, digits, max) as usize;
        let t = array[i];
        array[i] = array[rand_idx];
        array[rand_idx] = t;
    }
}

fn cards_array() -> [i32; NUMBER_OF_CARDS] {
    let mut cards: [i32; NUMBER_OF_CARDS] = [0; NUMBER_OF_CARDS];
    for i in 0..NUMBER_OF_CARDS {
        cards[i] = i as i32;
    }
    cards
}

fn main() {
    println!("Please input k");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let k: u32 = input.trim().parse().expect("Not a number!");

    if k < 2 {
        exit(-1);
    }

    let mut cards = cards_array();

    shuffle_array(&mut cards, k);

    for i in 0..NUMBER_OF_CARDS {
        println!("elem at index {} = {}", i, cards[i]);
    }
}

mod tests {
    use cards_array;
    use shuffle_array;
    use NUMBER_OF_CARDS;

    fn array_diff(a: &[i32; NUMBER_OF_CARDS], b: &[i32; NUMBER_OF_CARDS]) -> u32 {
        let mut result = 0;
        for i in 0..NUMBER_OF_CARDS {
            if a[i] != b[i] {
                result += 1;
            }
        }
        result
    }

    #[test]
    fn shuffle_array_test() {
        let cards1 = cards_array();
        let cards2 = cards_array();

        let mut shuffled_cards = cards_array();
        shuffle_array(&mut shuffled_cards, 2);

        let diff_shuffled = array_diff(&shuffled_cards, &cards1);
        let diff_sorted = array_diff(&cards1, &cards2);

        assert!(diff_shuffled > 46);
        assert!(diff_sorted == 0);
    }
}
