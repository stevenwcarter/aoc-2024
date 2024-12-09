use rayon::prelude::*;
advent_of_code::solution!(4);

fn find_word_in_grid(grid: &[Vec<char>]) -> usize {
    let directions = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let word_chars: Vec<char> = "XMAS".chars().collect();
    let rows = grid.len();
    let cols = grid[0].len();

    (0..rows)
        .into_par_iter()
        .map(|r| {
            (0..cols)
                .map(|c| {
                    if grid[r][c] == 'X' {
                        directions
                            .iter()
                            .filter(|(dr, dc)| {
                                check_word(grid, &word_chars, r as isize, c as isize, *dr, *dc)
                            })
                            .count()
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn check_word(
    grid: &[Vec<char>],
    word_chars: &[char],
    mut r: isize,
    mut c: isize,
    dr: isize,
    dc: isize,
) -> bool {
    r += dr;
    c += dc;
    if r + 3 * dr < 0
        || r + 3 * dr >= grid.len() as isize
        || c + 3 * dc < 0
        || c + 3 * dc >= grid[0].len() as isize
    {
        return false;
    }
    for &ch in &word_chars[1..] {
        if grid[r as usize][c as usize] != ch {
            return false;
        }
        r += dr;
        c += dc;
    }
    true
}

fn find_intersecting_mas(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 'A'
                && (check_mas(grid, r, c, -1, -1) && check_mas(grid, r, c, 1, -1)
                    || check_mas(grid, r, c, -1, -1) && check_mas(grid, r, c, -1, 1)
                    || check_mas(grid, r, c, 1, 1) && check_mas(grid, r, c, -1, 1)
                    || check_mas(grid, r, c, 1, 1) && check_mas(grid, r, c, 1, -1))
            {
                count += 1;
            }
        }
    }

    count
}

fn check_mas(grid: &[Vec<char>], r: usize, c: usize, dr: isize, dc: isize) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();

    let (r1, c1) = (r as isize + dr, c as isize + dc);
    let (r2, c2) = (r as isize - dr, c as isize - dc);

    if r1 >= 0
        && r1 < rows as isize
        && c1 >= 0
        && c1 < cols as isize
        && r2 >= 0
        && r2 < rows as isize
        && c2 >= 0
        && c2 < cols as isize
    {
        grid[r1 as usize][c1 as usize] == 'M' && grid[r2 as usize][c2 as usize] == 'S'
    } else {
        false
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let occurrences = find_word_in_grid(&grid);

    Some(occurrences)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let occurrences = find_intersecting_mas(&grid);

    Some(occurrences)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(17));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
