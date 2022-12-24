// Oponent:
// A - Rock
// B - Paper
// C - Scissors
//
// You: 
// X - Rock
// Y - Paper
// Z - Scissors
//
// Calculate total score:
// shape_score + round_outcome_score
//
// where shape_score:
// Rock - 1 point
// Paper - 2 points
// Scissors - 3 points
//
// where round_outcome_score:
// Lost - 0 points
// Draw - 3 points
// Win - 6 points
//
// For the second part:
// X - Lose
// Y - Draw Z - Win

use phf::phf_map;

static SCORES_FIRST_PART: phf::Map<&'static str, u32> = phf_map! {
    "A X" => 4,
    "A Y" => 8,
    "A Z" => 3,

    "B X" => 1,
    "B Y" => 5,
    "B Z" => 9,

    "C X" => 7,
    "C Y" => 2,
    "C Z" => 6,
};

static SCORES_SECOND_PART: phf::Map<&'static str, u32> = phf_map! {
    "A X" => 3,
    "A Y" => 4,
    "A Z" => 8,

    "B X" => 1,
    "B Y" => 5,
    "B Z" => 9,

    "C X" => 2,
    "C Y" => 6,
    "C Z" => 7,
};

pub fn find_highest_score_first_part(input: &str) -> u32 {
    input.trim().split("\n").fold(0, |acc, s| acc + SCORES_FIRST_PART[s])
}


pub fn find_highest_score_second_part(input: &str) -> u32 {
    input.trim().split("\n").fold(0, |acc, s| acc + SCORES_SECOND_PART[s])
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
        
        assert_eq!(find_highest_score_first_part(&input1), 15);
        assert_eq!(find_highest_score_first_part(&input2), 10310);
    }

    #[test]
    fn test_second_part() {
        let (input1, input2) = read_input_contents();

        assert_eq!(find_highest_score_second_part(&input1), 12);
        assert_eq!(find_highest_score_second_part(&input2), 14859);
    }
}
