use std::fs;


struct Solution;
impl Solution {
    fn star_one(&self) {
        let file_contents = fs::read_to_string("./input.txt").unwrap();
        let split_contents = file_contents.trim_end().split("\r\n");

        let mut sum_calibration_values = 0;

        for line in split_contents {
            let characters: Vec<char> = line.chars().collect();

            let strings: Vec<String> = characters.iter().map(|character| character.to_string()).collect();

            let new_strings: Vec<i32> = strings.iter().filter(|string| string.parse::<i32>().is_ok()).map(|string| string.parse::<i32>().unwrap()).collect();

            // println!("{:?}", new_strings);

            let combined_string = format!("{}{}", new_strings[0], new_strings[new_strings.len() - 1]);

            sum_calibration_values += combined_string.parse::<i32>().unwrap();

            // digit2.parse::<i32>().is_ok()

            // let slices: Vec<&str> = strings.iter().map(|string| string.as_str()).collect();

            // println!("{:?}", slices);

            // let mut i = 0;
            // let mut j = slices.len() - 1;

            // let mut i_num_found = false;
            // let mut j_num_found = false;

            // let mut i_digit = "-1";
            // let mut j_digit = "-1";

            // while i <= j {
            //     let digit1: &str = slices[i];
            //     let digit2: &str = slices[j];

            //     if  !i_num_found && digit1.parse::<i32>().is_ok() {
            //         i_digit = digit1;
            //         i_num_found = true;
            //     }
            //     if  !j_num_found && digit2.parse::<i32>().is_ok() {
            //         j_digit = digit2;
            //         j_num_found = true;
            //     }
                
            //     if i_num_found && j_num_found {
            //         // println!("{}: {} {}", line, i_digit, j_digit);
            //         break;
            //     }
            //     i += 1;
            //     j -= 1;
            // }

            // // println!("{}: {} {}", line, i_digit, j_digit);

            
            // if i_digit == "-1" {
            //     i_digit = j_digit;
            // } else if j_digit == "-1" {
            //     j_digit = i_digit;
            // }

            // let combined_string_number = format!("{}{}", i_digit, j_digit);

            // println!("{}", combined_string_number); // prints out combined numbers

            // sum_calibration_values += combined_string_number.parse::<i32>().unwrap();

            // println!("{}", sum_calibration_values);

        }

        println!("{}", sum_calibration_values);
    }
}
// two pointers until we hit a string that we can parse on both pointers,
// combine them, and parse and add to overall tally
fn main() {
    let s = Solution;
    s.star_one();
}
