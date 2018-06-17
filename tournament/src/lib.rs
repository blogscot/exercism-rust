use std::cmp::Ordering;
use std::collections::HashMap;

static HEADER: &'static str = "Team                           | MP |  W |  D |  L |  P";

enum MatchResult {
  Won,
  Drawn,
  Lost,
}

use MatchResult::*;

#[derive(Clone, PartialEq, PartialOrd, Eq)]
struct Team {
  name: String,
  played: u32,
  won: u32,
  drawn: u32,
  lost: u32,
  score: u32,
}

impl Team {
  fn new(name: &str) -> Self {
    Team {
      name: name.into(),
      played: 0,
      won: 0,
      drawn: 0,
      lost: 0,
      score: 0,
    }
  }

  fn set_match_result(&mut self, result: &MatchResult) {
    self.played += 1;
    match result {
      Won => {
        self.won += 1;
        self.score += 3;
      }
      Drawn => {
        self.drawn += 1;
        self.score += 1;
      }
      Lost => self.lost += 1,
    }
  }
}

impl Ord for Team {
  fn cmp(&self, other: &Team) -> Ordering {
    match self.score.cmp(&other.score) {
      Ordering::Equal => other.name.cmp(&self.name),
      order => order,
    }
  }
}

struct Tournament {
  matches: Vec<String>,
  teams: HashMap<String, Team>,
}

impl Tournament {
  pub fn new(input: &str) -> Self {
    let matches: Vec<&str> = input.split('\n').collect();
    let matches: Vec<String> = matches.iter().map(|&str| str.to_string()).collect();
    Tournament {
      matches,
      teams: HashMap::new(),
    }
  }
  fn parse_matches(&mut self) {
    for match_result in self.matches.clone() {
      self.parse_match_result(&match_result);
    }
  }
  fn parse_match_result(&mut self, input: &str) {
    let details: Vec<_> = input.split(';').collect::<Vec<_>>();
    let (home, away, result) = match *details.as_slice() {
      [home, away, result] => (home, away, result),
      _ => ("", "", ""),
    };
    let mut home_team = if self.teams.contains_key(home) {
      (&self.teams[home]).clone()
    } else {
      Team::new(home)
    };
    let mut away_team = if self.teams.contains_key(away) {
      (&self.teams[away]).clone()
    } else {
      Team::new(away)
    };
    match result {
      "win" => {
        home_team.set_match_result(&Won);
        away_team.set_match_result(&Lost);
      }
      "loss" => {
        home_team.set_match_result(&Lost);
        away_team.set_match_result(&Won);
      }
      "draw" => {
        home_team.set_match_result(&Drawn);
        away_team.set_match_result(&Drawn);
      }
      _ => panic!("Invalid match result found {}", result),
    }

    self.teams.insert(home.to_string(), home_team);
    self.teams.insert(away.to_string(), away_team);
  }

  fn construct_results_table(&mut self) -> String {
    let mut output: Vec<String> = vec![];
    output.push(HEADER.to_string());
    let mut sorted: Vec<_> = self.teams.values().cloned().collect();

    sorted.sort_by(|a, b| b.cmp(&a));

    for team in &sorted {
      output.push(format!(
        "{:<30 } |  {} |  {} |  {} |  {} |  {}",
        team.name, team.played, team.won, team.drawn, team.lost, team.score
      ));
    }
    output.join("\n")
  }
}

pub fn tally(input: &str) -> String {
  if input.is_empty() {
    return HEADER.to_string();
  }
  let mut tournament = Tournament::new(&input);
  tournament.parse_matches();
  tournament.construct_results_table()
}
