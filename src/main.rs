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

fn day01() {
   let input = include_str!("day_1_input.txt");

   let mut calories = Vec::new();
   let mut tmp = 0;

   for line in input.lines() {
      let val = line.parse::<i32>();

      if val.is_err() {
         calories.push(tmp);
         tmp = 0;
      }
      else {
         tmp = tmp + val.unwrap();
      }
   }

   calories.sort();
   calories.reverse();
   println!("day01-p1: {}", calories[0]);
   println!("day01-p2: {:?}", &calories[..=2].iter().sum::<i32>());
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
   let input: Vec<(i8, i8)> = include_str!("day_2_input.txt")
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

   println!("day02-p1: {}", total_score);

   let new_score: i32 = input
        .iter()
        .map(|(theirs, ours)| pick_play(*theirs, *ours))
        .map(|(theirs, ours)| score(theirs, ours))
        .sum();

    println!("day02-p2: {}", new_score);
}

fn day03() {
   let input: i32 = include_str!("day_3_input.txt")
   .lines()
   .map(|line| {
      let (first, second) = line.split_at(line.len()/2);
      let first: HashSet<char> = HashSet::from(first.chars().collect());
      let second: HashSet<char> = HashSet::from(second.chars().collect());
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
   println!("day03-p1: {}", input);

   let input: i32 = include_str!("day_3_input.txt")
   .lines()
   .collect::<Vec<&str>>()
   .chunks(3)
   .map(|chunk| {
      let first: HashSet<char> = HashSet::from(chunk[0].chars().collect());
      let second: HashSet<char> = HashSet::from(chunk[1].chars().collect());
      let third: HashSet<char> = HashSet::from(chunk[2].chars().collect());
      let intersection: Vec<char> = (&(&first & &second) & &third).into_iter().collect();
      intersection[0]
   })
   .map(|c|{
      if c.is_lowercase() {
         c as i32 - 96
      } else {
         c as i32 - 64 + 26
      }
   })
   .sum();

    println!("day03-p2: {}", input);
}

fn day06() {
   let input: Vec<char> = include_str!("day_6_input.txt")
   .chars()
   .collect();

   let mut tmp : VecDeque<char> = VecDeque::new();
   let mut count = 0;

   for c in input {
      tmp.push_back(c);
      count = count + 1;
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

   println!("day06-p2: {}", count);
}

fn main() {
    day04();
    day05();
}
