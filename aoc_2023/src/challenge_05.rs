use pest::Parser;
use pest_derive::Parser;
use rayon::prelude::*;
use std::fs::read_to_string;
use std::ops::Range;
use std::path::Path;
use std::vec;

#[derive(Parser)]
#[grammar = "pest/challenge_5.pest"]
struct Challenge5Parser;

const SEEDS: [u64; 20] = [
    2149186375, 163827995, 1217693442, 67424215, 365381741, 74637275, 1627905362, 77016740,
    22956580, 60539394, 586585112, 391263016, 2740196667, 355728559, 2326609724, 132259842,
    2479354214, 184627854, 3683286274, 337630529,
];

pub fn part_1() -> Option<u64> {
    let basepath = Path::new("./src/inputs/input5/");

    let seed_soil = MappingCollection::parse(&basepath.join("seed_soil.txt"));
    let soil_fertilizer = MappingCollection::parse(&basepath.join("soil_fertilizer.txt"));
    let fertilizer_water = MappingCollection::parse(&basepath.join("fertilizer_water.txt"));
    let water_light = MappingCollection::parse(&basepath.join("water_light.txt"));
    let light_temperature = MappingCollection::parse(&basepath.join("light_temperature.txt"));
    let temperature_humidity = MappingCollection::parse(&basepath.join("temperature_humidity.txt"));
    let humidity_location = MappingCollection::parse(&basepath.join("humidity_location.txt"));

    return SEEDS
        .iter()
        .map(|&seed| {
            let soil = map_vector(seed, &seed_soil);
            let fertilizer = map_vector(soil, &soil_fertilizer);
            let water = map_vector(fertilizer, &fertilizer_water);
            let light = map_vector(water, &water_light);
            let temperature = map_vector(light, &light_temperature);
            let humidity = map_vector(temperature, &temperature_humidity);
            let location = map_vector(humidity, &humidity_location);
            return location;
        })
        .min();
}

pub fn part_2() -> Option<u64> {
    let basepath = Path::new("./src/inputs/input5/");

    let seed_soil = MappingCollection::parse(&basepath.join("seed_soil.txt"));
    let soil_fertilizer = MappingCollection::parse(&basepath.join("soil_fertilizer.txt"));
    let fertilizer_water = MappingCollection::parse(&basepath.join("fertilizer_water.txt"));
    let water_light = MappingCollection::parse(&basepath.join("water_light.txt"));
    let light_temperature = MappingCollection::parse(&basepath.join("light_temperature.txt"));
    let temperature_humidity = MappingCollection::parse(&basepath.join("temperature_humidity.txt"));
    let humidity_location = MappingCollection::parse(&basepath.join("humidity_location.txt"));

    let seed_start: Vec<u64> = SEEDS.iter().step_by(2).cloned().collect();
    let range: Vec<u64> = SEEDS.iter().skip(1).step_by(2).cloned().collect();

    let mut min_vals = vec![];
    for i in 0..seed_start.len() {
        let min_found = (seed_start[i]..(seed_start[i] + range[i]))
            .into_par_iter()
            .map(|seed| {
                let soil = map_vector(seed, &seed_soil);
                let fertilizer = map_vector(soil, &soil_fertilizer);
                let water = map_vector(fertilizer, &fertilizer_water);
                let light = map_vector(water, &water_light);
                let temperature = map_vector(light, &light_temperature);
                let humidity = map_vector(temperature, &temperature_humidity);
                let location = map_vector(humidity, &humidity_location);
                return location;
            })
            .min()
            .unwrap();

        min_vals.push(min_found);
    }

    min_vals.iter().min().copied()
}

fn map_vector(input: u64, collection: &MappingCollection) -> u64 {
    collection
        .maps
        .iter()
        .map(|map| map.map(input))
        .reduce(|acc, e| match (acc, e) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (_, _) => panic!("We should only have a single mapping"),
        })
        .unwrap() // unwrap the reduce operation
        .unwrap_or(input)
}

#[derive(Debug)]
struct Mapping {
    dst: Range<u64>,
    src: Range<u64>,
}

struct MappingCollection {
    maps: Vec<Mapping>,
}

impl Mapping {
    fn new(dst_start: u64, src_start: u64, len: u64) -> Mapping {
        Mapping {
            dst: dst_start..(dst_start + len),
            src: src_start..(src_start + len),
        }
    }

    fn map(&self, input: u64) -> Option<u64> {
        if self.src.contains(&input) {
            let offset = &input - self.src.start;
            return Some(self.dst.start + offset);
        }

        return None;
    }

    fn parse(input: &str) -> Mapping {
        let parse = Challenge5Parser::parse(Rule::map, input)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let mut src_start = 0;
        let mut dst_start = 0;
        let mut len = 0;
        for record in parse.into_inner() {
            match record.as_rule() {
                Rule::src => {
                    src_start = record.as_str().parse::<u64>().unwrap();
                }
                Rule::dst => {
                    dst_start = record.as_str().parse::<u64>().unwrap();
                }
                Rule::len => {
                    len = record.as_str().parse::<u64>().unwrap();
                }
                _ => unreachable!(),
            }
        }

        return Mapping::new(dst_start, src_start, len);
    }
}

impl MappingCollection {
    fn parse(input: &Path) -> MappingCollection {
        let input: String = read_to_string(input).expect("Error reading file");
        let maps = input.lines().map(Mapping::parse).collect();
        return MappingCollection { maps };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_1() {
    //     assert_eq!(Some(31599214), part_1());
    // }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(Some(20358599), part_2());
    // }
}
