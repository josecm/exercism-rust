use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

#[derive(Clone)]
struct BucketState {
    vols: (u8, u8),
    moves: u8,
    visited: HashSet<(u8, u8)>,
}

/// Solve the bucket problem
pub fn solve(cap1: u8, cap2: u8, goal: u8, start_bucket: &Bucket) -> Option<BucketStats> {

    let mut states = VecDeque::new();
    let (init_state, forbidden_state) = match start_bucket {
        &Bucket::One => ((cap1, 0), (0, cap2)),
        &Bucket::Two => ((0, cap2), (cap1, 0)),
    };
    states.push_back(BucketState {
        vols: init_state,
        moves: 1,
        visited: HashSet::new(),
    });

    loop {
        let curr = match states.pop_front() {
            Some(state) => state,
            None => break,
        };

        if curr.vols.0 == goal || curr.vols.1 == goal {
            let moves = curr.moves;
            let (goal_bucket, other_bucket) = if goal == curr.vols.0 {
                (Bucket::One, curr.vols.1)
            } else {
                (Bucket::Two, curr.vols.0)
            };
            return Some(BucketStats { goal_bucket, other_bucket, moves })
        }

        let moves = curr.moves + 1;
        let total_vol = curr.vols.0 + curr.vols.1;
        let new_vols = [
            (u8::min(total_vol, cap1), total_vol.saturating_sub(cap1)),
            (total_vol.saturating_sub(cap2), u8::min(total_vol, cap2)),
            (0, curr.vols.1),
            (curr.vols.0, 0),
            (cap1, curr.vols.1),
            (curr.vols.0, cap2),
        ];
        let new_states = new_vols.iter().map(|&vols| {
            BucketState {
                vols,
                moves,
                visited: curr.visited.clone(),
            }
        }).collect::<Vec<_>>();

        for mut state in new_states.into_iter() {
            if state.vols == forbidden_state || curr.visited.contains(&state.vols) {
                continue
            }
            state.visited.insert(state.vols);
            states.push_back(state);
        }
    }

    None
}
