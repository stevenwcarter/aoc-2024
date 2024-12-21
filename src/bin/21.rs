use cached::proc_macro::cached;

advent_of_code::solution!(21);

fn number_pad(key: char) -> (i32, i32) {
    match key {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '0' => (1, 3),
        'A' => (2, 3),
        _ => unreachable!("Invalid character {key}"),
    }
}

fn directional_pad(key: char) -> (i32, i32) {
    match key {
        '^' => (1, 0),
        'A' => (2, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        _ => unreachable!("Invalid character {key}"),
    }
}

#[cached]
fn find_arrow_sequence(
    x: i32,
    y: i32,
    recursion_depth: usize,
    check_horizontal_first: bool,
) -> usize {
    let (x_offset, y_offset) = (x.unsigned_abs(), y.unsigned_abs());
    let mut directions_to_check: Vec<char> = Vec::new();
    if x > 0 {
        (0..x_offset).for_each(|_| {
            directions_to_check.push('<');
        });
    } else {
        (0..x_offset).for_each(|_| {
            directions_to_check.push('>');
        });
    }
    if y > 0 {
        (0..y_offset).for_each(|_| {
            directions_to_check.push('^');
        });
    } else {
        (0..y_offset).for_each(|_| {
            directions_to_check.push('v');
        });
    }

    if !check_horizontal_first {
        directions_to_check.reverse();
    }

    directions_to_check.push('A');

    if recursion_depth == 0 {
        directions_to_check.len()
    } else {
        let recursion_depth = recursion_depth - 1;
        let mut current_location = directional_pad('A');

        directions_to_check
            .into_iter()
            .map(|c| {
                let needed = directional_pad(c);
                let point = current_location;
                current_location = needed;
                let offset = (point.0 - needed.0, point.1 - needed.1);
                if offset.0 == 0 || offset.1 == 0 || (needed == (0, 1) && point.1 == 0) {
                    find_arrow_sequence(offset.0, offset.1, recursion_depth, false)
                } else if point == (0, 1) && needed.1 == 0 {
                    find_arrow_sequence(offset.0, offset.1, recursion_depth, true)
                } else {
                    find_arrow_sequence(offset.0, offset.1, recursion_depth, false).min(
                        find_arrow_sequence(offset.0, offset.1, recursion_depth, true),
                    )
                }
            })
            .sum()
    }
}

fn solve_passcode(passcode: &str, recursion_depth: usize) -> usize {
    let mut current_location = number_pad('A');

    // remove trailing A: 029A -> 029 -> 29
    let n = passcode[0..3].parse::<usize>().unwrap();

    n * passcode
        .chars()
        .map(|ch| {
            let needed = number_pad(ch);
            let point = current_location;
            let offset = (current_location.0 - needed.0, current_location.1 - needed.1);
            current_location = needed;
            // prefer up > right > other
            if point.1 == 3 && needed.0 == 0 {
                find_arrow_sequence(offset.0, offset.1, recursion_depth, false)
            } else if point.1 == 0 && needed.0 == 3 {
                find_arrow_sequence(offset.0, offset.1, recursion_depth, true)
            } else {
                find_arrow_sequence(offset.0, offset.1, recursion_depth, true).min(
                    find_arrow_sequence(offset.0, offset.1, recursion_depth, false),
                )
            }
        })
        .sum::<usize>()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|passcode| solve_passcode(passcode, 2))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|passcode| solve_passcode(passcode, 25))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154115708116294));
    }
}
