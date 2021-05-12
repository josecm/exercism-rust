use std::collections::BTreeMap;
use std::{fmt, ops};

#[derive(PartialEq, Copy, Clone)]
enum MatchResult {
    Win,
    Loss,
    Draw,
}

impl MatchResult {
    fn parse(r: &str) -> Option<Self> {
        match r {
            "win" => Some(Self::Win),
            "loss" => Some(Self::Loss),
            "draw" => Some(Self::Draw),
            _ => None
        }
    }

    fn points(&self) -> usize {
        match self {
            Self::Win => 3,
            Self::Draw => 1,
            Self::Loss => 0,
        }
    }
}

impl ops::Not for MatchResult {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Win => Self::Loss,
            Self::Loss => Self::Win,
            _ => Self::Draw,
        }
    }
}

struct TeamStats {
    name: String,
    results: Vec<MatchResult> 
}

impl TeamStats {
    fn new(name: &str) -> Self {
        TeamStats {
            name: name.to_string(),
            results: Vec::new(),
        }
    }

    fn played(&self) -> usize {
        self.results.len()
    }

    fn count_result(&self, mr: MatchResult) -> usize {
        self.results.iter().filter(|&r| *r == mr).count()
    }

    fn points(&self) -> usize {
        self.results.iter().map(|r| r.points()).sum()
    }

    fn update(&mut self, r: MatchResult) {
        self.results.push(r);
    }

    const HEADER: &'static str = "Team                           | MP |  W |  D |  L |  P";
}

impl fmt::Display for TeamStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:31}|{:3} |{:3} |{:3} |{:3} |{:3}",
            self.name,
            self.played(),
            self.count_result(MatchResult::Win),
            self.count_result(MatchResult::Draw),
            self.count_result(MatchResult::Loss),
            self.points()
        )
    }
}

pub fn tally(match_results: &str) -> String {
    let mut stats = BTreeMap::<&str, TeamStats>::new();

    for m in match_results.lines() {
        if let [t1, t2, mr] = (m.split(';').collect::<Vec<_>>())[..3] {
            match MatchResult::parse(mr) {
                Some(r) => {
                    stats.entry(t1).or_insert(TeamStats::new(t1)).update(r);
                    stats.entry(t2).or_insert(TeamStats::new(t2)).update(!r);
                },
                None => panic!("bad formated result {}", mr),
            }
        } else {
            panic!("bad line format!");
        }
    }

    let mut table = stats.values().collect::<Vec<_>>();
    table.sort_by(|&a, &b| b.points().cmp(&a.points()));

    std::iter::once(TeamStats::HEADER.to_string())
        .chain(table.iter().map(|t| t.to_string()))
        .collect::<Vec<_>>()
        .join("\n")
}
