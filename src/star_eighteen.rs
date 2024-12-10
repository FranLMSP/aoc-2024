use std::fs;

pub fn run() {
    let file = fs::read_to_string("./inputs/star_eighteen.txt").unwrap();
    let unpacked_fs = unpack_filesystem(&file);
    let defragged_fs = defrag_filesystem(&unpacked_fs);
    let checksum = calculate_checksum(&defragged_fs);

    println!("Result: {}", checksum);
}

fn unpack_filesystem(filesystem: &String) -> Vec<(usize, Option<usize>)> {
    let mut new_fs = vec![];
    let mut is_white_space = false;
    let mut index = 0;
    for current_char in filesystem.chars() {
        let size = current_char.to_string().parse::<usize>().unwrap();
        let char_to_insert = match is_white_space {
            true => None,
            false => Some(index),
        };

        if size > 0 {
            new_fs.push((size, char_to_insert));
        }

        if !is_white_space && size > 0 {
            index += 1;
        }

        is_white_space = !is_white_space;
    }

    new_fs
}

fn defrag_filesystem(unpacked_filesystem: &Vec<(usize, Option<usize>)>) -> Vec<Option<usize>> {
    let mut x = 0;
    let mut y = unpacked_filesystem.len() - 1;
    let mut new_unpacked_filesystem = unpacked_filesystem.clone();

    loop {
        // move y from right to left until we find a non empty space
        while new_unpacked_filesystem[y].1 == None && y > 0 {
            y -= 1;
        }
        if y <= 0 {
            break;
        }

        while x <= new_unpacked_filesystem.len() - 1 
            && x < y
            && (
                new_unpacked_filesystem[x].1 != None
                || (
                    new_unpacked_filesystem[x].1 == None
                    && new_unpacked_filesystem[x].0 < new_unpacked_filesystem[y].0
                )
            )
        {
            x += 1;
        }
        if x >= new_unpacked_filesystem.len() {
            break;
        }

        if y > 0 && x >= y {
            x = 0;
            y -= 1;
            continue;
        }

        if new_unpacked_filesystem[x].0 > 0
            && new_unpacked_filesystem[y].0 > 0
            && new_unpacked_filesystem[x].0 >= new_unpacked_filesystem[y].0
        {
            new_unpacked_filesystem[x].0 -= new_unpacked_filesystem[y].0;
            let temp_y = new_unpacked_filesystem[y];
            new_unpacked_filesystem[y].1 = None;
            new_unpacked_filesystem.insert(x, temp_y);
            x = 0;
        }
    }

    let mut defragged_fs = vec![];
    for (size, id) in new_unpacked_filesystem {
        for _ in 0..size {
            defragged_fs.push(id);
        }
    }

    defragged_fs
}

fn calculate_checksum(unpacked_filesystem: &Vec<Option<usize>>) -> usize {
    unpacked_filesystem.iter()
        .map(|c| c.unwrap_or(0))
        .enumerate()
        .reduce(|acc, (index, val)| (index, acc.1 + (index * val)))
        .unwrap().1
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unpack_filesystem() {
        assert_eq!(
            unpack_filesystem(&"12345".to_string()),
            vec![
                (1, Some(0)),
                (2, None),
                (3, Some(1)),
                (4, None),
                (5, Some(2))
            ],
        );
        assert_eq!(
            unpack_filesystem(&"2333133121414131402".to_string()),
            vec![
                (2, Some(0)),
                (3, None),
                (3, Some(1)),
                (3, None),
                (1, Some(2)),
                (3, None),
                (3, Some(3)),
                (1, None),
                (2, Some(4)),
                (1, None),
                (4, Some(5)),
                (1, None),
                (4, Some(6)),
                (1, None),
                (3, Some(7)),
                (1, None),
                (4, Some(8)),
                (2, Some(9)),
            ],
        );
    }

    #[test]
    fn test_defrag_filesystem() {
        let expected_string = "00992111777.44.333....5555.6666.....8888..";
        let expected_result: Vec<Option<usize>> = expected_string.chars()
            .map(|c| if c != '.' {Some(c.to_string().parse::<usize>().unwrap())} else {None})
            .collect();
        let unpacked_fs = vec![
            (2, Some(0)),
            (3, None),
            (3, Some(1)),
            (3, None),
            (1, Some(2)),
            (3, None),
            (3, Some(3)),
            (1, None),
            (2, Some(4)),
            (1, None),
            (4, Some(5)),
            (1, None),
            (4, Some(6)),
            (1, None),
            (3, Some(7)),
            (1, None),
            (4, Some(8)),
            (2, Some(9)),
        ];
        assert_eq!(
            defrag_filesystem(&unpacked_fs),
            expected_result
        );
    }

    #[test]
    fn test_calculate_checksum() {
        let fs_string = "0099811188827773336446555566..............";
        let input: Vec<Option<usize>> = fs_string.chars()
            .map(|c| if c != '.' {Some(c.to_string().parse::<usize>().unwrap())} else {None})
            .collect();
        assert_eq!(
            calculate_checksum(&input),
            1928,
        );
    }
}


