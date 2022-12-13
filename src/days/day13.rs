use regex::Regex;
use std::cmp::Ordering;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(PartialEq, Eq, Clone, Debug)]
enum Packet {
    Value(usize),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(is_left_smaller(self, other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        is_left_smaller(self, other)
    }
}

use Packet::*;

fn is_left_smaller(left: &Packet, right: &Packet) -> Ordering {
    match (left, right) {
        (List(x), List(y)) => {
            if x.is_empty() && !y.is_empty() {
                return Ordering::Less;
            } else if !x.is_empty() && y.is_empty() {
                return Ordering::Greater;
            } else {
                for (elem_1, elem_2) in x.iter().zip(y.iter()) {
                    let order = is_left_smaller(elem_1, elem_2);
                    if order != Ordering::Equal {
                        return order;
                    }
                }
                if x.len() != y.len() {
                    return x.len().cmp(&y.len());
                }
            }
        }
        (List(_), Value(_)) => {
            return is_left_smaller(left, &List(vec![right.clone()]));
        }
        (Value(_), List(_)) => {
            return is_left_smaller(&List(vec![left.clone()]), right);
        }
        (Value(x), Value(y)) => {
            return x.cmp(y);
        }
    }

    Ordering::Equal
}

fn get_input() -> Vec<(Packet, Packet)> {
    let reg = Regex::new(r"(\d+|\[|\])").unwrap();
    let input = fs::read_to_string(Path::new("./input/day13.input"))
        .expect("Something went wrong with the input");
    input
        .trim()
        .split("\r\n\r\n")
        .map(|compare| {
            let packets = compare
                .lines()
                .map(|packet| {
                    let mut current_packet = vec![];
                    let mut heap = vec![];
                    for cap in reg.captures_iter(packet) {
                        let symbol = &cap[1];
                        if symbol == "[" {
                            heap.push(List(current_packet));
                            current_packet = vec![];
                        } else if symbol == "]" {
                            if heap.is_empty() {
                                break;
                            }
                            let full_packet = List(current_packet);
                            if let List(list) = heap.pop().unwrap() {
                                current_packet = list;
                                current_packet.push(full_packet);
                            } else {
                                current_packet = vec![];
                            }
                        } else {
                            let value = symbol.parse::<usize>().unwrap();
                            current_packet.push(Value(value));
                        }
                    }
                    if current_packet.len() == 1 {
                        current_packet[0].clone()
                    } else {
                        List(current_packet)
                    }
                })
                .collect::<Vec<Packet>>();
            (packets[0].clone(), packets[1].clone())
        })
        .collect::<Vec<(Packet, Packet)>>()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let couples = get_input();
    let mut sum = 0;
    for (index, (left, right)) in couples.iter().enumerate() {
        if is_left_smaller(left, right) == Ordering::Less {
            sum += index + 1;
        }
    }

    println!("Sum indice is: {}", sum);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut packets = get_input()
        .into_iter()
        .flat_map(|(x, y)| vec![x, y])
        .collect::<Vec<_>>();
    let divider_six = List(vec![List(vec![Value(6)])]);
    let divider_two = List(vec![List(vec![Value(2)])]);
    packets.push(divider_two.clone());
    packets.push(divider_six.clone());

    packets.sort();

    let mut two_pos = None;
    let mut six_pos = None;

    for (index, packet) in packets.iter().enumerate() {
        if *packet == divider_two {
            two_pos = Some(index + 1);
        }
        if *packet == divider_six {
            six_pos = Some(index + 1);
        }
    }

    if let (Some(x), Some(y)) = (two_pos, six_pos) {
        println!("Decoder key is: {}", x * y);
    }

    Ok(())
}
