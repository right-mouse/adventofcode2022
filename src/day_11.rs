use crate::*;
use itertools::Itertools;
use std::{
    cell::RefCell,
    collections::{vec_deque, BinaryHeap, VecDeque},
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
    str::FromStr,
};

#[derive(Clone)]
struct Items {
    items: VecDeque<usize>,
    num_inspected: usize,
}

impl FromIterator<usize> for Items {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        Self {
            items: VecDeque::from_iter(iter),
            num_inspected: 0,
        }
    }
}

impl Items {
    fn iter(&self) -> vec_deque::Iter<'_, usize> {
        self.items.iter()
    }

    fn pop_front(&mut self) -> Option<usize> {
        let item = self.items.pop_front();
        if item.is_some() {
            self.num_inspected += 1;
        }
        item
    }

    fn push_back(&mut self, value: usize) {
        self.items.push_back(value)
    }
}

struct Monkey {
    items: RefCell<Items>,
    operation: Rc<dyn Fn(usize) -> usize>,
    test: Rc<dyn Fn(usize) -> usize>,
    n: usize,
}

impl Clone for Monkey {
    fn clone(&self) -> Self {
        Self {
            items: RefCell::new(self.items.borrow().clone()),
            operation: self.operation.clone(),
            test: self.test.clone(),
            n: self.n,
        }
    }
}

impl FromStr for Monkey {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        if lines.len() != 6 {
            return Err(format!("malformed input:\n{s}").into());
        }

        let items = RefCell::new(
            lines[1]
                .trim_start()
                .trim_start_matches("Starting items: ")
                .split(", ")
                .into_iter()
                .map(|n| n.parse::<usize>())
                .collect::<Result<Items, _>>()?,
        );

        enum OperationArg {
            Old,
            Constant(usize),
        }
        enum OperationType {
            Add,
            Multiply,
        }
        let operation_parts = lines[2]
            .trim_start()
            .trim_start_matches("Operation: new = ")
            .split(' ')
            .collect::<Vec<_>>();
        if operation_parts.len() != 3 {
            return Err(format!("malformed input:\n{s}").into());
        }
        let operation_lhs = if operation_parts[0] == "old" {
            OperationArg::Old
        } else {
            OperationArg::Constant(operation_parts[0].parse()?)
        };
        let operation_op = match operation_parts[1] {
            "+" => OperationType::Add,
            "*" => OperationType::Multiply,
            _ => return Err(format!("malformed input:\n{s}").into()),
        };
        let operation_rhs = if operation_parts[2] == "old" {
            OperationArg::Old
        } else {
            OperationArg::Constant(operation_parts[2].parse()?)
        };
        let operation = Rc::new(move |n| {
            let lhs = match operation_lhs {
                OperationArg::Old => n,
                OperationArg::Constant(c) => c,
            };
            let rhs = match operation_rhs {
                OperationArg::Old => n,
                OperationArg::Constant(c) => c,
            };
            match operation_op {
                OperationType::Add => lhs + rhs,
                OperationType::Multiply => lhs * rhs,
            }
        });

        let test_divisible_by = lines[3]
            .trim_start()
            .trim_start_matches("Test: divisible by ")
            .parse::<usize>()?;
        let test_if_true = lines[4]
            .trim_start()
            .trim_start_matches("If true: throw to monkey ")
            .parse::<usize>()?;
        let test_if_false = lines[5]
            .trim_start()
            .trim_start_matches("If false: throw to monkey ")
            .parse::<usize>()?;
        let test = Rc::new(move |n| {
            if n % test_divisible_by == 0 {
                test_if_true
            } else {
                test_if_false
            }
        });

        Ok(Monkey {
            items,
            operation,
            test,
            n: test_divisible_by,
        })
    }
}

#[derive(Clone)]
struct KeepAway {
    monkeys: Vec<Monkey>,
    m: usize,
}

impl Display for KeepAway {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, monkey) in self.monkeys.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "Monkey {i}: ")?;
            for (j, item) in monkey.items.borrow().iter().enumerate() {
                if j > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", *item)?;
            }
        }
        Ok(())
    }
}

impl KeepAway {
    fn simulate_round(&mut self, manageable_worry: bool) {
        for monkey in self.monkeys.iter() {
            while let Some(mut item) = monkey.items.borrow_mut().pop_front() {
                item = (monkey.operation)(item);
                if manageable_worry {
                    item /= 3;
                } else {
                    item %= self.m;
                }
                let idx = (monkey.test)(item);
                self.monkeys[idx].items.borrow_mut().push_back(item);
            }
        }
    }

    fn monkey_business(&self) -> Result<usize, Box<dyn Error>> {
        let mut inspect_heap = self
            .monkeys
            .iter()
            .map(|m| m.items.borrow().num_inspected)
            .collect::<BinaryHeap<_>>();
        let first = inspect_heap.pop().ok_or("not enough monkeys")?;
        let second = inspect_heap.pop().ok_or("not enough monkeys")?;
        Ok(first * second)
    }
}

pub fn solve(input: File) -> Result<Solution, Box<dyn Error>> {
    let reader = BufReader::new(input);
    let mut keep_away_manageable = KeepAway {
        monkeys: Vec::new(),
        m: 0,
    };
    for mut line in reader
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|line| !line.is_empty())
        .chunks(6)
        .into_iter()
    {
        keep_away_manageable.monkeys.push(Monkey::from_str(&line.join("\n"))?);
    }
    keep_away_manageable.m = keep_away_manageable.monkeys.iter().map(|monkey| monkey.n).product();
    let mut keep_away_unmanageable = keep_away_manageable.clone();
    for _ in 0..20 {
        keep_away_manageable.simulate_round(true);
    }
    for _ in 0..10000 {
        keep_away_unmanageable.simulate_round(false);
    }
    Ok((
        Box::new(keep_away_manageable.monkey_business()?),
        Box::new(keep_away_unmanageable.monkey_business()?),
    ))
}
