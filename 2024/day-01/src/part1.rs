use aoc_tools::load_input_file;
use miette::Result;

pub fn process(input: &str) -> Result<String> {
    // load the input file into a string
    // TODO: handle errors. unwrap is ok for now
    let input = load_input_file(input).unwrap();

    // parse the input file as two lists of unsigned integers
    let (left, right) = parse_input(&input)?;

    Ok("".to_string())
}

fn parse_input(input: &str) -> Result<(Vec<u32>, Vec<u32>)> {
    Ok((vec![], vec![]))
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_read_numbers() -> Result<(), std::io::Error> {
//         let (left_numbers, right_numbers) = read_numbers(
//             "
// 3   4
// 4   3
// 2   5
// 1   3
// 3   9
// 3   3
//         ")?;
//         assert_eq!(left_numbers, [3, 4, 2, 1, 3, 3]);
//         assert_eq!(right_numbers, [4, 3, 5, 3, 9, 3]);
//         Ok(())
//     }

//     #[test]
//     fn test_radix_sort() -> Result<(), std::io::Error> {
//         let numbers = vec![3, 4, 2, 1, 3, 3];
//         let sorted = radix_sort_parallel(&numbers);
//         assert_eq!(sorted, [1, 2, 3, 3, 3, 4]);
//         Ok(())
//     }

//     #[test]
//     fn test_calculate_total_distance() -> Result<(), std::io::Error> {
//         let left = vec![1, 2, 3, 3, 3, 4];
//         let right = vec![3, 3, 3, 4, 5, 9];
//         let total_distance = calculate_total_distance(&left, &right);
//         assert_eq!(total_distance, 11);
//         Ok(())
//     }
// }
