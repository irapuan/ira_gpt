use dialoguer::{theme::ColorfulTheme, MultiSelect};
use good_lp::*;
use is_terminal::IsTerminal;
use std::fs;
use std::io::{self, Read};
use IraGpt::app_error::AppError;
use IraGpt::player::ListOfPlayers;
use IraGpt::player::{self, Team};

/// Load players from file
fn load_players_database(filename: &str) -> Result<ListOfPlayers, AppError> {
    let data = fs::read_to_string(filename)?;
    let list: Team = serde_json::from_str(&data)?;
    Ok(list)
}

/// Save the last selected teams to a file, for future usage.
fn save_cache(selections: &Team, filename: &str) -> Result<(), AppError> {
    let serialized = serde_json::to_string(selections)?;
    fs::write(filename, serialized)?;
    Ok(())
}

/// Load last used list of players
fn load_last_used_list_of_players(filename: &str) -> Result<ListOfPlayers, AppError> {
    let data = fs::read_to_string(filename)?;
    Ok(serde_json::from_str(&data).unwrap_or_default())
}

/// Balance teams based on the criteria
fn balance_teams(
    players: &ListOfPlayers,
    number_of_teams: usize,
    players_per_team: usize,
) -> Result<Vec<Team>, AppError> {
    let mut variables = variables!();
    let team_allocation_variables: Vec<Vec<Variable>> = players
        .iter()
        .map(|_| {
            (0..number_of_teams)
                .map(|_| variables.add(variable().name("team".to_owned()).binary()))
                .collect()
        })
        .collect();

    // println!("team allocation: {:?}", team_allocation_variables);
    const CRITERIA: usize = 6;
    let max_diff: Vec<Variable> = (0..CRITERIA)
        .map(|_| variables.add(variable().min(0.0)))
        .collect();

    let mut lp_problem = variables
        .minimise(max_diff.iter().sum::<Expression>())
        .using(highs);

    for (player_idx, _) in players.iter().enumerate() {
        lp_problem = lp_problem.with(constraint!(
            team_allocation_variables[player_idx]
                .iter()
                .sum::<Expression>()
                == 1
        ));
    }

    for team_idx in 0..number_of_teams {
        let team_size_constraint: Expression = (0..players.len())
            .map(|player_idx| &team_allocation_variables[player_idx][team_idx])
            .sum::<Expression>();
        lp_problem = lp_problem.with(constraint!(team_size_constraint == players_per_team as i32));
    }

    for criteria_idx in 1..CRITERIA {
        let team_scores: Vec<Expression> = (0..number_of_teams)
            .map(|team_idx| {
                (0..players.len())
                    .map(|player_idx| {
                        players[player_idx].qualidades()[criteria_idx] as f64
                            * team_allocation_variables[player_idx][team_idx]
                    })
                    .fold(0.0.into(), |acc, expr| acc + expr)
            })
            .collect();

        let avg_score = team_scores.iter().sum::<Expression>() / number_of_teams as f64;

        for team_idx in 0..number_of_teams {
            lp_problem = lp_problem.with(constraint!(
                max_diff[criteria_idx] >= team_scores[team_idx].clone() - avg_score.clone()
            ));
            lp_problem = lp_problem.with(constraint!(
                max_diff[criteria_idx] >= avg_score.clone() - team_scores[team_idx].clone()
            ));
        }
    }

    let solution = lp_problem.solve()?;

    let mut teams: Vec<Team> = vec![vec![]; number_of_teams];
    for (i, player) in players.iter().enumerate() {
        for j in 0..number_of_teams {
            if solution.value(team_allocation_variables[i][j]) > 0.5 {
                teams[j].push(player.clone());
            }
        }
    }

    Ok(teams)
}

fn print_results(balanced_teams: &Vec<Team>) {
    let colors = ["Preto", "Azul", "Amarelo", "Laranja"];

    for (i, team) in balanced_teams.iter().enumerate() {
        println!("Time {}:", colors[i % colors.len()]);
        for player in team {
            println!("  {}", player.name);
        }
        println!(
            "Media de notas do time: {}",
            player::media_do_jogadores(team)
        );
        println!(
            "  Goleiro: {} - max: {}",
            player::rate_average(team, &player::Criteria::Keeper),
            player::rate_max(team, &player::Criteria::Keeper)
        );
        println!(
            "  Zagueiro: {} - max: {}",
            player::rate_average(team, &player::Criteria::Defender),
            player::rate_max(team, &player::Criteria::Defender)
        );
        println!(
            "  Meio: {} - max: {}",
            player::rate_average(team, &player::Criteria::Midfielder),
            player::rate_max(team, &player::Criteria::Midfielder)
        );
        println!(
            "  Atacante: {} - max: {}",
            player::rate_average(team, &player::Criteria::Forward),
            player::rate_max(team, &player::Criteria::Forward)
        );
        println!(
            "  Velocidade: {} - max: {}",
            player::rate_average(team, &player::Criteria::Speed),
            player::rate_max(team, &player::Criteria::Speed)
        );
        println!(
            "  Stamina: {} - max: {}",
            player::rate_average(team, &player::Criteria::Stamina),
            player::rate_max(team, &player::Criteria::Stamina)
        );
        println!();
    }
    println!(
        "Diferença máxima total entre todos os crtitérios: {}",
        player::total_diference(balanced_teams)
    );

    println!("--------resultado para copiar e colar-------------");
    for (team_idx, team) in balanced_teams.iter().enumerate() {
        println!("Time {}:", colors[team_idx % colors.len()]);
        for player in team {
            println!("{}", player.name);
        }
        println!();
    }
}

fn load_cache_or_stdin(
    players: &Team,
    saved_selections_file: &str,
) -> Result<Vec<bool>, AppError> {
    let saved_selections = load_last_used_list_of_players(saved_selections_file)?;
    let mut stdin_input = String::new();
    if !io::stdin().is_terminal() {
        // Only read if stdin is piped (not interactive terminal)
        io::stdin().read_to_string(&mut stdin_input)?;
    }
    let stdin_players: Vec<String> = stdin_input
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();
    let defaults: Vec<bool> = players
        .iter()
        .map(|player| {
            if !stdin_players.is_empty() {
                stdin_players.contains(&player.name)
            } else {
                saved_selections.contains(player)
            }
        })
        .collect();
    Ok(defaults)
}

fn select_players(
    players: Vec<player::Player>,
    defaults: Vec<bool>,
) -> Result<Vec<player::Player>, AppError> {
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Selecione os jogadores para formar os times")
        .items(&players)
        .defaults(&defaults)
        .interact()?;
    let selected_items: Team = selections
        .iter()
        .map(|&player_idx| players[player_idx].clone())
        .collect();
    Ok(selected_items)
}

fn main() -> Result<(), AppError> {
    // TODO: Add more docs

    let players = load_players_database("players.json")?;
    let saved_selections_file : &'static str = "selections.json";
    let defaults = load_cache_or_stdin(&players, saved_selections_file)?;
    let selected_items = select_players(players, defaults)?;

    save_cache(&selected_items, saved_selections_file)?;

    let players_per_team = 5;
    let number_of_teams = selected_items.len() / players_per_team;
    let balanced_teams = balance_teams(&selected_items, number_of_teams, players_per_team)?;

    print_results(&balanced_teams);

    Ok(())
}
