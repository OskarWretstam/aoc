use std::ops::Mul;
use std::ops::AddAssign;

#[derive(Debug, Clone)]
struct Vector {
    x: i64,
    y: i64,
}

#[derive(Debug)]
enum Direction {
    LEFT,
    RIGHT,
}

impl Vector {
    fn rotate(&mut self, direction: Direction) {
        println!("ROTATE");
        println!("facing {:?} rotate {:?}", self, direction);
        match direction {
            Direction::RIGHT => {
                let tmp = self.x;
                self.x = self.y;
                self.y = -tmp;
            },
            Direction::LEFT => {
                let tmp = self.x;
                self.x = -self.y;
                self.y = tmp;
            },
        }
        println!("facing {:?}", self);
    }

    fn manhattan_len(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

impl<T> Mul<T> for Vector
where
    i64: From<T>,
    T: Copy + std::fmt::Debug
{
    type Output = Vector;
    fn mul(mut self, rhs: T) -> Self::Output {
        println!("mul {:?}*{:?}", self, rhs);
        self.x *= i64::from(rhs);
        self.y *= i64::from(rhs);
        println!("returns: {:?}", self);
        self
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

struct Me {
    position: Vector,
    facing: Vector,
}

impl Me {
    fn new() -> Self {
        Me {
            position: Vector { x: 0, y: 0 },
            facing: Vector { x: 0, y: 1 },
        }
    }

    // TODO: Would like to get rid of clone here but what is the proper way..
    fn step(&mut self, steps: i64) {
        println!("STEP");
        println!("from {:?} facing {:?} steps {}", self.position, self.facing, steps);
        self.position += self.facing.clone()*steps;
        println!("to {:?}", self.position);
    }
}

pub fn s20161() -> String {
    let mut me = Me::new();

    let instrs: Vec<(char, char)> = include_str!("input")
        .split(", ")
        .map(|instr| (instr.chars().nth(0).unwrap(), instr.chars().nth(1).unwrap()))
        .collect();

    for instr in instrs {
        match instr.0 {
            'R' => me.facing.rotate(Direction::RIGHT),
            'L' => me.facing.rotate(Direction::LEFT),
            _ => panic!("Unexpected direction of rotation"),
        }

        me.step(instr.1 as i64 - '0' as i64);
    }

    me.position.manhattan_len().to_string()
}
