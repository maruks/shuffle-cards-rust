extern crate rand;

use rand::prelude::*;
use std::io;
use std::process::exit;

const NUMBER_OF_CARDS: usize = 52;
const RAND_BITS: u32 = 6;

fn random_number_k(k: u32) -> u32 {
    rand::thread_rng().gen_range(1, k + 1)
}

fn random_number(k: u32, bits: u32, result: u32) -> u32 {
    if bits == 0 {
        result
    } else {
        let rnd = random_number_k(k) - 1;
        random_number(k, bits - 1, (result << 1) | (rnd & 1))
    }
}

fn random_card_index(k: u32) -> u32 {
    let result = random_number(k, RAND_BITS, 0);
    if result < NUMBER_OF_CARDS as u32 {
        result
    } else {
        random_card_index(k)
    }
}

fn shuffle_array(array: &mut [i32; NUMBER_OF_CARDS], k: u32) {
    for i in 0..NUMBER_OF_CARDS {
        let rand_idx = random_card_index(k) as usize;
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
    use NUMBER_OF_CARDS;
    use shuffle_array;

    fn array_diff(a: [i32; NUMBER_OF_CARDS], b: [i32; NUMBER_OF_CARDS]) -> u32 {
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

        let diff_shuffled = array_diff(shuffled_cards, cards1);
        let diff_sorted = array_diff(cards1, cards2);

        assert!(diff_shuffled > 46);
        assert!(diff_sorted == 0);
    }
}
