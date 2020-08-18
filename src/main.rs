use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use std::collections::HashMap;

struct Player {
    id: u32,
    name: &'static str,
    group_id: u32
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player: {} ({}) - {}", self.name, self.id, self.group_id)
    }
}

fn get_player_from_id(players:&Vec<Player>, player_id: u32) -> Option<usize> {

    for (i, player) in players.iter().enumerate() {
        if player.id == player_id {
            return Some(i);
        }
    }
    None
}

fn get_available_player_pairings(
    players: &Vec<Player>,
    matchings: &HashMap<u32, u32>,
    current_group: u32) -> Vec<u32> {
    let mut pairings = Vec::new();

    for player in players {
        print!("{}", player);
        if player.group_id == current_group {
            print!(" Same Group\n");
            continue;
        }
        match matchings.get(&player.id) { 
            None => {
                pairings.push(player.id);
            },
            Some(i) => {
                print!(" {}\n", i);
                continue;
            }
        }
    }

    pairings.shuffle(&mut thread_rng());

    pairings
}

fn match_players(
    players: &mut Vec<Player>,
    matchings: &mut HashMap<u32, u32>,
    current_player: usize) -> bool {

    let Player {id, group_id, ..} = players[current_player];
    let available_pairings = get_available_player_pairings(players, matchings, group_id);

    if available_pairings.len() == 0 {
        println!("size to small, restarting");
        return false;
    }

    for available_pair in &available_pairings {
        print!(" {}", available_pair);
    }
    print!("\n");
    for available_pair in available_pairings {

        // println!("Inserting {} as {}", available_pair, id);
        matchings.insert(available_pair, id);

        if current_player + 1 == players.len() {
            return true;
        }

        if match_players(players, matchings, current_player + 1) {
            return true;
        }
        print!("match failed on {} in group {}", id, group_id)
    }
    print!("\n");

    false
}


fn main(){
    let mut players = Vec::new();
    let mut matchings = HashMap::new();

    let ben = Player {id: 1, name: "Ben", group_id: 1};
    let andrew = Player {id: 2, name: "Andrew", group_id: 1};
    let nick = Player {id: 3, name: "Nick", group_id: 2};
    let collin = Player {id: 4, name: "Collin", group_id: 2};
    let zoe = Player {id: 5, name: "Zoe", group_id: 3};
    let emily = Player {id: 6, name: "Emily", group_id: 3};
    let hayden = Player {id: 7, name: "Hayden", group_id: 4};
    let colton = Player {id: 8, name: "Colton", group_id: 4};

    players.push(ben);
    players.push(andrew);
    players.push(nick);
    players.push(collin);
    players.push(zoe);
    players.push(emily);
    // players.push(hayden);
    // players.push(colton);

    if match_players(&mut players, &mut matchings, 0) {
        println!("Sorted!");
    }
    else {
        println!("Failed to sort :(");
    }

    for player in &players {
        let player_match = match matchings.get(&player.id) {
            None => panic!("Couldn't find anyone"),
            Some(i) => i
        };

        let player_match = match get_player_from_id(&players, *player_match) {
            None => panic!("Couldn't find anyone"),
            Some(i) => &players[i],
        };

        println!("{} matched with {}", player, player_match);
    }
}
