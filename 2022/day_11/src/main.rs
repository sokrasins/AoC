use std::fs;

//const FILE: &str = "test.txt";
const FILE: &str = "final.txt";

enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Null
}

#[derive(Debug)]
struct Move {
    item: i128,
    monkey: usize,
}

impl Move {
    fn new(item: i128, monkey: usize) -> Self {
        Self { item, monkey }
    }
}

struct Operation {
    op: Op,
    lhs: Option<i128>,
    rhs: Option<i128>,
}

impl Operation {
    pub fn new(op: Op, lhs: Option<i128>, rhs: Option<i128>) -> Self {
        Self { op, lhs, rhs }
    }

    pub fn apply(&self, old: i128) -> i128 {
        let lhs = match self.lhs {
            Some(x) => x,
            None => old,
        };

        let rhs = match self.rhs {
            Some(x) => x,
            None => old,
        };

        return match self.op {
            Op::Add => lhs + rhs,
            Op::Subtract => lhs - rhs,
            Op::Multiply => lhs * rhs,
            Op::Divide => lhs / rhs,
            Op::Null => 0,
        }
    }

    fn get_lhs(&self, item: i128) -> i128 {
        match self.lhs {
            Some(x) => x,
            None => item,
        }
    }

    fn get_rhs(&self, item: i128) -> i128 {
        match self.rhs {
            Some(x) => x,
            None => item,
        }
    }
}

struct Test {
    divisor: i128,
    throw_true: usize,
    throw_false: usize,
}

impl Test {
    fn new(divisor: i128, throw_true: usize, throw_false: usize) -> Self {
        Self { divisor, throw_true, throw_false }
    }

    fn apply(&self, item: i128) -> Move {
        Move {
            item,
            monkey: if item % self.divisor == 0 { self.throw_true } else { self.throw_false },
        }
    }
}

struct Monkey {
    items: Vec<i128>,
    operation: Operation,
    test: Test,
    inspections: usize,
}

impl Monkey {
    fn new (items: Vec<i128>, operation: Operation, test: Test) -> Self {
        Self { items, operation, test, inspections: 0 }
    }

    fn round(&mut self, modulus: Option<i128>) -> Vec<Move> {
        let mut move_list: Vec<Move> = Vec::new();

        for item in self.items.iter() {
            let mut result = match modulus {
                Some(m) => *item % m,
                None => *item,
            };
            result = self.operation.apply(result);
            move_list.push(self.test.apply(result));
        }

        self.inspections += self.items.len();

        self.items.clear();

        move_list
    }
}

fn distribute(monkeys: &mut Vec<Monkey>, moves: &Vec<Move>) {
    for mv in moves.iter() {
        monkeys[mv.monkey].items.push(mv.item);
    }
}

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();
    let mut lines = contents.lines();
    let mut monkeys: Vec<Monkey> = Vec::new();

    loop {
        let mut items: Vec<i128> = Vec::new();

        // Throw way monkey number
        lines.next();   

        // Parse items
        let mut items_repr = lines.next().unwrap().split(":");
        items_repr.next();
        let items_list = items_repr.next().unwrap().split(", ");
        for item in items_list {
            items.push(item.trim().parse().unwrap());
        }

        // Parse Operation 
        let mut op_repr = lines.next().unwrap().split_whitespace();
        op_repr.next();
        op_repr.next();
        op_repr.next();

        let lhs: Option<i128> = match op_repr.next().unwrap().parse() {
            Ok(arg) => Some(arg),
            Err(_) => None,
        };

        let op: Op = match op_repr.next().unwrap() {
            "+" => Op::Add,
            "*" => Op::Multiply,
            _ => Op::Null,
        };

        let rhs: Option<i128> = match op_repr.next().unwrap().parse() {
            Ok(arg) => Some(arg),
            Err(_) => None,
        };

        let operation = Operation::new(op, lhs, rhs);

        // Parse test
        let mut test_repr = lines.next().unwrap().split_whitespace();
        test_repr.next();
        test_repr.next();
        test_repr.next();
        let divisor: i128 = test_repr.next().unwrap().parse().unwrap();
        
        let mut true_repr = lines.next().unwrap().split_whitespace();
        true_repr.next();
        true_repr.next();
        true_repr.next();
        true_repr.next();
        true_repr.next();
        let throw_true: usize = true_repr.next().unwrap().parse().unwrap();

        let mut false_repr = lines.next().unwrap().split_whitespace();
        false_repr.next();
        false_repr.next();
        false_repr.next();
        false_repr.next();
        false_repr.next();
        let throw_false: usize = false_repr.next().unwrap().parse().unwrap();

        let test = Test::new(divisor, throw_true, throw_false);

        monkeys.push(Monkey::new(items, operation, test));

        match lines.next() {
            None => break,
            _ => (),
        }
    }

    let rounds: usize = 10000;

    let mut modulo: i128 = 1;
    for m in monkeys.iter() {
        modulo *= m.test.divisor;
    }

    for r in 0..rounds {
        println!("round {}", r);
        for i in 0..monkeys.len() {
            let moves = monkeys[i].round(Some(modulo)); 
            distribute(&mut monkeys, &moves);
        }
    }

    
    for monkey in monkeys.iter() {
        println!("{:?}", monkey.items);
        println!("{}", monkey.inspections);
    }

    let mut inspections: Vec<usize> = monkeys.iter().map(|x| x.inspections).collect();
    inspections.sort();
    inspections.reverse();
    let score = inspections[0] * inspections[1];

    println!("{:?}", inspections);
    println!("score: {}", score);


}
