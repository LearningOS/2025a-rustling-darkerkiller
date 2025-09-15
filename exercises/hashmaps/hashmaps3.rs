// hashmaps3.rs
//
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: England,France,4,2 (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, goals the
// team scored, and goals the team conceded. One approach to build the scores
// table is to use a Hashmap. The solution is partially written to use a
// Hashmap, complete it to pass the test.
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.

use std::collections::HashMap;

#[derive(Default)]
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    let mut scores: HashMap<String, Team> = HashMap::new();

    for line in results.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        let t1_name = parts[0].to_string();
        let t2_name = parts[1].to_string();
        let t1_score: u8 = parts[2].parse().unwrap();
        let t2_score: u8 = parts[3].parse().unwrap();

        // 更新 team1
        let t1 = scores.entry(t1_name).or_default();
        t1.goals_scored += t1_score;
        t1.goals_conceded += t2_score;

        // 更新 team2
        let t2 = scores.entry(t2_name).or_default();
        t2.goals_scored += t2_score;
        t2.goals_conceded += t1_score;
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        ["England,France,4,2",
         "France,Italy,3,1",
         "Poland,Spain,2,0",
         "Germany,England,2,1"]
        .join("\n")
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());
        let mut keys: Vec<_> = scores.keys().collect();
        keys.sort();
        assert_eq!(keys, vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]);
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let eng = scores.get("England").unwrap();
        assert_eq!(eng.goals_scored, 5);
        assert_eq!(eng.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let esp = scores.get("Spain").unwrap();
        assert_eq!(esp.goals_scored, 0);
        assert_eq!(esp.goals_conceded, 2);
    }
}
