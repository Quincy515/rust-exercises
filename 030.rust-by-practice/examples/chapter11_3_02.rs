
use std::collections::HashMap;
fn main() {
    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    // 使用两种方法实现 team_map2
    // 提示:其中一种方法是使用 `collect` 方法
    let teams_map2: HashMap<_, _> = teams.into_iter().collect();
    let teams_map3 = HashMap::from(teams);

    assert_eq!(teams_map1, teams_map2);
    assert_eq!(teams_map1, teams_map3);

    println!("Success!")
}

