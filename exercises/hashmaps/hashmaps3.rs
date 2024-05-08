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

// A structure to store the goal details of a team.
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO: Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be the number of goals conceded from team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by
        // team_1.

        /*
        对于每场比赛，首先检查scores哈希表中是否已经存在该球队。
        如果存在，则更新该球队的得分和失分；
        如果不存在，则创建一个新的Team结构体，并将其插入到scores哈希表中。
         */
        if scores.contains_key(&team_1_name) {
            let team = scores.get_mut(&team_1_name).unwrap();
            team.goals_scored += team_1_score;
            team.goals_conceded += team_2_score;
        } else {
            let team = Team {
                goals_scored: team_1_score,
                goals_conceded: team_2_score,
            };
            scores.insert(team_1_name, team);
        }
        if scores.contains_key(&team_2_name) {
            let team = scores.get_mut(&team_2_name).unwrap();
            team.goals_scored += team_2_score;
            team.goals_conceded += team_1_score;
        } else {
            let team = Team {
                goals_scored: team_2_score,
                goals_conceded: team_1_score,
            };
            scores.insert(team_2_name, team);
        }
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}

/*
这段代码是一个Rust程序，用于处理足球比赛的结果，并生成一个得分表。得分表是一个HashMap，其中键是球队的名字，值是一个包含进球数和失球数的结构体。

首先，我们定义了一个名为`Team`的结构体，用于存储球队的进球数和失球数。然后，我们实现了一个名为`build_scores_table`的函数，该函数接受一个包含比赛结果的字符串，并返回一个得分表。

在`build_scores_table`函数中，我们首先创建了一个空的得分表。然后，我们遍历输入字符串中的每一行，将每行的球队名字和进球数分别提取出来。接下来，我们将进球数添加到球队的进球数中，并将失球数添加到球队的失球数中。最后，我们将球队的名字和进球数失球数插入得分表中。

在`tests`模块中，我们编写了一些测试用例来验证`build_scores_table`函数的正确性。这些测试用例首先创建了一个包含比赛结果的字符串，然后调用`build_scores_table`函数生成得分表，最后检查得分表中的数据是否符合预期。



*/
