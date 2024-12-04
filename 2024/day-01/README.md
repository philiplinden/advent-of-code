# [2024 day 1](https://adventofcode.com/2024/day/1)

## part 1

### problem

**order the lists into smallest pairs**

1. pair the smallest number in the left list (of [input1.txt](input1.txt)) with
   the smallest number in the right list (of [input1.txt](input1.txt))
2. pair the next smallest number in the left list with the next smallest number
   in the right list
3. repeat until one list is empty. They probably have the same length.

**find the distance between pairs**

1. for each pair, find the distance between the two numbers
2. sum the distances

### solution

I see two brute force solutions.

1. sort both lists in ascending order, then iterate through the lists by index
   and get the distance between the two numbers at each index. That's `O(logn)`
   for sorting and `O(n)` for the iteration, so `O(n+logn)`.
2. keep the two lists unsorted, and every iteration find the minimum value in
   each list. Find the distance between the two minimum values and add it to a
   sum. Remove the minimum value from its list. Repeat until one list is empty.
   This is `O(n^2)`, yikes.

If the lists are sorted, we can parallelize the difference calculation for each
index. The performance hit will come from sorting the lists.

- **Bucket sort (`O(1e5)`)** - since we know the numbers are unsigned integers, a
  bucket sort will sort the numbers in `O(n)` time. The integers are between 0
  and 100,000 so we would need 100,000 buckets, `n=1e5`.
- **Radix sort (`O(5e3+50)`)** - radix sort will sort the numbers in `O(n)`
  time, but it requires a lot of memory. `O(d(n+k))`, where d is number of
  digits and k is the base. We have 5 digits so `d=5` and it's decimal so
  `k=10`. There are `O(5(n+10))`. We have 1000 rows so `n=1e3`.
  `O(5(1e3+10)) = O(5e3+50)`. I guess this is parallelizable too.

I'm going to try radix sort.

Oh, I can't forget: I have to parse the input file too!
