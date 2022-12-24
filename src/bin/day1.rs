pub fn find_elf_with_max_calories(input: &str) -> u64 {
    let mut iter = input.trim().split("\n");
    let mut calories = Vec::new();
    
    let mut sum = 0;
    while let Some(chunk) = iter.next() {
        match chunk.parse::<u64>() {
            Ok(val) => sum += val,
            Err(_) => {
                calories.push(sum);
                sum = 0;
            }
        }
    }

    *calories.iter().max().unwrap()
}


pub fn find_top_three_elves_max_calories(input: &str) -> u64 {
    let mut iter = input.split("\n");
    let mut calories = Vec::new();
    
    let mut sum = 0;
    while let Some(chunk) = iter.next() {
        match chunk.parse::<u64>() {
            Ok(val) => sum += val,
            Err(_) => {
                calories.push(sum);
                sum = 0;
            }
        }
    }

    calories.sort();

    calories[(calories.len() - 3)..calories.len()].iter().fold(0, |acc, x| acc + x)
}

fn main() {
    let test = include_str!("day1.test");
    let input = include_str!("day1.input");

    println!("Test result first part: {}", find_elf_with_max_calories(test));
    println!("Input result first part: {}", find_elf_with_max_calories(input));


    println!("Test result first part: {}", find_top_three_elves_max_calories(test));
    println!("Input result second part: {}", find_top_three_elves_max_calories(input));
}
