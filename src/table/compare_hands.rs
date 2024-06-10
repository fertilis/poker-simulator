use super::hand_evaluation::{self as he, Rankable};

use super::components::Card;

pub fn compare_hands(hands: &Vec<Vec<Card>>) -> Vec<usize> {
    if hands.len() == 0 {
        return vec![];
    }
    if hands.len() == 1 {
        return vec![0];
    }
    let mut ranks: Vec<he::Rank> = Vec::new();
    for hand in hands {
        let hand: Vec<he::Card> = hand.iter().map(|x| he::Card::new(x.0)).collect();
        let rank = hand.rank();
        ranks.push(rank);
    }
    let max_rank = ranks.iter().max().unwrap();
    let mut indices = Vec::new();
    for (i, rank) in ranks.iter().enumerate() {
        if rank == max_rank {
            indices.push(i);
        }
    }
    indices
}
