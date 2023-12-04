fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    print!("{}", output);
}

fn get_calibration(input: &str) -> String {
    let mut digit_list = "".to_string();
    println!("input {}", input);
    for char in input.chars() {
        if "1234567890".contains(char) {
            digit_list += &char.to_string();
        }
    }

    let output = digit_list.chars().next().unwrap().to_string()
        + &digit_list.chars().last().unwrap().to_string();

    println!("output {}", output);

    output
}

fn part1(input: &str) -> i32 {
    print!("{}", input);
    let mut output: i32 = 0;
    for line in input.lines() {
        output += get_calibration(line).parse::<i32>().unwrap();
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibration() {
        let result = get_calibration("1abc2");
        assert_eq!(result, "12");

        let result2 = get_calibration("pqr3stu8vwx");
        assert_eq!(result2, "38");

        let result3 = get_calibration("a1b2c3d4e5f");
        assert_eq!(result3, "15");

        let result4 = get_calibration("treb7uchet");
        assert_eq!(result4, "77");
    }

    #[test]
    fn test_part1() {
        let result5 = part1(
            "1abc2\n\
            pqr3stu8vwx\n\
            a1b2c3d4e5f\n\
            treb7uchet\n\
            ",
        );

        assert_eq!(result5, 142);
    }
}
