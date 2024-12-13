use std::{collections::HashMap, fs};

pub fn run() {
    let file = fs::read_to_string("./inputs/star_twenty_three.txt").unwrap();
    let (grid, w, h) = parse_input(&file);
    let stats = get_garden_stats(&grid, w, h);
    let result = calculate_price(&stats);

    println!("Result: {}", result);
}

const DIRECTIONS: [(isize, isize); 4] = [
    (0, -1), // Up
    (0, 1), // Down
    (-1, 0), // Left
    (1, 0), // Right
];

pub fn parse_input(input: &String) -> (Vec<char>, isize, isize) {

    let w = input.lines().next().unwrap().len();
    let h = input.lines().count();

    let mut input = input.clone();
    input.retain(|s| !s.is_whitespace());

    (input.chars().collect::<Vec<char>>(), w as isize, h as isize)
}

fn calculate_price(stats: &Vec<GardenPlotStat>) -> usize {
    stats.iter()
        .map(|s| s.area * s.perimeter)
        .sum()
}


#[derive(Copy, Clone, PartialEq, Debug)]
struct GardenPlotStat {
    plant: char,
    area: usize,
    perimeter: usize,
}

fn get_garden_stats(grid: &Vec<char>, w: isize, h: isize) -> Vec<GardenPlotStat> {
    let mut garden_plot_stats = vec![];
    let mut checked_locations: HashMap<(isize, isize), bool> = HashMap::new();
    for y in 0..h {
        for x in 0..w {
            let plant = grid[(x + (y * h)) as usize];
            if checked_locations.get(&(x, y)).is_some() {
                continue;
            }
            garden_plot_stats.push(get_plot_stats(plant, grid, (x, y), &mut checked_locations, w, h));
        }
    }

    garden_plot_stats
}

fn get_plot_stats(plant: char, grid: &Vec<char>, current_location: (isize, isize), checked_locations: &mut HashMap<(isize, isize), bool>, w: isize, h: isize) -> GardenPlotStat {
    if checked_locations.get(&current_location).is_some() {
        return GardenPlotStat {plant: plant, area: 0, perimeter: 0};
    }
    let mut stats = GardenPlotStat {plant: plant, area: 0, perimeter: 0};
    let neighbor_x = current_location.0 as usize;
    let neighbor_y = current_location.1 as usize;
    let neighbor_plant = grid[neighbor_x + (neighbor_y * h as usize)];
    if neighbor_plant == plant {
        stats.area = 1;
        stats.perimeter = get_plot_perimeter(plant, grid, current_location.0, current_location.1, w, h);
        checked_locations.insert(current_location, true);
    } else {
        return GardenPlotStat {plant: plant, area: 0, perimeter: 0};
    }


    let skip_directions = get_skip_directions(current_location.0, current_location.1, w, h);
    for direction in DIRECTIONS {
        if skip_directions.contains(&direction) {
            continue;
        }
        let new_location = (current_location.0 + direction.0, current_location.1 + direction.1);
        let found_stats = get_plot_stats(plant, grid, new_location, checked_locations, w, h);
        stats.area += found_stats.area;
        stats.perimeter += found_stats.perimeter;
    }

    stats
}

fn get_plot_perimeter(plant: char, grid: &Vec<char>, x: isize, y: isize, w: isize, h: isize) -> usize {
    let mut perimeter = 0;
    let skip_directions = get_skip_directions(x, y, w, h);

    for direction in DIRECTIONS {
        if skip_directions.contains(&direction) {
            perimeter += 1;
            continue;
        }
        let neighbor_x = (x + direction.0) as usize;
        let neighbor_y = (y + direction.1) as usize;
        let neighbor_plant = grid[neighbor_x + (neighbor_y * h as usize)];
        if neighbor_plant != plant {
            perimeter += 1;
        }
    }

    perimeter
}


fn get_skip_directions(x: isize, y: isize, w: isize, h: isize) -> Vec<(isize, isize)> {
    let mut skip_directions = vec![];
    if x == 0 {
        skip_directions.push((-1, 0));
    }
    if y == 0 {
        skip_directions.push((0, -1));
    }
    if x == w - 1 {
        skip_directions.push((1, 0));
    }
    if y == h - 1 {
        skip_directions.push((0, 1));
    }
    skip_directions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_plot_perimeter() {
        let input = vec![
            "AAAA",
            "BBCD",
            "BBCC",
            "EEEC"
        ].join("\n");
        let (grid, w, h) = parse_input(&input);
        assert_eq!(
            get_plot_perimeter('A', &grid, 0, 0, w, h),
            3,
        );
        assert_eq!(
            get_plot_perimeter('A', &grid, 1, 0, w, h),
            2,
        );
        assert_eq!(
            get_plot_perimeter('A', &grid, 2, 0, w, h),
            2,
        );
        assert_eq!(
            get_plot_perimeter('A', &grid, 3, 0, w, h),
            3,
        );
        assert_eq!(
            get_plot_perimeter('C', &grid, 3, 3, w, h),
            3,
        );
        assert_eq!(
            get_plot_perimeter('E', &grid, 0, 3, w, h),
            3,
        );
    }

    #[test]
    fn test_get_garden_stats() {
        let input = vec![
            "AAAA",
            "BBCD",
            "BBCC",
            "EEEC"
        ].join("\n");
        let (grid, w, h) = parse_input(&input);
        let mut result = get_garden_stats(&grid, w, h);
        result.sort_by(|a, b| a.plant.cmp(&b.plant));
        assert_eq!(
            result,
            [
                GardenPlotStat {
                    plant: 'A',
                    area: 4,
                    perimeter: 10,
                },
                GardenPlotStat {
                    plant: 'B',
                    area: 4,
                    perimeter: 8,
                },
                GardenPlotStat {
                    plant: 'C',
                    area: 4,
                    perimeter: 10,
                },
                GardenPlotStat {
                    plant: 'D',
                    area: 1,
                    perimeter: 4,
                },
                GardenPlotStat {
                    plant: 'E',
                    area: 3,
                    perimeter: 8,
                },
            ],
        );

        let input = vec![
            "OOOOO",
            "OXOXO",
            "OOOOO",
            "OXOXO",
            "OOOOO"
        ].join("\n");
        let (grid, w, h) = parse_input(&input);
        let mut result = get_garden_stats(&grid, w, h);
        result.sort_by(|a, b| a.plant.cmp(&b.plant));
        assert_eq!(
            result,
            [
                GardenPlotStat {
                    plant: 'O',
                    area: 21,
                    perimeter: 36,
                },
                GardenPlotStat {
                    plant: 'X',
                    area: 1,
                    perimeter: 4,
                },
                GardenPlotStat {
                    plant: 'X',
                    area: 1,
                    perimeter: 4,
                },
                GardenPlotStat {
                    plant: 'X',
                    area: 1,
                    perimeter: 4,
                },
                GardenPlotStat {
                    plant: 'X',
                    area: 1,
                    perimeter: 4,
                },
            ],
        );
    }

    #[test]
    fn test_calculate_price() {
        let input = vec![
            "AAAA",
            "BBCD",
            "BBCC",
            "EEEC"
        ].join("\n");
        let (grid, w, h) = parse_input(&input);
        let stats = get_garden_stats(&grid, w, h);
        assert_eq!(calculate_price(&stats), 140);

        let input = vec![
            "OOOOO",
            "OXOXO",
            "OOOOO",
            "OXOXO",
            "OOOOO"
        ].join("\n");
        let (grid, w, h) = parse_input(&input);
        let stats = get_garden_stats(&grid, w, h);
        assert_eq!(calculate_price(&stats), 772);

        let input = vec![
            "RRRRIICCFF",
            "RRRRIICCCF",
            "VVRRRCCFFF",
            "VVRCCCJFFF",
            "VVVVCJJCFE",
            "VVIVCCJJEE",
            "VVIIICJJEE",
            "MIIIIIJJEE",
            "MIIISIJEEE",
            "MMMISSJEEE",
        ].join("\n");
        let (grid, w, h) = parse_input(&input);
        let stats = get_garden_stats(&grid, w, h);
        assert_eq!(calculate_price(&stats), 1930);
    }
}
