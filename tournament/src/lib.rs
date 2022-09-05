use std::{collections::HashMap, vec};

#[derive(Debug, Default)]
struct Team {
    name: String,
    MP: u8,
    W: u8,
    D: u8,
    L: u8,
    P: u8,
}

enum MatchResult {
    Win,
    Loss,
    Draw,
}

impl MatchResult {
    fn reserve(&self) -> Self {
        match self {
            MatchResult::Loss => MatchResult::Win,
            MatchResult::Win => MatchResult::Loss,
            MatchResult::Draw => MatchResult::Draw,
        }
    }
}

impl From<&str> for MatchResult {
    fn from(result: &str) -> MatchResult {
        match result {
            "win" => MatchResult::Win,
            "loss" => MatchResult::Loss,
            _ => MatchResult::Draw,
        }
    }
}

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";

impl From<&Team> for String {
    fn from(team: &Team) -> String {
        format!(
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            team.name, team.MP, team.W, team.D, team.L, team.P
        )
    }
}

impl Team {
    pub fn new(name: String) -> Team {
        Team {
            name,
            ..Default::default()
        }
    }

    pub fn win(&mut self) {
        self.MP += 1;
        self.W += 1;
        self.P += 3;
    }

    pub fn draw(&mut self) {
        self.MP += 1;
        self.D += 1;
        self.P += 1;
    }

    pub fn loss(&mut self) {
        self.MP += 1;
        self.L += 1;
    }

    pub fn add_match(&mut self, result: &MatchResult) {
        match result {
            MatchResult::Win => self.win(),
            MatchResult::Draw => self.draw(),
            MatchResult::Loss => self.loss(),
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let mut scores: HashMap<String, Team> = HashMap::new(); // name->Team
    match_results.lines().for_each(|line| {
        let arr: Vec<&str> = line.split(";").collect();
        let p1 = arr[0];
        let p2 = arr[1];
        let result = arr[2].into();

        scores
            .entry(p1.into())
            .or_insert(Team::new(p1.into()))
            .add_match(&result);
        scores
            .entry(p2.into())
            .or_insert(Team::new(p2.into()))
            .add_match(&result.reserve());
    });
    use std::cmp::Ordering;

    // println!("{:#?}", scores);

    let mut score_values: Vec<&Team> = scores.values().collect();
    // score_values.sort_by(|a, b| {
    //     if a.P > b.P {
    //         Ordering::Less
    //     } else if a.P == b.P {
    //         if a.name < b.name {
    //             Ordering::Less
    //         } else {
    //             Ordering::Greater
    //         }
    //     } else {
    //         Ordering::Greater
    //     }
    // });
    // println!("{:#?}", score_values);

    score_values.sort_by(|a,b| b.P.cmp(&a.P).then(a.name.cmp(&b.name)));

    vec![String::from(HEADER)]
        .into_iter()
        .chain(score_values.into_iter().map(|t| t.into()))
        .collect::<Vec<String>>()
        .join("\n")
}
