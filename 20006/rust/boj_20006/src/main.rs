use std::{io::stdin, error::Error};

struct PlayerInfo {
    level: i32,
    name: String
}

impl std::fmt::Display for PlayerInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.level, self.name)
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let (p, m) = {
        let mut buf = String::new();
        stdin().read_line(&mut buf)?;

        let mut buf = buf.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap());
        (buf.next().unwrap(), buf.next().unwrap())
    };

    let mut players: Vec<PlayerInfo>  = Vec::new();
    for _ in 0..p {
        let mut buf = String::new();
        stdin().read_line(&mut buf)?;

        let mut buf = buf.trim().split_whitespace();
        
        let lv = buf.next().unwrap().parse::<i32>()?;
        let name = buf.next().unwrap().to_string();

        players.push(PlayerInfo { level: lv, name: name });
    }

    let mut rooms: Vec<Vec<&PlayerInfo>> = Vec::new();
    for player_idx in 0..p {
        let mut flag = false;
        
        if rooms.is_empty() {
            rooms.push(vec![&players[player_idx]]);
            continue;
        }

        for room_idx in 0..rooms.len() {
            if rooms[room_idx][0].level-10 <= players[player_idx].level && players[player_idx].level <= rooms[room_idx][0].level+10 {
                if rooms[room_idx].len() == m {
                    flag = true;
                    continue;
                }

                rooms[room_idx].push(&players[player_idx]);
                flag = false;
                break;
            } else {
                flag = true;
            }
        }

        if flag {
            rooms.push(vec![&players[player_idx]]);
        }
    }

    for mut room in rooms {
        room.sort_by_key(|&info| &info.name);
        println!("{}", if room.len() == m { "Started!" } else { "Waiting!" });
        for player in room {
            println!("{player}");
        }
    }
    
    Ok(())
}
