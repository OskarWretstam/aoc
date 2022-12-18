use std::cmp::max;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::ops::Add;
use std::ops::Sub;

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
    (win_multiplier(theirs, ours) * 3 + (ours + 1)) as i32
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

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, _rhs: Coord) -> Coord {
        Coord {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl Coord {
    fn distance(self, other: Coord) -> i32 {
        let d1 = (self.x - other.x).abs();
        let d2 = (self.y - other.y).abs();
        max(d1, d2)
    }

    fn to_unit(self) -> Coord {
        Coord {
            x: if self.x > 0 {
                1
            } else if self.x < 0 {
                -1
            } else {
                0
            },
            y: if self.y > 0 {
                1
            } else if self.y < 0 {
                -1
            } else {
                0
            },
        }
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, _rhs: Coord) -> Coord {
        Coord {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

#[allow(dead_code)]
struct Move {
    direction: Coord,
    number_of_steps: i32,
}

fn day09() {
    let input = include_str!("9.txt");

    let mut head = Coord { x: 0, y: 0 };
    let mut tail = Coord { x: 0, y: 0 };

    let mut tail_visited: Vec<Coord> = Vec::new();
    tail_visited.push(tail);
    let mut moves: Vec<Move> = Vec::new();

    for line in input.lines() {
        let tmp: Vec<&str> = line.split(' ').collect();
        let direction = match tmp[0] {
            "R" => Coord { x: 1, y: 0 },
            "U" => Coord { x: 0, y: 1 },
            "L" => Coord { x: -1, y: 0 },
            "D" => Coord { x: 0, y: -1 },
            _ => panic!["Parsed unknown direction: {}", tmp[0]],
        };

        moves.push(Move {
            direction,
            number_of_steps: tmp[1].parse::<i32>().unwrap(),
        });
    }

    for m in &moves {
        for _ in 0..m.number_of_steps {
            head = head + m.direction;
            if head.distance(tail) > 1 {
                tail = tail + (head - tail).to_unit();
                if !tail_visited.contains(&tail) {
                    tail_visited.push(tail);
                }
            }
        }
    }

    println!("d09p1: {}", tail_visited.len());

    let mut knots: Vec<Coord> = Vec::new();
    for _ in 0..10 {
        knots.push(Coord { x: 0, y: 0 });
    }

    let mut tail_visited: Vec<Coord> = Vec::new();
    tail_visited.push(knots[0]);

    for m in &moves {
        for _ in 0..m.number_of_steps {
            knots[0] = knots[0] + m.direction;
            for i in 1..knots.len() {
                if knots[i - 1].distance(knots[i]) > 1 {
                    knots[i] = knots[i] + (knots[i - 1] - knots[i]).to_unit();
                }
            }
            if !tail_visited.contains(&knots[knots.len() - 1]) {
                tail_visited.push(knots[knots.len() - 1]);
            }
        }
    }

    println!("d09p2: {}", tail_visited.len());
}

fn day10() {
    let input = include_str!("10.txt");
    let mut x = 1;
    let mut signal: Vec<i32> = Vec::new();
    let mut cycle = 0;
    let mut to_add;
    let stops = vec![20, 60, 100, 140, 180, 220];
    let mut stop_ix = 0;

    for line in input.lines() {
        match line {
            "noop" => {
                cycle += 1;
                to_add = 0;
            }
            _ => {
                cycle += 2;
                let tmp: Vec<&str> = line.split(' ').collect();
                to_add = tmp[1].parse::<i32>().unwrap();
            }
        };

        if stop_ix < stops.len() && cycle >= stops[stop_ix] {
            signal.push(stops[stop_ix] * x);
            x += to_add;
            stop_ix += 1;
        } else {
            x += to_add;
        }
    }

    println!("d10p1: {}", signal.into_iter().sum::<i32>());

    x = 1;
    cycle = 0;
    let mut pixels: Vec<char> = Vec::new();

    for line in input.lines() {
        match line {
            "noop" => {
                if cycle >= x - 1 && cycle <= x + 1 {
                    pixels.push('#');
                } else {
                    pixels.push('.');
                }
                cycle += 1;
                cycle %= 40;
            }
            _ => {
                if cycle >= x - 1 && cycle <= x + 1 {
                    pixels.push('#');
                } else {
                    pixels.push('.');
                }
                cycle += 1;
                cycle %= 40;

                if cycle >= x - 1 && cycle <= x + 1 {
                    pixels.push('#');
                } else {
                    pixels.push('.');
                }
                cycle += 1;
                cycle %= 40;

                let tmp: Vec<&str> = line.split(' ').collect();
                x += tmp[1].parse::<i32>().unwrap();
            }
        }
    }

    println!("d10p2:");
    for j in 0..6 {
        for i in 0..40 {
            print!("{}", pixels[(j * 40) + i]);
        }
        println!();
    }
}

// #[allow(dead_code)]
// struct Monkey<F, G>
// where
//     F: Fn(i32) -> i32,
//     G: Fn(i32) -> i32,
// {
//     items: VecDeque<i32>,
//     operation: F,
//     test: G,
// }
//
// impl<F, G> Monkey<F, G>
// where
//     F: Fn(i32) -> i32,
//     G: Fn(i32) -> i32,
// {
//     fn new(items: VecDeque<i32>, operation: F, test: G) -> Self {
//         Self {
//             items,
//             operation,
//             test,
//         }
//     }
// }

#[allow(dead_code)]
struct Monkey {
    items: VecDeque<usize>,
    inspection_count: usize,
    pub operation: Box<dyn Fn(usize) -> usize>,
    pub test: Box<dyn Fn(usize) -> usize>,
}

#[allow(dead_code)]
impl Monkey {
    fn new(
        items: VecDeque<usize>,
        inspection_count: usize,
        operation: impl Fn(usize) -> usize + 'static,
        test: impl Fn(usize) -> usize + 'static,
    ) -> Self {
        Self {
            items,
            inspection_count,
            operation: Box::new(operation),
            test: Box::new(test),
        }
    }
}

fn parse_initial_monkeys(input: Vec<&'static str>) -> (Vec<Monkey>, usize) {
    let mut monkeys = Vec::new();
    let mut worry_level_limiter: usize = 1;
    // Parse..
    for monkey in input {
        let mut iter = monkey.lines();
        iter.next().unwrap(); // Monkey numbers, don't care

        let starting_items = iter
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .skip_while(|word| word.chars().next().unwrap().is_alphabetic())
            .map(|word| word.replace(',', ""))
            .map(|word| word.parse::<usize>().unwrap());

        // Construct closure for operation
        let tmp: Vec<&str> = iter
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .skip_while(|word| word.chars().next().unwrap().is_alphabetic())
            .collect();

        let operation: Box<dyn Fn(usize) -> usize> = match tmp[tmp.len() - 2] {
            "+" => {
                if tmp[tmp.len() - 1] == "old" {
                    Box::new(|worry_level: usize| worry_level + worry_level)
                } else {
                    Box::new(move |worry_level: usize| {
                        worry_level + tmp[tmp.len() - 1].parse::<usize>().unwrap()
                    })
                }
            }
            "*" => {
                if tmp[tmp.len() - 1] == "old" {
                    Box::new(|worry_level: usize| worry_level * worry_level)
                } else {
                    Box::new(move |worry_level: usize| {
                        worry_level * tmp[tmp.len() - 1].parse::<usize>().unwrap()
                    })
                }
            }
            _ => {
                panic!("Operation: unknown operand {}", tmp[tmp.len() - 2])
            }
        };

        // Construct closure for test
        let tmp: Vec<usize> = iter
            .map(|line| line.trim())
            .map(|line| {
                let words: Vec<&str> = line.split(' ').collect();
                words[words.len() - 1].parse::<usize>().unwrap()
            })
            .collect();

        worry_level_limiter *= tmp[0];

        let test = Box::new(move |worry_level| {
            if worry_level % tmp[0] == 0 {
                tmp[1]
            } else {
                tmp[2]
            }
        });

        monkeys.push(Monkey {
            items: VecDeque::from_iter(starting_items),
            inspection_count: 0,
            operation,
            test,
        });
    }
    (monkeys, worry_level_limiter)
}

fn day11() {
    let input: Vec<&str> = include_str!("11.txt").split("\n\n").collect();
    let (mut monkeys_p1, _) = parse_initial_monkeys(input);
    for _ in 0..20 {
        for i in 0..monkeys_p1.len() {
            while let Some(mut item) = monkeys_p1[i].items.pop_front() {
                monkeys_p1[i].inspection_count += 1;
                item = (monkeys_p1[i].operation)(item);
                item /= 3;
                let ix = (monkeys_p1[i].test)(item);
                monkeys_p1[ix].items.push_back(item);
            }
        }
    }

    let mut p1: Vec<usize> = monkeys_p1
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect::<Vec<usize>>();
    p1.sort_by(|a, b| b.cmp(a));

    println!("d11p1: {}", p1[0] * p1[1]);

    let input: Vec<&str> = include_str!("11.txt").split("\n\n").collect();

    let (mut monkeys_p2, worry_level_limiter) = parse_initial_monkeys(input);
    for _ in 0..10000 {
        for i in 0..monkeys_p2.len() {
            while let Some(mut item) = monkeys_p2[i].items.pop_front() {
                monkeys_p2[i].inspection_count += 1;
                item = (monkeys_p2[i].operation)(item);
                item %= worry_level_limiter;
                let ix = (monkeys_p2[i].test)(item);
                monkeys_p2[ix].items.push_back(item);
            }
        }
    }

    let mut p2: Vec<usize> = monkeys_p2
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect::<Vec<usize>>();
    p2.sort_by(|a, b| b.cmp(a));

    println!("d11p2: {}", p2[0] * p2[1]);
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
    day09();
    day10();
    day11();
}
