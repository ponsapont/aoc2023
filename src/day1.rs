#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

#[aoc(day1, part1)]
pub fn day2part1(input: &[String]) -> u32 {
    let numbers: Vec<u32> = input
        .iter()
        .map(|l| {
            let only_nums: Vec<char> = l
                .chars()
                .filter(|c| c.to_string().parse::<u32>().is_ok())
                .collect();
            let (a, b) = if only_nums.len() == 1 {
                (only_nums[0], only_nums[0])
            } else if only_nums.is_empty() {
                ('0', '0')
            } else {
                (*only_nums.first().unwrap(), *only_nums.last().unwrap())
            };
            let num = format!("{}{}", a, b);
            num.to_string().parse::<u32>().unwrap()
        })
        .collect();
    // println!("{:#?}", numbers);
    numbers.into_iter().reduce(|acc, x| acc + x).unwrap()
}

#[aoc(day1, part2)]
pub fn day2part2(input: &[String]) -> u32 {
    let input: Vec<String> = input.iter().map(|line| replace_numbers(line)).collect();
    // println!("Input: {:#?}", input);
    day2part1(&input)
}

/// Replaces the numbers from the given line. Occurrences need to be applied from longest to shortest
fn replace_numbers(line: &str) -> String {
    line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}
