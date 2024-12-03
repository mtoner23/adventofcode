use sorted_list::SortedList;
use std::cmp::*;
use std::error::Error;
use std::fs;

#[repr(u8)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
enum Type {
    Five = 6,
    Four = 5,
    FullHouse = 4,
    Three = 3,
    Two = 2,
    One = 1,
    High = 0,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

fn create_card(input: char) -> Card {
    match input {
        'A' => Card::Ace,
        'K' => Card::King,
        'Q' => Card::Queen,
        'T' => Card::Ten,
        '9' => Card::Nine,
        '8' => Card::Eight,
        '7' => Card::Seven,
        '6' => Card::Six,
        '5' => Card::Five,
        '4' => Card::Four,
        '3' => Card::Three,
        '2' => Card::Two,
        'J' => Card::Joker,
        _ => todo!("Unkown Card Char"),
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand {
    cards: String,
    hand_type: Type,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        } else {
            for (s, o) in self.cards.chars().zip(other.cards.chars()) {
                let comparison = create_card(s).cmp(&create_card(o));
                if comparison == Ordering::Equal {
                    continue;
                } else {
                    // println!("hands {:?}", comparison);
                    return comparison;
                }
            }
            println!("ERR: Hands Equal");
            return Ordering::Equal;
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

fn create_hand(cards: &str) -> Hand {
    let mut chars: Vec<char> = vec![];
    let mut counts: Vec<usize> = vec![];
    let mut jokers = 0;
    println!("input: {}", cards);
    for c in cards.chars() {
        let mut found = false;
        if c == 'J' {
            jokers += 1;
            continue;
        }
        for (count, test) in counts.iter_mut().zip(chars.clone()) {
            if c == test {
                *count += 1;
                found = true;
                break;
            }
        }
        if !found {
            chars.push(c);
            counts.push(1);
        }
    }

    println!("chars {:?}, counts {:?}, jokers {}", chars, counts, jokers);
    let max = counts.iter_mut().max();

    (if let Some(m) = max {
        *m += jokers;
    } else {
        counts.push(5);
        chars.push('J');
    });

    // *max += jokers;

    println!("chars {:?}, counts {:?}", chars, counts);

    let hand_type = assign_type(counts);
    return Hand {
        cards: cards.to_string(),
        hand_type: hand_type,
    };
}

fn assign_type(counts: Vec<usize>) -> Type {
    assert_eq!(counts.iter().sum::<usize>(), 5);
    if counts.len() == 1 {
        Type::Five
    } else if counts.len() == 2 {
        if counts[0] == 3 || counts[0] == 2 {
            Type::FullHouse
        } else {
            assert!(counts[0] == 4 || counts[0] == 1);
            Type::Four
        }
    } else if counts.len() == 3 {
        if counts.contains(&3) {
            return Type::Three;
        } else {
            return Type::Two;
        }
    } else if counts.len() == 4 {
        return Type::One;
    } else {
        assert_eq!(counts.len(), 5);
        return Type::High;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "src/input.txt";
    let file = fs::read_to_string(filepath)?;

    let lines = file.lines();

    let mut list: SortedList<Hand, usize> = SortedList::new();
    let mut unsorted: Vec<Hand> = vec![];
    for line in lines {
        let mut split = line.split_whitespace();
        let hand = create_hand(split.next().unwrap());
        let bid: usize = split.next().unwrap().parse::<usize>().unwrap();
        println!("h: {:?}, b {:?}", hand, bid);
        list.insert(hand.clone(), bid);
        unsorted.push(hand.clone());
    }

    let mut rank = 1;
    let mut sum = 0;
    for l in list.iter() {
        sum += l.1 * rank;
        rank += 1;
        println!("{:?}", l);
    }

    println!("Sum: {}", sum);

    return Ok(());
}
