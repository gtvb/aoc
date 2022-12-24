// In how many assignment pairs does one range fully contain the other?

use std::collections::HashMap;

pub fn find_totally_overlapped_pair_count(input: &str) -> u32 {
    let mut count = 0;

    input.trim().split("\n").for_each(|ranges| {
        let (range1, range2) = ranges.split_once(",").unwrap();

        let range1 = range1.split_once("-").unwrap();
        let range2 = range2.split_once("-").unwrap();

        let (range1_lower, range1_upper) = ((range1.0).parse::<u32>().unwrap(), (range1.1).parse::<u32>().unwrap());
        let (range2_lower, range2_upper) = ((range2.0).parse::<u32>().unwrap(), (range2.1).parse::<u32>().unwrap());

        if (range1_lower <= range2_lower && range1_upper >= range2_upper)
            || (range2_lower <= range1_lower && range2_upper >= range1_upper) {
            count += 1;
        }
    });

    count
}

pub fn find_overlapping_ranges_count(input: &str) -> u32 {
    let mut count = 0;

    let iter = input.trim().split("\n");
    let mut map: HashMap<u32, u32> = HashMap::new();

    for ranges in iter {
        let (range1, range2) = ranges.split_once(",").unwrap();

        let range1 = range1.split_once("-").unwrap();
        let range2 = range2.split_once("-").unwrap();

        let (range1_lower, range1_upper) = ((range1.0).parse::<u32>().unwrap(), (range1.1).parse::<u32>().unwrap());
        let (range2_lower, range2_upper) = ((range2.0).parse::<u32>().unwrap(), (range2.1).parse::<u32>().unwrap());
       
        for i in range1_lower..=range1_upper {
            *map.entry(i).or_insert(0) += 1;
        }

        for j in range2_lower..=range2_upper {
            *map.entry(j).or_insert(0) += 1;
        }

        for (_, value) in map.iter() {
            if *value >= 2 {
                count += 1;
                break;
            }
        }

        map.clear();
    };

    count
}

#[cfg(test)]
mod test {
    use super::*;

    fn read_input_contents() -> (&'static str, &'static str) {
        (include_str!("./input_simple.txt"), include_str!("./input.txt"))
    }

    #[test]
    fn test_first_part() {
        let (input1, input2) = read_input_contents();
        
        assert_eq!(find_totally_overlapped_pair_count(&input1), 2);
        assert_eq!(find_totally_overlapped_pair_count(&input2), 509);
    }

    #[test]
    fn test_second_part() {
        let (input1, input2) = read_input_contents();

        assert_eq!(find_overlapping_ranges_count(&input1), 4);
        assert_eq!(find_overlapping_ranges_count(&input2), 870);
    }
}
