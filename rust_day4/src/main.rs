use std::fs::read_to_string;

fn main() {
    let valid_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let passports_input = read_to_string("./src/input.txt").expect("cannot load file");
    let passports: Vec<&str> = passports_input.split("\n\n").collect();

    let part_one_valid_count: i32 = passports
        .iter()
        .map(|&passport| has_required_fields(passport, &valid_fields) as i32)
        .sum();
    println!("Part 1: Valid count is {}.", part_one_valid_count);
}

fn has_required_fields(passport: &str, required_fields: &Vec<&str>) -> bool {
    required_fields.iter().all(|&required| passport.contains(required))
}
