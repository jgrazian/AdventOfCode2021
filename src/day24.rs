use crate::prelude::*;

pub struct Day24 {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ALU {
    mem: [i128; 4],
}

impl ALU {
    #[allow(dead_code)]
    fn new() -> Self {
        ALU { mem: [0; 4] }
    }

    #[allow(dead_code)]
    fn run(&mut self, instructions: &[Op], inputs: &[i128]) -> i128 {
        let mut inputs = inputs.iter();
        for op in instructions {
            if let StepResult::Input(i) = self._step(&op) {
                self.mem[i] = *inputs.next().unwrap();
            }
        }
        self.mem[3]
    }

    fn _step(&mut self, op: &Op) -> StepResult {
        match op {
            Op::Inp(i) => StepResult::Input(*i),
            Op::Add(i, v) => {
                self.mem[*i] += self._get_value(v);
                StepResult::Continue
            }
            Op::Mul(i, v) => {
                self.mem[*i] *= self._get_value(v);
                StepResult::Continue
            }
            Op::Div(i, v) => {
                self.mem[*i] /= self._get_value(v);
                StepResult::Continue
            }
            Op::Mod(i, v) => {
                self.mem[*i] = self.mem[*i] % self._get_value(v);
                StepResult::Continue
            }
            Op::Eql(i, v) => {
                self.mem[*i] = if self.mem[*i] == self._get_value(v) {
                    1
                } else {
                    0
                };
                StepResult::Continue
            }
        }
    }

    fn _get_value(&self, v: &Value) -> i128 {
        match v {
            Value::Variable(i) => self.mem[*i],
            Value::Constant(i) => *i,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StepResult {
    Input(usize),
    Continue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Value {
    Variable(usize),
    Constant(i128),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Inp(usize),
    Add(usize, Value),
    Mul(usize, Value),
    Div(usize, Value),
    Mod(usize, Value),
    Eql(usize, Value),
}

fn parse_instructions(input: &str) -> Op {
    let input = input.trim();

    let map_idx = |s| match s {
        "w" => 0usize,
        "x" => 1,
        "y" => 2,
        "z" => 3,
        x @ _ => panic!("Unknown instruction: {}", x),
    };

    let map_var = |v| match v {
        "w" | "x" | "y" | "z" => Value::Variable(map_idx(v)),
        _ => Value::Constant(v.parse().unwrap()),
    };

    match &input[0..3] {
        "inp" => Op::Inp(map_idx(input[4..5].trim())),
        "add" => Op::Add(map_idx(input[4..5].trim()), map_var(&input[6..])),
        "mul" => Op::Mul(map_idx(input[4..5].trim()), map_var(&input[6..])),
        "div" => Op::Div(map_idx(input[4..5].trim()), map_var(&input[6..])),
        "mod" => Op::Mod(map_idx(input[4..5].trim()), map_var(&input[6..])),
        "eql" => Op::Eql(map_idx(input[4..5].trim()), map_var(&input[6..])),
        x @ _ => panic!("Unknown instruction: {}", x),
    }
}

impl Solution for Day24 {
    fn part1(&self, input: &str) -> Box<dyn ToString> {
        let instructions = map_lines(input, parse_instructions).collect::<Vec<_>>();

        let pairs = (0..14)
            .map(|block| {
                let x = if let Op::Add(_, v) = instructions[block * 18 + 5] {
                    if let Value::Constant(x) = v {
                        x
                    } else {
                        panic!("Expected constant value");
                    }
                } else {
                    panic!("Expected add instruction");
                };

                let y = if let Op::Add(_, v) = instructions[block * 18 + 15] {
                    if let Value::Constant(x) = v {
                        x
                    } else {
                        panic!("Expected constant value");
                    }
                } else {
                    panic!("Expected add instruction");
                };

                (x, y)
            })
            .collect::<Vec<_>>();

        let mut stack = Vec::new();
        let mut links = HashMap::new();

        for (i, (a, b)) in pairs.iter().enumerate() {
            if *a > 0 {
                stack.push((i, b));
            } else {
                let (j, bj) = stack.pop().unwrap();
                links.insert(i, (j, bj + a));
            }
        }

        let mut assignments = vec![0; 14];
        for (i, (j, delta)) in links.iter() {
            assignments[*i] = 9.min(9 + delta);
            assignments[*j] = 9.min(9 - delta);
        }

        Box::new(
            assignments
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(""),
        )
    }

    fn part2(&self, input: &str) -> Box<dyn ToString> {
        let instructions = map_lines(input, parse_instructions).collect::<Vec<_>>();

        let pairs = (0..14)
            .map(|block| {
                let x = if let Op::Add(_, v) = instructions[block * 18 + 5] {
                    if let Value::Constant(x) = v {
                        x
                    } else {
                        panic!("Expected constant value");
                    }
                } else {
                    panic!("Expected add instruction");
                };

                let y = if let Op::Add(_, v) = instructions[block * 18 + 15] {
                    if let Value::Constant(x) = v {
                        x
                    } else {
                        panic!("Expected constant value");
                    }
                } else {
                    panic!("Expected add instruction");
                };

                (x, y)
            })
            .collect::<Vec<_>>();

        let mut stack = Vec::new();
        let mut links = HashMap::new();

        for (i, (a, b)) in pairs.iter().enumerate() {
            if *a > 0 {
                stack.push((i, b));
            } else {
                let (j, bj) = stack.pop().unwrap();
                links.insert(i, (j, bj + a));
            }
        }

        let mut assignments = vec![0; 14];
        for (i, (j, delta)) in links.iter() {
            assignments[*i] = 1.max(1 + delta);
            assignments[*j] = 1.max(1 - delta);
        }

        Box::new(
            assignments
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(""),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alu() {
        let mut alu = ALU::new();

        let s = "inp x
                      mul x -1";
        let instructions = map_lines(s, parse_instructions).collect::<Vec<_>>();
        alu.run(&instructions, &[1]);
        assert_eq!(alu.mem[1], -1);

        let s = "inp z
                      inp x
                      mul z 3
                      eql z x";
        let instructions = map_lines(s, parse_instructions).collect::<Vec<_>>();
        assert_eq!(alu.run(&instructions, &[3, 9]), 1);
        assert_eq!(alu.run(&instructions, &[3, 8]), 0);

        let s = "inp w
                      add z w
                      mod z 2
                      div w 2
                      add y w
                      mod y 2
                      div w 2
                      add x w
                      mod x 2
                      div w 2
                      mod w 2";
        let instructions = map_lines(s, parse_instructions).collect::<Vec<_>>();
        alu.run(&instructions, &[10]);
        assert_eq!(alu.mem, [1, 0, 1, 0]);
    }

    // #[test]
    // fn test_part2() {
    //     let day = Day24 {};
    //     assert_eq!(day.part2(INPUT).to_string(), PART2);
    // }
}
