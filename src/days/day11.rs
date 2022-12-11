use std::error::Error;
use std::fs;
use std::path::Path;

type MonkeyAction = dyn Fn(usize) -> usize;
struct Monkey {
    inventory: Vec<usize>,
    inspection: Box<MonkeyAction>,
    test: Box<MonkeyAction>,
    inspection_performed: usize,
}

impl Monkey {
    fn new(descriptor: &str) -> Monkey {
        let blocks = descriptor.lines().collect::<Vec<_>>();
        let inventory_string = blocks[1].split(": ").collect::<Vec<_>>()[1];

        let inventory = inventory_string
            .split(", ")
            .map(|level| level.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let inspection = Monkey::get_op_from_str(blocks[2]);
        let test_string = descriptor.lines().skip(3).collect::<Vec<_>>();
        let test = Monkey::get_test_from_str(test_string);

        Monkey {
            inventory,
            inspection: Box::new(inspection),
            test: Box::new(test),
            inspection_performed: 0,
        }
    }

    fn execute(&mut self) -> Vec<(usize, usize)> {
        let mut targets = vec![];
        for item in self.inventory.iter() {
            let new_level = Box::as_ref(&self.inspection)(*item);
            let target = Box::as_ref(&self.test)(new_level);
            targets.push((target, new_level));
            self.inspection_performed += 1;
        }

        self.inventory = vec![];
        targets
    }

    fn get_op_from_str(op_string: &str) -> impl Fn(usize) -> usize {
        let output = op_string.split(" = ").collect::<Vec<_>>()[1];
        let temp = output.split(' ').collect::<Vec<_>>();

        let right = temp[2].to_owned();
        let operand = temp[1].chars().next().unwrap();

        move |worry_lvl: usize| -> usize {
            let other = if right == "old" {
                worry_lvl
            } else {
                right.parse().unwrap()
            };

            let result = match operand {
                '+' => worry_lvl + other,
                '*' => worry_lvl * other,
                _ => unreachable!(),
            };

            result / 3
        }
    }

    fn get_test_from_str(test_string: Vec<&str>) -> impl Fn(usize) -> usize {
        let values = test_string
            .iter()
            .map(|line| line.split(' ').last().unwrap().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        move |worry_lvl: usize| -> usize {
            if worry_lvl % values[0] == 0 {
                values[1]
            } else {
                values[2]
            }
        }
    }
}

fn get_input() -> Vec<Monkey> {
    let input = fs::read_to_string(Path::new("./input/day11.input"))
        .expect("Something went wrong with the input");
    let descriptors = input.trim().split("\r\n\r\n").collect::<Vec<_>>();

    descriptors
        .iter()
        .map(|block| Monkey::new(block))
        .collect::<Vec<_>>()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut monkeys = get_input();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let execution = monkeys[i].execute();
            for (target, value) in execution {
                monkeys[target].inventory.push(value);
            }
        }
    }

    let mut inspections = monkeys
        .iter()
        .map(|monkey| monkey.inspection_performed)
        .collect::<Vec<_>>();

    inspections.sort();

    println!(
        "Level of monkey business is: {}",
        inspections.iter().rev().take(2).product::<usize>()
    );

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
