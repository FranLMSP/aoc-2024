use std::fs;

pub fn run() {
    let file = fs::read_to_string("./inputs/star_seventeen.txt").unwrap();
    let unpacked_fs = unpack_filesystem(&file);
    let defragged_fs = defrag_filesystem(&unpacked_fs);
    let checksum = calculate_checksum(&defragged_fs);

    println!("Result: {}", checksum);
}

fn unpack_filesystem(filesystem: &String) -> Vec<Option<usize>> {
    let mut new_chars = vec![];
    let mut is_white_space = false;
    let mut index = 0;
    for current_char in filesystem.chars() {
        let count = current_char.to_string().parse::<usize>().unwrap();
        let char_to_insert = match is_white_space {
            true => None,
            false => Some(index),
        };

        for _ in 0..count {
            new_chars.push(char_to_insert);
        }

        if !is_white_space && count > 0 {
            index += 1;
        }

        is_white_space = !is_white_space;
    }

    new_chars
}

fn defrag_filesystem(unpacked_filesystem: &Vec<Option<usize>>) -> Vec<Option<usize>> {
    let mut x = 0;
    let mut y = unpacked_filesystem.len() - 1;
    let mut is_finished = false;

    let mut new_fs = unpacked_filesystem.clone();

    while !is_finished {
        // move x from left to right until we find an empty space
        while new_fs[x] != None && x < unpacked_filesystem.len() {
            x += 1;
        }

        // move y from right to left until we find a non empty space
        while new_fs[y] == None && y > 0 {
            y -= 1;
        }

        is_finished = x > y;
        if !is_finished {
            new_fs[x] = new_fs[y];
            new_fs[y] = None;
        }
    }

    new_fs
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
        let expected_string = "00...111...2...333.44.5555.6666.777.888899";
        let expected_result: Vec<Option<usize>> = expected_string.chars()
            .map(|c| if c != '.' {Some(c.to_string().parse::<usize>().unwrap())} else {None})
            .collect();
        assert_eq!(
            unpack_filesystem(&"2333133121414131402".to_string()),
            expected_result
        );

        let expected_string = "0..111....22222";
        let expected_result: Vec<Option<usize>> = expected_string.chars()
            .map(|c| if c != '.' {Some(c.to_string().parse::<usize>().unwrap())} else {None})
            .collect();
        assert_eq!(
            unpack_filesystem(&"12345".to_string()),
            expected_result,
        );
    }

    #[test]
    fn test_defrag_filesystem() {
        let fs_string = "00...111...2...333.44.5555.6666.777.888899";
        let input: Vec<Option<usize>> = fs_string.chars()
            .map(|c| if c != '.' {Some(c.to_string().parse::<usize>().unwrap())} else {None})
            .collect();
        let expected_string = "0099811188827773336446555566..............";
        let expected_result: Vec<Option<usize>> = expected_string.chars()
            .map(|c| if c != '.' {Some(c.to_string().parse::<usize>().unwrap())} else {None})
            .collect();
        assert_eq!(
            defrag_filesystem(&input),
            expected_result
        );

        let fs_string = "0..111....22222";
        let input: Vec<Option<usize>> = fs_string.chars()
            .map(|c| if c != '.' {Some(c.to_string().parse::<usize>().unwrap())} else {None})
            .collect();
        let expected_string = "022111222......";
        let expected_result: Vec<Option<usize>> = expected_string.chars()
            .map(|c| if c != '.' {Some(c.to_string().parse::<usize>().unwrap())} else {None})
            .collect();
        assert_eq!(
            defrag_filesystem(&input),
            expected_result,
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

