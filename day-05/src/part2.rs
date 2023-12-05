use rayon::prelude::*;
use std::error::Error;

#[derive(Clone, Debug, PartialEq)]
pub enum MapKind {
    None,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl MapKind {
    pub fn from(label: &str) -> MapKind {
        match label {
            "seed-to-soil" => MapKind::Soil,
            "soil-to-fertilizer" => MapKind::Fertilizer,
            "fertilizer-to-water" => MapKind::Water,
            "water-to-light" => MapKind::Light,
            "light-to-temperature" => MapKind::Temperature,
            "temperature-to-humidity" => MapKind::Humidity,
            "humidity-to-location" => MapKind::Location,
            _ => MapKind::None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct SeedMapEntry {
    dst_start: u64,
    src_start: u64,
    length: u64,
}

impl SeedMapEntry {
    pub fn new(dst_start: u64, src_start: u64, length: u64) -> SeedMapEntry {
        SeedMapEntry {
            dst_start,
            src_start,
            length,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SeedMap {
    kind: MapKind,
    entries: Vec<SeedMapEntry>,
}

impl SeedMap {
    pub fn new(kind: MapKind) -> SeedMap {
        SeedMap {
            kind,
            entries: vec![],
        }
    }

    pub fn empty() -> SeedMap {
        SeedMap::new(MapKind::None)
    }

    pub fn next_id(&self, id: u64) -> Option<u64> {
        if let Some(entry) = self.entries.iter().find(|entry| {
            // find the entry that can contain the id
            id >= entry.src_start && id < entry.src_start + entry.length
        }) {
            // return destination plus offset
            let new_id = entry.dst_start + (id - entry.src_start);
            return Some(new_id);
        }
        // default to id if no matching map range
        Some(id)
    }
}

pub fn run(input: &str) -> Result<String, Box<dyn Error>> {
    let mut lines = input.lines();
    let mut seed_ranges: Vec<(u64, u64)> = vec![];
    let mut maps: Vec<SeedMap> = vec![];

    let mut current_map: SeedMap = SeedMap::empty();

    // parse seed id list
    if let Some(line) = &lines.next() {
        let seed_str = line.replace("seeds: ", "");
        let seed_split: Vec<&str> = seed_str.split(' ').collect();
        seed_ranges = seed_split
            .iter()
            .zip(seed_split.iter().skip(1))
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .map(|(_, (start, length))| {
                (
                    start.parse::<u64>().unwrap(),
                    length.parse::<u64>().unwrap(),
                )
            })
            .collect();
    }

    // skip first blank line
    lines.next();

    // loop through and build maps
    lines.for_each(|line| {
        if !line.is_empty() {
            if line.chars().next().unwrap().is_ascii_digit() {
                if let [dst_start, src_start, length] = line
                    .split(' ')
                    .map(|value| value.parse::<u64>().expect("should be valid map integer"))
                    .take(3)
                    .collect::<Vec<u64>>()[..]
                {
                    let entry = SeedMapEntry::new(dst_start, src_start, length);
                    current_map.entries.push(entry);
                }
            } else if let Some(label) = line.split(' ').next() {
                let kind = MapKind::from(label);
                if current_map.kind != MapKind::None {
                    maps.push(current_map.clone());
                }
                current_map = SeedMap::new(kind);
            }
        }
    });

    // add last populated map
    maps.push(current_map.clone());

    for map in &mut maps {
        map.entries.sort();
    }

    let min_id = seed_ranges
        .par_iter()
        .map(|&(start, length)| {
            let end = start + length;
            (start..end)
                .into_par_iter()
                .map(|i| {
                    let min = maps.iter().fold(i, |acc, map| {
                        if let Some(next_id) = map.next_id(acc) {
                            return next_id;
                        }
                        acc
                    });
                    min
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    Ok(min_id.to_string())
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn part2_example_input() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let expected = "46".to_string();

        assert_eq!(expected, run(input).expect("should return expected value"));
    }
}
