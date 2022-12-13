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

    println!("d4p1: {:?}", input);

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

    println!("d4p2: {:?}", input);
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

    print!("d5p1: ");
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

    print!("d5p2: ");
    for i in 0..stacks.len() {
        print!("{}", p2_stacks[i].last().unwrap());
    }
    println!();
}

fn main() {
    day04();
    day05();
}
