use regex::Regex;
static FILENAME: &str = "input/data";

fn main() {
    let data = std::fs::read_to_string(FILENAME).expect("could not read file");
    let passports = data.split("\n\n").collect::<Vec<&str>>();
    println!("part one: {}", part_one(&passports));
    println!("part two: {}", part_two(&passports));
}

fn part_one(passports: &[&str]) -> usize {
    let keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    passports
        .iter()
        .filter(|pass| keys.iter().all(|key| pass.contains(key)))
        .count()
}

fn part_two(passports: &[&str]) -> usize {
    let reg_exp = [
        r"byr:(?:19[2-9][0-9]|200[0-2])",
        r"iyr:20(?:1[0-9]|20)",
        r"eyr:20(?:2[0-9]|30)",
        r"hgt:(?:1[5-8][0-9]cm|19[0-3]cm|59in|6[0-9]in|7[0-6]in)",
        r"hcl:#[0-9|a-f]{6}",
        r"ecl:(?:amb|blu|brn|gry|grn|hzl|oth)",
        r"pid:[0-9]{9}(?:\n| |$)",
    ]
    .iter()
    .map(|re_str| Regex::new(re_str).expect("rgx error"));
    passports
        .iter()
        .filter(|pass| reg_exp.clone().all(|re| re.is_match(pass)))
        .count()
}

mod tests {
    #[test]
    fn test_part_one() {
        let data = std::fs::read_to_string("input/data").expect("could not read file");
        let passports = data.split("\n\n").collect::<Vec<&str>>();
        assert_eq!(190, super::part_one(&passports));
    }

    #[test]
    fn test_part_two() {
        let data = std::fs::read_to_string("input/data").expect("could not read file");
        let passports = data.split("\n\n").collect::<Vec<&str>>();
        assert_eq!(121, super::part_two(&passports));
    }
}
