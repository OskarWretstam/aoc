use std::collections::HashSet;
use std::collections::VecDeque;

fn day01() {
    let input = include_str!("1.txt");

    let mut calories = Vec::new();
    let mut tmp = 0;

    for line in input.lines() {
        let val = line.parse::<i32>();

        if val.is_err() {
            calories.push(tmp);
            tmp = 0;
        } else {
            tmp = tmp + val.unwrap();
        }
    }

    calories.sort();
    calories.reverse();
    println!("d01p1: {}", calories[0]);
    println!("d01p2: {:?}", &calories[..=2].iter().sum::<i32>());
}

fn win_multiplier(theirs: i8, ours: i8) -> i8 {
    let diff = ours - theirs;
    (match diff {
        2 => -1,
        -2 => 1,
        _ => diff,
    }) + 1
}

fn score(theirs: i8, ours: i8) -> i32 {
    return (win_multiplier(theirs, ours) * 3 + (ours + 1)) as i32;
}

fn pick_play(theirs: i8, ours: i8) -> (i8, i8) {
    (
        theirs,
        match theirs {
            0 => (ours + 2) % 3,
            2 => (ours + 1) % 3,
            _ => ours,
        },
    )
}

fn day02() {
    let input: Vec<(i8, i8)> = include_str!("2.txt")
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            (
                (iter.next().unwrap().as_bytes()[0] - b'A') as i8,
                (iter.next().unwrap().as_bytes()[0] - b'X') as i8,
            )
        })
        .collect();

    let total_score: i32 = input
        .iter()
        .map(|(theirs, ours)| score(*theirs, *ours))
        .sum();

    println!("d02p1: {}", total_score);

    let new_score: i32 = input
        .iter()
        .map(|(theirs, ours)| pick_play(*theirs, *ours))
        .map(|(theirs, ours)| score(theirs, ours))
        .sum();

    println!("d02p2: {}", new_score);
}

fn day03() {
    let input: i32 = include_str!("3.txt")
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            let first: HashSet<char> = first.chars().collect();
            let second: HashSet<char> = second.chars().collect();
            let common = first.intersection(&second).next().unwrap().to_owned();
            common
        })
        .map(|c| {
            if c.is_lowercase() {
                c as i32 - 96
            } else {
                c as i32 - 64 + 26
            }
        })
        .sum();
    println!("d03p1: {}", input);

    let input: i32 = include_str!("3.txt")
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            let first: HashSet<char> = chunk[0].chars().collect();
            let second: HashSet<char> = chunk[1].chars().collect();
            let third: HashSet<char> = chunk[2].chars().collect();
            let intersection: Vec<char> = (&(&first & &second) & &third).into_iter().collect();
            intersection[0]
        })
        .map(|c| {
            if c.is_lowercase() {
                c as i32 - 96
            } else {
                c as i32 - 64 + 26
            }
        })
        .sum();

    println!("d03p2: {}", input);
}

fn day04() {
    let input: i32 = include_str!("4.txt")
        .lines()
        .map(|line| {
            let mut interval = Vec::new();
            for s in line.split(',') {
                for ss in s.split('-') {
                    interval.push(ss.parse::<i32>().unwrap());
                }
            }

            if interval[0] <= interval[2] && interval[1] >= interval[3] {
                return 1;
            }
            if interval[2] <= interval[0] && interval[3] >= interval[1] {
                return 1;
            }
            0
        })
        .sum();

    println!("d04p1: {:?}", input);

    let input: i32 = include_str!("4.txt")
        .lines()
        .map(|line| {
            let mut interval = Vec::new();
            for s in line.split(',') {
                for ss in s.split('-') {
                    interval.push(ss.parse::<i32>().unwrap());
                }
            }

            if interval[0] >= interval[2] && interval[0] <= interval[3] {
                return 1;
            }

            if interval[1] >= interval[2] && interval[1] <= interval[3] {
                return 1;
            }

            if interval[2] >= interval[0] && interval[2] <= interval[1] {
                return 1;
            }

            if interval[3] >= interval[0] && interval[3] <= interval[1] {
                return 1;
            }

            0
        })
        .sum();

    println!("d04p2: {:?}", input);
}

fn day05() {
    let input = include_str!("5.txt");
    let mut stacks: Vec<Vec<char>> = Vec::new();

    // Prepare the stacks...
    for _ in 0..9 {
        stacks.push(Vec::new());
    }

    for line in input.lines() {
        if line.contains(" 1  ") {
            for i in 0..stacks.len() {
                stacks[i].reverse();
            }
            break;
        }

        let chars: Vec<char> = line.chars().collect();
        let mut stack = 0;
        for i in (1..chars.len()).step_by(4) {
            if chars[i].is_uppercase() {
                stacks[stack].push(chars[i]);
            }
            stack += 1;
        }
    }

    // preform the moves p1
    let mut p1_stacks = stacks.clone();
    for line in input.lines().skip_while(|tmp| !tmp.contains("move")) {
        let instr: Vec<usize> = line
            .split_ascii_whitespace()
            .map(|col| match col.parse::<usize>() {
                Ok(a) => a,
                _ => 0,
            })
            .collect();

        for _ in 0..instr[1] {
            let tmp = p1_stacks[instr[3] - 1].pop().unwrap();
            p1_stacks[instr[5] - 1].push(tmp);
        }
    }

    print!("d05p1: ");
    for i in 0..stacks.len() {
        print!("{}", p1_stacks[i].last().unwrap());
    }
    println!();

    // preform the moves p2
    let mut p2_stacks = stacks.clone();
    for line in input.lines().skip_while(|tmp| !tmp.contains("move")) {
        let instr: Vec<usize> = line
            .split_ascii_whitespace()
            .map(|col| match col.parse::<usize>() {
                Ok(a) => a,
                _ => 0,
            })
            .collect();

        let mut tmp_vec = Vec::new();
        for _ in 0..instr[1] {
            tmp_vec.push(p2_stacks[instr[3] - 1].pop().unwrap());
        }

        for _ in 0..instr[1] {
            p2_stacks[instr[5] - 1].push(tmp_vec.pop().unwrap());
        }
    }

    print!("d05p2: ");
    for i in 0..stacks.len() {
        print!("{}", p2_stacks[i].last().unwrap());
    }
    println!();
}

fn day06() {
    let input: Vec<char> = include_str!("6.txt").chars().collect();

    let mut tmp: VecDeque<char> = VecDeque::new();
    let mut count = 0;

    for c in input.clone() {
        tmp.push_back(c);
        count += 1;
        if tmp.len() == 4 {
            let mut tmptmp = Vec::from(tmp.clone());
            tmp.pop_front();
            tmptmp.sort();
            tmptmp.dedup();
            if tmptmp.len() == 4 {
                break;
            }
        }
    }

    println!("d06p1: {}", count);

    let mut tmp: VecDeque<char> = VecDeque::new();
    let mut count = 0;

    for c in input {
        tmp.push_back(c);
        count += 1;
        if tmp.len() == 14 {
            let mut tmptmp = Vec::from(tmp.clone());
            tmp.pop_front();
            tmptmp.sort();
            tmptmp.dedup();
            if tmptmp.len() == 14 {
                break;
            }
        }
    }

    println!("d06p2: {}", count);
}

#[allow(dead_code)]
struct Directory {
    name: String,
    size: usize,
    parent: usize,
}

fn day07() {
    let input: Vec<&str> = include_str!("7.txt")
        .lines()
        .filter(|line| !line.contains("$ ls"))
        .filter(|line| !line.contains("dir"))
        .collect();

    let mut filesystem = vec![Directory {
        name: "/".to_string(),
        size: 0,
        parent: 0,
    }];

    let root_index = 0;
    let mut current_index = 0;

    for cmd in input {
        if cmd.contains("cd ") {
            let tmp = cmd.split(' ').collect::<Vec<&str>>();
            let path = tmp[2];

            match path {
                "/" => current_index = root_index,
                ".." => current_index = filesystem[current_index].parent,
                a_directory => {
                    filesystem.push(Directory {
                        name: String::from(a_directory),
                        size: 0,
                        parent: current_index,
                    });
                    current_index = filesystem.len() - 1;
                }
            }
        } else {
            let tmp = cmd.split(' ').collect::<Vec<&str>>();
            let size = tmp[0].parse::<usize>().unwrap();

            let mut ix = current_index;
            loop {
                filesystem[ix].size += size;
                if ix == root_index {
                    break;
                }
                ix = filesystem[ix].parent;
            }
        }
    }

    let p1: usize = filesystem
        .iter()
        .map(|dir| dir.size)
        .filter(|size| *size <= 100000)
        .sum();

    println!("d07p1: {}", p1);

    let free_space = 70000000 - filesystem[0].size;
    let required_space = 30000000 - free_space;

    let p2 = filesystem
        .iter()
        .map(|d| d.size)
        .filter(|size| *size >= required_space)
        .min()
        .unwrap();

    println!("d07p2: {}", p2);
}

fn is_visible(data: &[Vec<u32>], x: usize, y: usize, side: usize) -> i32 {
    let height = data[x][y];
    let mut tmp;
    let mut seen_directions = 4;

    tmp = x;
    while tmp > 0 {
        tmp -= 1;
        if data[tmp][y] >= height {
            seen_directions -= 1;
            break;
        }
    }

    tmp = x;
    while tmp < side - 1 {
        tmp += 1;
        if data[tmp][y] >= height {
            seen_directions -= 1;
            break;
        }
    }

    tmp = y;
    while tmp > 0 {
        tmp -= 1;
        if data[x][tmp] >= height {
            seen_directions -= 1;
            break;
        }
    }

    tmp = y;
    while tmp < side - 1 {
        tmp += 1;
        if data[x][tmp] >= height {
            seen_directions -= 1;
            break;
        }
    }

    if seen_directions == 0 {
        return 0;
    }

    1
}

fn scenic_score(data: &[Vec<u32>], x: usize, y: usize, side: usize) -> usize {
    let height = data[x][y];
    let mut tmp;
    let mut score = 1;

    tmp = x;
    while tmp > 0 {
        tmp -= 1;
        if data[tmp][y] >= height {
            break;
        }
    }
    score *= x - tmp;

    tmp = x;
    while tmp < side - 1 {
        tmp += 1;
        if data[tmp][y] >= height {
            break;
        }
    }
    score *= tmp - x;

    tmp = y;
    while tmp > 0 {
        tmp -= 1;
        if data[x][tmp] >= height {
            break;
        }
    }
    score *= y - tmp;

    tmp = y;
    while tmp < side - 1 {
        tmp += 1;
        if data[x][tmp] >= height {
            break;
        }
    }
    score *= tmp - y;

    score
}

fn day08() {
    let input = include_str!("8.txt");
    let mut data: Vec<Vec<u32>> = Vec::new();

    for line in input.lines() {
        data.push(Vec::new());
        for c in line.chars() {
            let end = data.len() - 1;
            data[end].push(c.to_digit(10).unwrap());
        }
    }

    let mut visible_count: i32 = 0;
    for x in 0..data.len() {
        for y in 0..data[x].len() {
            visible_count += is_visible(&data, x, y, data.len());
        }
    }
    println!("d08p1: {}", visible_count);

    let mut scenic_scores: Vec<usize> = Vec::new();
    for x in 0..data.len() {
        for y in 0..data[x].len() {
            scenic_scores.push(scenic_score(&data, x, y, data.len()));
        }
    }
    println!("d08p2: {}", scenic_scores.iter().max().unwrap());
}

fn main() {
    day01();
    day02();
    day03();
    day04();
    day05();
    day06();
    day07();
    day08();
}
