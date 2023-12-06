use anyhow::Result;
use itertools::Itertools;
use std::{collections::HashMap, fs, time::Instant};

#[derive(Clone, Eq, Hash, PartialEq, Debug, PartialOrd, Ord)]
enum Category {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Clone)]
struct MapRange {
    destination_range: RangeTuple,
    source_range: RangeTuple,
}
#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Clone)]
struct RangeTuple {
    start: usize,
    finish: usize,
}
impl RangeTuple {
    fn new(start: usize, finish: usize) -> Self {
        return RangeTuple { start, finish };
    }
    fn is_in_range(&self, num: &usize) -> bool {
        return self.start <= *num && self.finish >= *num;
    }
}

impl MapRange {
    fn new(input: &str) -> Self {
        let input = input.trim().split(" ").collect_vec();
        let range_len = input.get(2).unwrap().parse::<usize>().unwrap();
        let dest_start = input.get(0).unwrap().parse::<usize>().unwrap();
        let src_start = input.get(1).unwrap().parse::<usize>().unwrap();

        return Self {
            destination_range: RangeTuple::new(dest_start, dest_start + range_len - 1),
            source_range: RangeTuple::new(src_start, src_start + range_len - 1),
        };
    }
    fn get_dest_pos_from_src(&self, src_pos: usize) -> InRangeDecisionPosition {
        if self.source_range.is_in_range(&src_pos) {
            let offset = self.source_range.finish - src_pos;
            return InRangeDecisionPosition {
                is_in_range: true,
                position: self.destination_range.finish - offset,
            };
        }
        return InRangeDecisionPosition {
            is_in_range: false,
            position: src_pos,
        };
    }
}
#[derive(Clone, Debug)]
struct InRangeDecisionPosition {
    is_in_range: bool,
    position: usize,
}

fn parse_seed_ids_only(line: &str) -> Vec<usize> {
    let mut seeds: Vec<usize> = Vec::new();
    let colon_idx = line.find(":").unwrap();
    let line = &line[colon_idx + 1..];
    let trimmed_line = line.trim();
    trimmed_line
        .split(" ")
        .tuples()
        .for_each(|(start_seed_id, range)| {
            let start_seed_id = start_seed_id.parse::<usize>().unwrap();
            let range = range.parse::<usize>().unwrap();
            for id in start_seed_id..start_seed_id + range {
                seeds.push(id)
            }
        });
    return seeds;
}

fn match_category(line: &str) -> Category {
    return match line {
        "seed-to-soil map:" => Category::SeedToSoil,
        "soil-to-fertilizer map:" => Category::SoilToFertilizer,
        "fertilizer-to-water map:" => Category::FertilizerToWater,
        "water-to-light map:" => Category::WaterToLight,
        "light-to-temperature map:" => Category::LightToTemperature,
        "temperature-to-humidity map:" => Category::TemperatureToHumidity,
        "humidity-to-location map:" => Category::HumidityToLocation,
        _ => panic!("wrong line here {}", line),
    };
}

fn main() -> Result<()> {
    let now = Instant::now();
    let input_lines = fs::read_to_string("./inputs/day5.prod")?;
    let input_lines = input_lines.trim().split("\n\n").collect_vec();
    let mut category_maps: HashMap<Category, Vec<MapRange>> = HashMap::new();
    println!("reading ranges");
    input_lines.iter().skip(1).for_each(|map_and_data| {
        let map = map_and_data.split("\n").collect_vec();
        let category = match_category(map.get(0).unwrap());
        map.iter().skip(1).for_each(|range| {
            let map_range: MapRange = MapRange::new(range);
            match category_maps.get_mut(&category) {
                //TODO_HERE probably here we could to the comparison 
                //instead of pushing to the vec
                //for context look TODO comment below
                Some(ranges) => ranges.push(map_range),
                None => {
                    let _ = category_maps.insert(category.clone(), vec![map_range]);
                }
            }
        })
    });
    let mut seed_ids: Vec<usize> = parse_seed_ids_only(input_lines.get(0).unwrap());
    let mut min_location: usize = usize::MAX;
    println!("comparing seeds now");
    let mut seed_id_tmp: usize = 0;
    //TODO This can be optimized so well, for example it could be put into the 
    //part where ranges are extracted (look TODO_HERE comment)
    //but it's 1am, i want to go to sleep
    seed_ids.iter_mut().for_each(|seed_id| {
        seed_id_tmp = seed_id.clone();
        category_maps
            .iter_mut()
            .sorted()
            .for_each(|(category, ranges)| {
                for range in ranges {
                    let in_range_pos: InRangeDecisionPosition =
                        range.get_dest_pos_from_src(seed_id_tmp);
                    if in_range_pos.is_in_range {
                        //If it is in the range break the loop
                        seed_id_tmp = in_range_pos.position;
                        break;
                    }
                    //Loop over all ranges to check if it is there
                    // It is going to do some overwrite but that's to be though about later
                    // seed_id = &mut in_range_pos.position;
                    seed_id_tmp = in_range_pos.position;
                }
                if category == &Category::HumidityToLocation {
                    min_location = std::cmp::min(seed_id_tmp, min_location);
                }
            });
    });

    println!("Min location: {}", min_location);
    println!("In time {:?}", now.elapsed());

    Ok(())
}
