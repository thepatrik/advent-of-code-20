use std::collections::HashSet;

static FILENAME: &str = "input/data";

type Deck = Vec<usize>;

fn main() {
    let data = std::fs::read_to_string(FILENAME).expect("could not read file");
    println!("part one: {}", part_one(&data));
    println!("part two: {}", part_two(&data));
}

fn part_one(data: &str) -> usize {
    let orig_decks = to_decks(&data);
    let decks = reg_combat(orig_decks);
    let winner = if decks.1.is_empty() { decks.0 } else { decks.1 };
    winner
        .iter()
        .rev()
        .enumerate()
        .map(|(x, card)| (x + 1) * card)
        .sum()
}

fn part_two(data: &str) -> usize {
    let orig_decks = to_decks(&data);
    let decks = rec_combat(orig_decks);
    let winner = if decks.1.is_empty() { decks.0 } else { decks.1 };
    winner
        .iter()
        .rev()
        .enumerate()
        .map(|(x, card)| (x + 1) * card)
        .sum()
}

fn reg_combat(mut decks: (Deck, Deck)) -> (Deck, Deck) {
    while decks.0.len() > 0 && decks.1.len() > 0 {
        let card_1 = decks.0.remove(0);
        let card_2 = decks.1.remove(0);

        if card_1 > card_2 {
            decks.0.insert(decks.0.len(), card_1);
            decks.0.insert(decks.0.len(), card_2);
        } else {
            decks.1.insert(decks.1.len(), card_2);
            decks.1.insert(decks.1.len(), card_1);
        }
    }
    decks
}

fn rec_combat(mut decks: (Deck, Deck)) -> (Deck, Deck) {
    let mut cache: HashSet<(Deck, Deck)> = HashSet::new();

    while decks.0.len() > 0 && decks.1.len() > 0 {
        if !cache.insert(decks.clone()) {
            return (decks.0, vec![]);
        }

        let card_1 = decks.0.remove(0);
        let card_2 = decks.1.remove(0);

        let is_winner = |decks: (Deck, Deck)| {
            if decks.0.len() >= card_1 && decks.1.len() >= card_2 {
                let sub_decks = (
                    decks.0.clone().into_iter().take(card_1).collect(),
                    decks.1.clone().into_iter().take(card_2).collect(),
                );
                return rec_combat(sub_decks).1.is_empty();
            }
            card_1 > card_2
        };

        if is_winner(decks.clone()) {
            decks.0.insert(decks.0.len(), card_1);
            decks.0.insert(decks.0.len(), card_2);
        } else {
            decks.1.insert(decks.1.len(), card_2);
            decks.1.insert(decks.1.len(), card_1);
        }
    }

    decks
}

fn to_decks(data: &str) -> (Deck, Deck) {
    let mut decks: (Deck, Deck) = (Vec::new(), Vec::new());
    let mut input = data.split("\n\n");
    decks.0 = input
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|card| card.parse().unwrap())
        .collect::<Deck>();
    decks.1 = input
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|card| card.parse().unwrap())
        .collect::<Deck>();

    decks
}

mod tests {
    #[test]
    fn test_part_one() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        assert_eq!(33925, super::part_one(&data));
    }

    #[test]
    fn test_part_two() {
        let data = std::fs::read_to_string(super::FILENAME).expect("could not read file");
        assert_eq!(33441, super::part_two(&data));
    }
}
