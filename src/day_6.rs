use crate::util;

// Direction: [left, down, up, right] (vim binds)
const GUARD: [char; 4] = ['<', 'v', '^', '>'];
const WALL: char = '#';

pub async fn answer() -> Result<(u32, u32), anyhow::Error> {
    let mut input = groom(util::get_input(6).await?);
    // let mut input = groom("....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...".to_owned());
    let guard_pos = find_guard(&input).unwrap();
    find_visited_squares(&mut input, guard_pos);
    let part_1 = count_visited(input);
    Ok((part_1 as u32, 1))
}

fn groom(input: String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_guard(input: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .find(|(_, &ref inner_vec)| {
            inner_vec.contains(&GUARD[0])
                | inner_vec.contains(&GUARD[1])
                | inner_vec.contains(&GUARD[2])
                | inner_vec.contains(&GUARD[3])
        })
        .map(|(index, &ref inner_vec)| {
            (
                index,
                inner_vec.iter().position(|r| GUARD.contains(r)).unwrap(),
            )
        })
}

fn find_visited_squares(input: &mut Vec<Vec<char>>, initial_pos: (usize, usize)) {
    if let Some(new_pos) = move_guard(input, initial_pos) {
        find_visited_squares(input, new_pos);
    }
}

fn move_guard(input: &mut Vec<Vec<char>>, pos: (usize, usize)) -> Option<(usize, usize)> {
    let direction = GUARD.iter().position(|&r| r == input[pos.0][pos.1])?;

    match direction {
        0 => {
            if pos.1 == 0 {
                input[pos.0][pos.1] = 'X';
                None
            } else if input[pos.0][pos.1 - 1] == WALL {
                input[pos.0][pos.1] = GUARD[2];
                Some(pos)
            } else {
                input[pos.0][pos.1] = 'X';
                input[pos.0][pos.1 - 1] = GUARD[0];
                Some((pos.0, pos.1 - 1))
            }
        }
        1 => {
            if pos.0 == input.len() - 1 {
                input[pos.0][pos.1] = 'X';
                None
            } else if input[pos.0 + 1][pos.1] == WALL {
                input[pos.0][pos.1] = GUARD[0];
                Some(pos)
            } else {
                input[pos.0][pos.1] = 'X';
                input[pos.0 + 1][pos.1] = GUARD[1];
                Some((pos.0 + 1, pos.1))
            }
        }
        2 => {
            if pos.0 == 0 {
                input[pos.0][pos.1] = 'X';
                None
            } else if input[pos.0 - 1][pos.1] == WALL {
                input[pos.0][pos.1] = GUARD[3];
                Some(pos)
            } else {
                input[pos.0][pos.1] = 'X';
                input[pos.0 - 1][pos.1] = GUARD[2];
                Some((pos.0 - 1, pos.1))
            }
        }
        3 => {
            if pos.1 == input[pos.0].len() - 1 {
                input[pos.0][pos.1] = 'X';
                None
            } else if input[pos.0][pos.1 + 1] == WALL {
                input[pos.0][pos.1] = GUARD[1];
                Some(pos)
            } else {
                input[pos.0][pos.1] = 'X';
                input[pos.0][pos.1 + 1] = GUARD[3];
                Some((pos.0, pos.1 + 1))
            }
        }
        _ => None,
    }
}

fn count_visited(map: Vec<Vec<char>>) -> usize {
    map.into_iter().flatten().filter(|&n| n == 'X').count()
}
