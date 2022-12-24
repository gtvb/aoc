// Find the item type that appears in both compartments of each rucksack. 
// What is the sum of the priorities of those item types?
//
// a -> z == 1 -> 26
// A -> Z == 27 -> 52
//
// to convert an lowercase letter, get its ASCII representation
// and subtract 96 from it.
// to convert an uppercase letter, get its ASCII representation
// and subtract 38 from it

pub fn find_priorities_first_part(input: &str) -> u32 {
    let mut sum = 0;
    input.trim().split("\n").for_each(|rucksack_contents| {
        let (first, second) = rucksack_contents.split_at(rucksack_contents.len() / 2);

        for ch in first.chars() {
            if !second.contains(ch) {
                continue;
            }

            if ch.is_lowercase() {
                sum += ch as u32 - 96;
                break
            } else {
                sum += ch as u32 - 38;
                break;
            }
        }
    });

    sum
}

pub fn find_priorities_for_groups_second_part(input: &str) -> u32 {
    let mut sum = 0;
    let input: Vec<_> = input.trim().split("\n").collect();

    for i in (0..input.len()).step_by(3) {
        for ch in input[i].chars() {
            if input[i + 1].contains(ch) && input[i + 2].contains(ch) {
                if ch.is_lowercase() {
                    sum += ch as u32 - 96;
                    break
                } else {
                    sum += ch as u32 - 38;
                    break;
                }
            }
        }
    }

    sum
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
        
        assert_eq!(find_priorities_first_part(&input1), 157);
        assert_eq!(find_priorities_first_part(&input2), 8153);
    }

    #[test]
    fn test_second_part() {
        let (input1, input2) = read_input_contents();

        assert_eq!(find_priorities_for_groups_second_part(&input1), 70);
        assert_eq!(find_priorities_for_groups_second_part(&input2), 2342);
    }
}
