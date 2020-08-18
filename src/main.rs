use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use std::collections::HashMap;

struct Player {
    name: &'static str,
    group_id: u32
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player: {} ({})", self.name, self.group_id)
    }
}

fn get_available_player_pairings(
    players: &Vec<Player>,
    matchings: &HashMap<usize, usize>,
    current_group: u32) -> Vec<usize> {
    let mut pairings = Vec::new();

    for (i, player) in players.iter().enumerate() {
        if player.group_id == current_group {
            continue;
        }
        match matchings.get(&i) { 
            None => {
                pairings.push(i);
            },
            _ => {
                continue;
            }
        }
    }

    pairings.shuffle(&mut thread_rng());

    pairings
}

fn match_players(
    players: &mut Vec<Player>,
    matchings: &mut HashMap<usize, usize>,
    current_player: usize) -> bool {

    let Player {group_id, ..} = players[current_player];
    let available_pairings = get_available_player_pairings(players, matchings, group_id);

    if available_pairings.len() == 0 {
        return false;
    }

    for available_pair in &available_pairings {
        print!(" {}", available_pair);
    }
    print!("\n");
    for available_pair in available_pairings {
        matchings.insert(available_pair, current_player);

        if current_player + 1 == players.len() {
            return true;
        }
        else if match_players(players, matchings, current_player + 1) {
            return true;
        }
        else {
            matchings.remove(&available_pair);
        }
    }

    false
}


fn main(){
    let mut players = Vec::new();
    let mut matchings = HashMap::new();

    let ben = Player {name: "Ben", group_id: 1};
    let andrew = Player {name: "Andrew", group_id: 1};
    let nick = Player {name: "Nick", group_id: 2};
    let collin = Player {name: "Collin", group_id: 2};
    let zoe = Player {name: "Zoe", group_id: 3};
    let emily = Player {name: "Emily", group_id: 3};
    let hayden = Player {name: "Hayden", group_id: 4};
    let colton = Player {name: "Colton", group_id: 4};

    players.push(ben);
    players.push(andrew);
    players.push(nick);
    players.push(collin);
    players.push(zoe);
    players.push(emily);
    players.push(hayden);
    players.push(colton);

    if match_players(&mut players, &mut matchings, 0) {
        println!("Sorted!");
    }
    else {
        println!("Failed to sort :(");
    }

    for (i, player) in players.iter().enumerate() {
        let player_match = match matchings.get(&i) {
            None => panic!("Couldn't find anyone"),
            Some(i) => i
        };

        println!("{} matched with {}", player, &players[*player_match]);
    }
}
