use std::{collections::{BinaryHeap, HashMap}, fs, hash::Hash};

pub fn run() {
    let file = fs::read_to_string("./inputs/star_thirty_two.txt").unwrap();
    let (starting_point, maze, w, h)= parse_input(&file);

    let end = maze.iter().find(|t| t.tile_type == 'E').unwrap();
    let result = find_paths_dijkstra(&starting_point, &end.position, &maze, w, h).unwrap_or((-1, 0));
    println!("Result: {}, valid tiles: {}", result.0, result.1);
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug, PartialEq, Clone)]
struct Tile {
    tile_type: char,
    position: Position,
}

fn parse_input(input: &String) -> (Position, Vec<Tile>, isize, isize) {
    let mut tiles = vec![];
    let mut starting_point = Position{x: 0, y: 0};

    let w = input.lines().next().unwrap().len() as isize;
    let h = input.lines().count() as isize;

    let mut input = input.clone();
    input.retain(|s| !s.is_whitespace());

    let chars = input.chars().collect::<Vec<char>>();

    for y in 0..h {
        for x in 0..w {
            let x = x as isize; let y = y as isize;
            let tile_type = chars[(x + (y * w)) as usize];
            if tile_type == 'S' {
                starting_point.x = x;
                starting_point.y = y;
            }
            tiles.push(Tile {
                position: Position { x, y },
                tile_type: tile_type,
            });
        }
    }

    (starting_point, tiles, w, h)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    position: Position,
    direction: usize,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Node {
    cost: isize,
    state: State,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const MOVES: [(isize, isize); 4] = [
    (0, -1), // Up
    (0, 1), // Down
    (-1, 0), // Left
    (1, 0), // Right
];
const TURN_COST: isize = 1000;
const MOVE_COST: isize = 1;

fn find_paths_dijkstra(start: &Position, end: &Position, maze: &Vec<Tile>, w: isize, h: isize) -> Option<(isize, usize)> {
    let mut priority_queue = BinaryHeap::new();
    let mut distances = HashMap::new();
    let start_node = Node {
        cost: 0,
        state: State {
            position: start.clone(),
            direction: 0,
        }
    };
    priority_queue.push(start_node.clone());
    distances.insert(start_node.state.clone(), 0);
    let mut pushed_states = vec![];

    while !priority_queue.is_empty() {
        let current_node = priority_queue.pop().unwrap();
        if current_node.state.position == *end {
            println!("aaaa: {}", pushed_states.len());
            let valid_tiles = count_valid_tiles(&pushed_states, end, start, w, h, current_node.cost);
            return Some((current_node.cost, valid_tiles));
        }

        let found_distance = distances.get(&current_node.state);
        if let Some(distance) = found_distance {
            if current_node.cost > *distance {
                continue;
            }
        }

        do_move(maze, &mut distances, &mut priority_queue, &current_node, current_node.state.direction, w, start, &mut pushed_states);

        for (direction, _) in MOVES.to_vec().iter().enumerate() {
            do_move(maze, &mut distances, &mut priority_queue, &current_node, direction, w, start, &mut pushed_states);
        }
    }

    None
}

fn do_move(
    maze: &Vec<Tile>,
    distances: &mut HashMap<State, isize>,
    priority_queue: &mut BinaryHeap<Node>,
    current_node: &Node,
    direction: usize,
    w: isize,
    start: &Position,
    pushed_states: &mut Vec<Node>,
) {
    let is_backwards = (current_node.state.direction == 0 && direction == 1)
        || (current_node.state.direction == 2 && direction == 3)
        || (current_node.state.direction == 1 && direction == 0)
        || (current_node.state.direction == 3 && direction == 2);

    if is_backwards {
        return;
    }

    let m = MOVES[direction];
    let new_position = Position {
        x: current_node.state.position.x + m.0,
        y: current_node.state.position.y + m.1,
    };
    let tile = &maze[(new_position.x + (new_position.y * w)) as usize];
    if tile.tile_type == '#' {
        return;
    }

    let move_cost = match current_node.state.direction != direction || current_node.state.position == *start{
        true => current_node.cost + MOVE_COST + TURN_COST,
        false => current_node.cost + MOVE_COST,
    };

    let new_state = State {
        position: new_position.clone(),
        direction: direction,
    };

    let found_distance = distances.get(&new_state);
    if found_distance.is_none() || move_cost < *found_distance.unwrap() {
        let new_node = Node {
            cost: move_cost,
            state: new_state.clone(),
        };
        if !pushed_states.iter().find(|s| s.state.position == new_state.position && s.cost <= new_node.cost).is_some() {
            pushed_states.push(new_node.clone());
        }
        priority_queue.push(Node {
            cost: move_cost,
            state: new_state.clone(),
        });
        distances.insert(new_state, move_cost);
    }

}

fn count_valid_tiles(found_tiles: &Vec<Node>, start: &Position, end: &Position, w: isize, h: isize, expected_score: isize) -> usize {
    let mut lookup_map = HashMap::new();
    for t in found_tiles {
        lookup_map.insert((t.state.position.x, t.state.position.y), t);
    }
    let mut new_maze = vec![];
    for y in 0..h {
        for x in 0..w {
            let mut c = match lookup_map.get(&(x, y)).is_some() {
                true => '.',
                false => '#',
            };
            if x == start.x && y == end.y {
                c = 'S';
            }
            if x == end.x && y == end.y {
                c = 'E';
            }
            new_maze.push(Tile {
                tile_type: c,
                position: Position {
                    x, y
                }
            });
        }
    }

    let mut new_found_paths = vec![];
    println!("finding paths...");
    find_paths(start, &String::new(), &HashMap::new(), &Position{x: -1, y: -1}, &new_maze, w, h, &mut new_found_paths);
    println!("finding reachable paths...");
    let new_found_paths = get_end_reachable_paths(&new_found_paths, expected_score);

    let mut unique_tiles = HashMap::new();

    println!("finding unique tiles...");
    for path in new_found_paths {
        let mut current_pos = Position{x: 0, y: 0};
        unique_tiles.insert(current_pos, true);
        for c in path.chars() {
            let direction = map_route_to_direction(c);
            current_pos.x += direction.x;
            current_pos.y += direction.y;
            unique_tiles.insert(current_pos, true);
        }
    }

    unique_tiles.len()
}

fn get_end_reachable_paths(paths: &Vec<String>, expected_score: isize) -> Vec<String> {
    let mut new_paths = vec![];
    for p in paths {
        let turns = count_turns(&p);
        let score = p.len() + (turns * 1000);
        if score == expected_score as usize {
            new_paths.push(p.clone());
        }
    }

    new_paths
}

fn count_turns(route: &String) -> usize {
    let mut last_char = ' ';
    let mut is_first = true;
    let mut count = 0;
    for c in route.chars() {
        if is_first {
            last_char = c;
            is_first = false;
            continue;
        }
        if c != last_char {
            count += 1;
        }
        last_char = c;
    }

    count + 1
}

fn map_route_to_direction(route: char) -> Position {
    match route {
        '^' => Position{x: 0, y: -1},
        'v' => Position{x: 0, y: 1},
        '<' => Position{x: -1, y: 0},
        '>' => Position{x: 1, y: 0},
        _ => unreachable!(),
    }
}

const DIRECTIONS: [Position; 4] = [
    Position{x: 1, y: 0}, // Right
    Position{x: -1, y: 0}, // Left
    Position{x: 0, y: -1}, // Up
    Position{x: 0, y: 1}, // Down
];

fn find_paths(
    current_position: &Position,
    current_route: &String,
    visited_tiles: &HashMap<Position, bool>,
    previous_position: &Position,
    maze: &Vec<Tile>,
    w: isize,
    h: isize,
    found_paths: &mut Vec<String>
) {
    // Are we in a loop?
    if visited_tiles.get(&current_position).is_some() {
        return;
    }
    let current_tile = &maze[(current_position.x + (current_position.y * w)) as usize];
    if current_tile.tile_type == 'E' {
        found_paths.push(current_route.clone());
        return;
    }

    for direction in DIRECTIONS {
        if !can_move(current_position, &direction, previous_position, maze, w, h) {
            continue;
        }

        let new_position = Position {
            x: current_position.x + direction.x,
            y: current_position.y + direction.y,
        };

        // Probably an available space so keep looking
        let new_route = format!("{}{}", current_route, map_direction_to_route((direction.x, direction.y)));
        let mut new_visited_tiles = visited_tiles.clone();
        new_visited_tiles.insert(current_position.clone(), true);
        find_paths(
            &new_position,
            &new_route,
            &new_visited_tiles,
            current_position,
            maze, w, h,
            found_paths,
        );
    }
}

fn map_direction_to_route(direction: (isize, isize)) -> String {
    match direction {
        (0, -1) => String::from("^"),
        (0, 1)  => String::from("v"),
        (-1, 0) => String::from("<"),
        (1, 0)  => String::from(">"),
        _ => unreachable!(),
    }
}

fn can_move(current_position: &Position, direction: &Position, previous_position: &Position, maze: &Vec<Tile>, w: isize, h: isize) -> bool {
    let new_position = Position {
        x: current_position.x + direction.x,
        y: current_position.y + direction.y,
    };

    // Don't go backwards
    if new_position == *previous_position {
        return false;
    }
    // Don't go to invalid locations (unlikely to happen given the input but just in case)
    if new_position.x < 0 || new_position.y < 0 || new_position.x >= w || new_position.y >= h {
        return false;
    }

    let x = new_position.x;
    let y = new_position.y;
    let next_tile = &maze[(x + (y * w)) as usize];

    // let next_tile = maze.iter().find(|t| t.position == new_position).unwrap();

    // Are we about to hit a wall?
    if next_tile.tile_type == '#' {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = vec![
            "#.#",
            ".SE",
        ].join("\n");
        let result = parse_input(&input);
        assert_eq!(
            result,
            (
                Position{x: 1, y: 1},
                vec![
                    Tile{
                        tile_type: '#',
                        position: Position{x: 0, y: 0},
                    },
                    Tile{
                        tile_type: '.',
                        position: Position{x: 1, y: 0},
                    },
                    Tile{
                        tile_type: '#',
                        position: Position{x: 2, y: 0},
                    },
                    Tile{
                        tile_type: '.',
                        position: Position{x: 0, y: 1},
                    },
                    Tile{
                        tile_type: 'S',
                        position: Position{x: 1, y: 1},
                    },
                    Tile{
                        tile_type: 'E',
                        position: Position{x: 2, y: 1},
                    },
                ],
                3, 2
            )
        );
    }

    #[test]
    fn test_get_lowest_score_dijkstra() {
        let input = vec![
            "###############",
            "#.......#....E#",
            "#.#.###.#.###.#",
            "#.....#.#...#.#",
            "#.###.#####.#.#",
            "#.#.#.......#.#",
            "#.#.#####.###.#",
            "#...........#.#",
            "###.#.#####.#.#",
            "#...#.....#.#.#",
            "#.#.#.###.#.#.#",
            "#.....#...#.#.#",
            "#.###.#.#.#.#.#",
            "#S..#.....#...#",
            "###############",
        ].join("\n");
        let (starting_point, maze, w, h)= parse_input(&input);
        let end = maze.iter().find(|t| t.tile_type == 'E').unwrap();
        let result = find_paths_dijkstra(&starting_point, &end.position, &maze, w, h);

        assert_eq!(result, Some((7036, 45)));

        let input = vec![
            "#################",
            "#...#...#...#..E#",
            "#.#.#.#.#.#.#.#.#",
            "#.#.#.#...#...#.#",
            "#.#.#.#.###.#.#.#",
            "#...#.#.#.....#.#",
            "#.#.#.#.#.#####.#",
            "#.#...#.#.#.....#",
            "#.#.#####.#.###.#",
            "#.#.#.......#...#",
            "#.#.###.#####.###",
            "#.#.#...#.....#.#",
            "#.#.#.#####.###.#",
            "#.#.#.........#.#",
            "#.#.#.#########.#",
            "#S#.............#",
            "#################",
        ].join("\n");
        let (starting_point, maze, w, h)= parse_input(&input);
        let end = maze.iter().find(|t| t.tile_type == 'E').unwrap();
        let result = find_paths_dijkstra(&starting_point, &end.position, &maze, w, h);

        assert_eq!(result, Some((11048, 64)));
    }
}
