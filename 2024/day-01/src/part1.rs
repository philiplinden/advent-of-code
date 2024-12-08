use log::info;
use miette::{Error, Result};
use rayon::prelude::*;

pub fn process(input: &str, parallel: bool) -> Result<String> {
    // parse the input file as two lists of unsigned integers
    let (left, right) = parse_input(&input)?;

    // now sort each list
    let left_sorted = radix_sort(&left, parallel);
    let right_sorted = radix_sort(&right, parallel);
    let total_distance = calculate_total_distance(&left_sorted, &right_sorted);
    Ok(format!("{}", total_distance))
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
fn parse_input(input: &str) -> Result<(Vec<usize>, Vec<usize>), Error> {
    info!("Parsing input string as two lists of unsigned integers");
    let (left, right): (Vec<_>, Vec<_>) = input
        .lines()
        .filter_map(|line| {
            // each line is supposed to be two strings separated by three spaces
            line.split_once("   ")
                // if the split is successful, parse the strings as unsigned integers
                .map(|(left, right)| {
                    (
                        left.trim().parse::<usize>().ok(),
                        right.trim().parse::<usize>().ok(),
                    )
                })
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
/// This is a series implementation of radix sort.
///
/// Inspect each digit of each number and move it to the correct bin such that
/// the numbers for that digit are sorted in ascending order.
///
/// This radix sort uses counting sort as the stable sort for each digit.
/// We do this in parallel for each digit.
fn radix_sort(arr: &Vec<usize>, parallel: bool) -> Vec<usize> {
    // Determine the base (radix) as the smallest power of 2 greater than or
    // equal to the array length
    let base = optimal_base(arr.len());

    // Start with the smallest bit (2^0 = 1) and progressively move to larger
    // bit groups. This meta loop is not parallelizable.
    let mut exp: usize = 1;
    let mut sorted = arr.clone(); // clone here so we don't mutate the original
    while exp < base {
        sorted = if parallel {
            count_sort_parallel(&sorted, base, exp)
        } else {
            count_sort(&sorted, base, exp)
        };
        exp *= 2;
    }
    sorted
}

/// Find the smallest power of 2 greater than `n`.
fn optimal_base(n: usize) -> usize {
    (2 as usize).pow(n.next_power_of_two().trailing_zeros())
}

/// Sort the array by placing elements in buckets according to the specified bit
fn count_sort(arr: &[usize], base: usize, exp: usize) -> Vec<usize> {
    let n = arr.len();

    // Output will temporarily store the sorted elements
    let mut output: Vec<usize> = vec![0; n];
    // Count array to store the count of occurrences of each unique object
    let mut count: Vec<usize> = vec![0; base];

    // Count the occurrences of each digit in the current bit group
    for &num in arr.iter() {
        // Extract the digit in the current bit group
        let digit = (num / exp) % base;
        count[digit] += 1;
    }

    // Modify count array to store actual position of digits
    for i in 1..base {
        count[i] += count[i - 1];
    }

    // Build the output array
    for &num in arr.iter().rev() {
        let digit = (num / exp) % base;
        let index = count[digit as usize] - 1;
        output[index] = num;
        count[digit] -= 1;
    }

    output
}

/// Sort the array by placing elements in buckets according to the specified bit
/// using parallel iterators.
fn count_sort_parallel(arr: &[usize], base: usize, exp: usize) -> Vec<usize> {
    // Parallelize the counting of each digit
    let count: Vec<usize> = (0..base)
        .into_par_iter()
        .map(|digit| {
            arr.iter()
                .filter(|&&num| (num / exp) % base == digit)
                .count()
        })
        .collect();

    // The prefix sum array holds the starting index for each bucket in the final
    // sorted array.
    let prefix_sum = calculate_prefix_sum(&count);

    // Unpack the elements of `arr` into a sorted array using the `prefix_sum`
    // array to place the elements in the correct order.
    unpack_into_sorted_array(&arr, &prefix_sum, base, exp)
}

/// Calculate the prefix sum of the `count` array. Must be done sequentially.
fn calculate_prefix_sum(count: &[usize]) -> Vec<usize> {
    let mut prefix_sum = vec![0; count.len()];
    for i in 1..count.len() {
        prefix_sum[i] = prefix_sum[i - 1] + count[i - 1];
    }
    prefix_sum
}

/// Unpack the elements of `arr` into a sorted array using the `prefix_sum`
/// array to place the elements in the correct order.
/// TODO: This could be parallelized in theory but I couldn't get it to work. We
/// do the iteration sequentially for stability.
fn unpack_into_sorted_array(
    arr: &[usize],
    prefix_sum: &[usize],
    base: usize,
    exp: usize,
) -> Vec<usize> {
    // Mutable prefix sum to track placement
    let mut placement_index = prefix_sum.to_vec();

    // Create output array
    let mut output = vec![0; arr.len()];

    // Iterate through the original array
    for &num in arr.iter() {
        let digit = (num / exp) % base;
        let index = placement_index[digit];
        output[index] = num;
        placement_index[digit] += 1;
    }

    output
}

fn calculate_total_distance(left: &[usize], right: &[usize]) -> usize {
    let total_distance: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (*l as i32 - *r as i32).abs())
        .sum();
    total_distance as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const LEFT: [usize; 6] = [3, 4, 2, 1, 3, 3];
    const LEFT_SORTED: [usize; 6] = [1, 2, 3, 3, 3, 4];
    const RIGHT: [usize; 6] = [4, 3, 5, 3, 9, 3];
    const RIGHT_SORTED: [usize; 6] = [3, 3, 3, 4, 5, 9];

    #[test]
    fn test_parse_input() -> Result<(), Error> {
        let (left_numbers, right_numbers) = parse_input(
            "
3   4
4   3
2   5
1   3
3   9
3   3
        ",
        )?;
        assert_eq!(left_numbers, LEFT);
        assert_eq!(right_numbers, RIGHT);
        Ok(())
    }

    #[test]
    fn test_radix_sort() -> Result<(), std::io::Error> {
        let my_left_sorted = radix_sort(&LEFT.to_vec(), false);
        let my_right_sorted = radix_sort(&RIGHT.to_vec(), false);
        assert_eq!(my_left_sorted, LEFT_SORTED);
        assert_eq!(my_right_sorted, RIGHT_SORTED);
        Ok(())
    }

    #[test]
    fn test_radix_sort_parallel() -> Result<(), std::io::Error> {
        let my_left_sorted = radix_sort(&LEFT.to_vec(), true);
        let my_right_sorted = radix_sort(&RIGHT.to_vec(), true);
        assert_eq!(my_left_sorted, LEFT_SORTED);
        assert_eq!(my_right_sorted, RIGHT_SORTED);
        Ok(())
    }

    #[test]
    fn test_calculate_total_distance() -> Result<(), std::io::Error> {
        let total_distance = calculate_total_distance(&LEFT_SORTED, &RIGHT_SORTED);
        assert_eq!(total_distance, 11);
        Ok(())
    }
}
