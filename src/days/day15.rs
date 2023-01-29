use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::Path;

type Coordinate = (i64, i64);

struct Sensor {
    position: Coordinate,
    beacon: Coordinate,
    range: i64,
}

impl Sensor {
    fn new(position: Coordinate, beacon: Coordinate) -> Self {
        let range = (position.0 - beacon.0).abs() + (position.1 - beacon.1).abs();
        Sensor {
            position,
            beacon,
            range,
        }
    }
    fn cannot_be_beacon(&self, ping: Coordinate, exclude_own: bool) -> bool {
        let range = (self.position.0 - ping.0).abs() + (self.position.1 - ping.1).abs();
        let in_range = range <= self.range;
        if exclude_own {
            in_range && (ping != self.beacon)
        } else {
            in_range
        }
    }
}

// See https://github.com/Crazytieguy/advent-of-code/blob/2e01e7a016bb5f270d4878b53ce3ef46beb3625b/2022/src/bin/day15/main.rs for a faster way to do it

fn get_input() -> Vec<Sensor> {
    let reg = Regex::new(r"(-?\d+).*?(-?\d+).*?(-?\d+).*?(-?\d+)").unwrap();
    let mut beacons = vec![];

    let input = fs::read_to_string(Path::new("./input/day15.input"))
        .expect("Something went wrong with the input");

    for read in reg.captures_iter(&input) {
        let raw_values = [&read[1], &read[2], &read[3], &read[4]];
        let values = raw_values
            .into_iter()
            .map(str::parse::<i64>)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        beacons.push(Sensor::new((values[0], values[1]), (values[2], values[3])));
    }

    beacons
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let target_y = 2_000_000;

    let mut sensors = get_input();
    let mut range_x = (i64::MAX, i64::MIN);
    for sensor in sensors.iter() {
        range_x = (
            range_x
                .0
                .min(sensor.position.0 as i64 - sensor.range as i64),
            range_x
                .1
                .max(sensor.position.0 as i64 + sensor.range as i64),
        );
    }

    sensors.retain(|sensor| {
        sensor.position.1 + sensor.range >= target_y && sensor.position.1 - sensor.range <= target_y
    });

    println!("Sensors in range: {}", sensors.len());

    let covered = (range_x.0..=range_x.1)
        .into_iter()
        .filter(|x| {
            sensors
                .iter()
                .any(|sensor| sensor.cannot_be_beacon((*x, target_y), true))
        })
        .count();

    println!("Covered tiles on row {}: {}", target_y, covered);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let max = 4_000_000;
    let sensors = get_input();
    let mut beacon_coordinates = (0, 0);

    'outer: for y in 0..=max {
        let mut x = 0;
        let mut old_x;
        while x <= max {
            old_x = x;
            let mut found = true;
            for sensor in sensors.iter() {
                if sensor.cannot_be_beacon((x, y), false) {
                    x = sensor.position.0 + (sensor.range - (sensor.position.1 - y).abs()) + 1;
                    found = false;
                    break;
                }
            }
            if found {
                beacon_coordinates = (x, y);
                break 'outer;
            }
            if old_x == x {
                println!("Infinite loop?");
            }
        }
    }

    println!(
        "Distress beacon tuning frequency: {}",
        beacon_coordinates.0 * 4_000_000 + beacon_coordinates.1
    );

    Ok(())
}
