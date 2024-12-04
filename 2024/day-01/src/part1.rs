use rayon::prelude::*; // For parallel iterators
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

const INPUT_FILE: &str = "input1.txt";

#[tracing::instrument]
pub fn process() -> miette::Result<()> {
    // Read input numbers from the input files
    let (left_numbers, right_numbers) = read_numbers(INPUT_FILE)?;

    // Sort both lists using radix sort in parallel
    let (sorted_left, sorted_right) = rayon::join(
        || radix_sort_parallel(&left_numbers),
        || radix_sort_parallel(&right_numbers),
    );

    // Pair the sorted numbers and calculate the sum of distances
    let total_distance = calculate_total_distance(&sorted_left, &sorted_right);

    // Print the result
    println!("Total Distance: {}", total_distance);

    Ok(())
}

/// Reads numbers from a given file and returns a vector of u32
fn read_numbers<P: AsRef<Path>>(filename: P) -> io::Result<Vec<u32>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    // Collect each line, parse it to u32, and collect into a vector
    reader
        .lines()
        .filter_map(|line| line.ok()) // Ignore lines with errors
        .map(|line| line.trim().parse::<u32>().unwrap_or(0)) // Parse to u32, default to 0 on failure
        .collect()
}

// Helper function to parse numbers from a string slice
fn read_numbers_from_str(data: &str) -> Result<Vec<u32>> {
    todo!("implement read numbers from string")
}

/// Performs radix sort on a vector of u32 using parallel processing
fn radix_sort_parallel(numbers: &Vec<u32>) -> Vec<u32> {
    todo!("implement radix sort")
}

/// Calculates the total distance between paired numbers from two sorted lists
fn calculate_total_distance(left: &Vec<u32>, right: &Vec<u32>) -> u64 {
    todo!("implement calculate total distance")
}

#[cfg(test)]
mod tests {
    use super::*;
    use miette::Result;

    #[test]
    fn test_read_numbers() -> Result<()> {
        let (left_numbers, right_numbers) = read_numbers(
            "
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

    #[test]
    fn test_radix_sort() -> Result<()> {
        let numbers = vec![3, 4, 2, 1, 3, 3];
        let sorted = radix_sort_parallel(&numbers);
        assert_eq!(sorted, [1, 2, 3, 3, 3, 4]);
        Ok(())
    }

    #[test]
    fn test_calculate_total_distance() -> Result<()> {
        let left = vec![1, 2, 3, 3, 3, 4];
        let right = vec![3, 3, 3, 4, 5, 9];
        let total_distance = calculate_total_distance(&left, &right);
        assert_eq!(total_distance, 11);
        Ok(())
    }
}
