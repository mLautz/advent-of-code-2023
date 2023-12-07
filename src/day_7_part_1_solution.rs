use std::cmp::Ordering;
use std::fs;
use fancy_regex::Regex;

// WRONG - 249100464

enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind
}

pub fn day_seven_part_1_solution() {
    println!("Day 7 - Part 1");

    let contents = fs::read_to_string("./inputs/day-7-input-example").unwrap();
    let hands = parse_hands(contents);

    let ranked_hands = sort_hands(hands);
    // dbg!(&ranked_hands);

    let mut ranking = 1;
    let mut total_winnings = 0;
    for hand in ranked_hands {
        // println!("Adding to winnings {} * {} --> {}", hand.2, ranking, hand.2 * ranking);
        total_winnings += hand.2 * ranking;
        ranking += 1;
    }

    println!("Total winnings ===> {}", total_winnings);
}

fn parse_hands(file_content: String) -> Vec<(String, u32, u32)> {
    let mut card_hands = Vec::new();

    for line in file_content.lines() {
        let line_split: Vec<&str> = line.split(' ').collect();
        let hand_string = line_split.get(0).unwrap().to_string();
        let bid_string = line_split.get(1).unwrap();

        let hand_type: u32 = determine_hand_type(&hand_string);

        card_hands.push((hand_string, hand_type, bid_string.parse::<u32>().unwrap()));
    }

    return card_hands;
}

fn determine_hand_type(hand_string: &String) -> u32 {
    let mut hand_type = None;

    let five_kind_regex = Regex::new(r"([A-Z0-9])\1\1\1\1").unwrap();
    if five_kind_regex.is_match(hand_string).unwrap() { hand_type = Some(HandType::FiveKind)}

    if hand_type.is_none() {
        let four_kind_regex = Regex::new(r"\S{0,1}(\S)\S{0,1}\1\S{0,1}\1\S{0,1}\1\S{0,1}").unwrap();
        if four_kind_regex.is_match(hand_string).unwrap() { hand_type = Some(HandType::FourKind)}
    }

    if hand_type.is_none() {
        let full_house_regex = Regex::new(r"^(\S)\1{0,2}(\S)\2{0,2}\1{0,2}\2{0,2}\1{0,1}$").unwrap();
        if full_house_regex.is_match(hand_string).unwrap() { hand_type = Some(HandType::FullHouse)}
    }

    if hand_type.is_none() {
        let three_kind_regex = Regex::new(r"\S{0,1}([A-Z0-9])\S{0,1}\1\S{0,1}\1\S{0,1}").unwrap();
        if three_kind_regex.is_match(hand_string).unwrap() { hand_type = Some(HandType::ThreeKind)}
    }

    if hand_type.is_none() {
        // KJJK - \S{0,1}([A-Z0-9])\S{0,1}([A-Z0-9])\S{0,1}\2\S{0,1}\1\S{0,1}
        // KKJJ - \S{0,1}([A-Z0-9])\S{0,1}\3\S{0,1}([A-Z0-9])\S{0,1}\4\S{0,1}
        // KJKJ - \S{0,1}([A-Z0-9])\S{0,1}([A-Z0-9])\S{0,1}\5\S{0,1}\6\S{0,1}
        let two_pair_regex = Regex::new(r"\S{0,1}([A-Z0-9])\S{0,1}([A-Z0-9])\S{0,1}\2\S{0,1}\1\S{0,1}|\S{0,1}([A-Z0-9])\S{0,1}\3\S{0,1}([A-Z0-9])\S{0,1}\4\S{0,1}|\S{0,1}([A-Z0-9])\S{0,1}([A-Z0-9])\S{0,1}\5\S{0,1}\6\S{0,1}").unwrap();
        if two_pair_regex.is_match(hand_string).unwrap() { hand_type = Some(HandType::TwoPair)}
    }

    if hand_type.is_none() {
        let one_pair_regex = Regex::new(r"([A-Z0-9])\S{0,3}\1").unwrap();
        if one_pair_regex.is_match(hand_string).unwrap() { hand_type = Some(HandType::OnePair)}
    }

    if hand_type.is_none() {
        hand_type = Some(HandType::HighCard);
    }

    return match hand_type.unwrap() {
        HandType::HighCard => 1,
        HandType::OnePair => 2,
        HandType::TwoPair => 3,
        HandType::ThreeKind => 4,
        HandType::FullHouse => 5,
        HandType::FourKind => 6,
        HandType::FiveKind => 7
    }
}

fn sort_hands(hands: Vec<(String, u32, u32)>) -> Vec<(String, u32, u32)> {
    let mut sorted_hands = hands.clone();
    sorted_hands.sort_by(|a: &(String, u32, u32), b: &(String, u32, u32)| {
        let mut hand_rank_order = a.1.cmp(&b.1);
        if hand_rank_order == Ordering::Equal {
            hand_rank_order = rank_by_first_highest(&a.0,&b.0);
        }

        return hand_rank_order;
    });

    return sorted_hands;
}

fn rank_by_first_highest(a: &String, b: &String) -> Ordering {
    let card_pairs = a.chars().zip(b.chars());

    for pair in card_pairs {
        let order = get_card_val(pair.0).cmp(&get_card_val(pair.1));
        if order != Ordering::Equal { return order }
    }

    return Ordering::Equal;
}

fn get_card_val(card: char) -> u32 {
    return match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0
    }
}