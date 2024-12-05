use miette::{Result, Error};
use log::info;

pub fn process(input: &str) -> Result<String> {
    // parse the input file as two lists of unsigned integers
    let (left, right) = parse_input(&input)?;

    // now sort each list
    // let left_sorted = radix_sort_parallel(&left);
    // let right_sorted = radix_sort_parallel(&right);

    Ok(format!("{:?}", (left, right)))
}

/// Parse the input starting with two lists of unsigned integers.
/// The input data looks like this:
/// ```
/// 3   4
/// 4   3
/// 2   5
/// 1   3
/// 3   9
/// 3   3
/// ```
/// And the output will be two lists of integers:
/// ```
/// [3, 4, 2, 1, 3, 3]
/// [4, 3, 5, 3, 9, 3]
/// ```
fn parse_input(input: &str) -> Result<(Vec<u32>, Vec<u32>), Error> {
    info!("Parsing input string as two lists of unsigned integers");
    let (left, right): (Vec<_>, Vec<_>) = input.lines()
        .filter_map(|line| {
            // each line is supposed to be two strings separated by three spaces
            line.split_once("   ")
                // if the split is successful, parse the strings as unsigned integers
                .map(|(left, right)| (left.trim().parse::<u32>().ok(), right.trim().parse::<u32>().ok()))
        })
        // filter out any lines that don't split into two unsigned integers
        .filter(|(left, right)| left.is_some() && right.is_some())
        // unwrap the optional unsigned integers, since we know only valid lines make it through
        .map(|(left, right)| (left.unwrap(), right.unwrap()))
        // unzip the tuples into two vectors
        .unzip();

    Ok((left, right))
}

/// Sort a vector of unsigned integers in ascending order.
/// This is a parallel implementation of radix sort.
///
/// Inspect each digit of each number and move it to the correct bin such that
/// the numbers for that digit are sorted in ascending order.
///
/// This radix sort uses counting sort as the stable sort for each digit.
/// We do this in parallel for each digit.
fn radix_sort_parallel(numbers: &[u32]) -> Vec<u32> {
    let mut numbers = numbers.to_vec();
    numbers.sort();
    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() -> Result<(), Error> {
        let (left_numbers, right_numbers) = parse_input("
3   4
4   3
2   5
1   3
3   9
3   3
        ")?;
        assert_eq!(left_numbers, [3, 4, 2, 1, 3, 3]);
        assert_eq!(right_numbers, [4, 3, 5, 3, 9, 3]);
        Ok(())
    }
}

    #[test]
    fn test_radix_sort() -> Result<(), std::io::Error> {
        let numbers = vec![3, 4, 2, 1, 3, 3];
        let sorted = radix_sort_parallel(&numbers);
        assert_eq!(sorted, [1, 2, 3, 3, 3, 4]);
        Ok(())
    }

//     #[test]
//     fn test_calculate_total_distance() -> Result<(), std::io::Error> {
//         let left = vec![1, 2, 3, 3, 3, 4];
//         let right = vec![3, 3, 3, 4, 5, 9];
//         let total_distance = calculate_total_distance(&left, &right);
//         assert_eq!(total_distance, 11);
//         Ok(())
//     }
// }
