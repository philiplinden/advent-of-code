# [2024 day 1](https://adventofcode.com/2024/day/1)

## problem

**order the lists into smallest pairs**

1. pair the smallest number in the left list (of [input1.txt](input1.txt)) with
   the smallest number in the right list (of [input1.txt](input1.txt))
2. pair the next smallest number in the left list with the next smallest number
   in the right list
3. repeat until one list is empty. They probably have the same length.

**find the distance between pairs**

1. for each pair, find the distance between the two numbers
2. sum the distances

## solution

### Choosing a sorting algorithm

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

- **Bucket sort (`O(n+k)`)** - since we know the numbers are unsigned integers,
  a bucket sort will sort the numbers in `O(n+k)` time. The integers are between
  0 and 100,000 so we would need 100,000 buckets, `n=1e5` I guess?
- **Counting sort (`O(n)`)** - since we know the range of the numbers
  we can use counting sort, which is `O(n)`. For large bases like `u32` this
  would consume half of my computer's memory, but maybe I can be smarter about
  this by using a base that's smaller than `u32`.
- **Radix sort (`O(d(n+k))`)** - radix sort will sort the numbers in `O(n)`
  time, but it requires a lot of memory. `O(d(n+k))`, where d is number of
  digits and k is the base. We have 5 digits so `d=5` and it's decimal so
  `k=10`. There are `O(5(n+10))`. We have 1000 rows so `n=1e3`. `O(5(1e3+10)) =
  O(5e3+50)`. This is apparently parallelizable.

A question remains: is base 10 the best choice for the base? We'll get to that
later.

#### Bucket sort

The bucket sort algorithm is as follows:

1. Create an array of `k` empty arrays. There is one for each value in our base.
2. Scan through the input array and examine the value of the element's least
   significant digit. Use this value as an index to determine which bucket the
   element should go in.
3. Now "rebuild" the array from the buckets. Pop the elements from the buckets
   in order (from the bottom, FIFO!). The array isn't sorted yet, so we repeat.
4. Repeat the process for the next most significant digit.
5. Continue until all digits have been processed. The array is now sorted.

#### Counting sort

[Why is Radix Sort so Fast? Part 2 Radix Sort](https://youtu.be/ujb2CIWE8zY)

The counting sort algorithm is as follows:

1. Start with an array of values in base `k`. Get the value of this digit for each
   element in the list. We should now have a list of numbers between 0 and `k-1`.
3. Count how many occurrences of each number in our base `k` show up.
4. We now "rebuild" the array from the counts. Make a new list and use the counts
   to place each number in the correct position in a new list. So if we counted
   3 zeros, put three zeros at the front of the new list.
5. The numbers for this digit have been sorted (without any comparisons 🤯).

#### Radix sort

Here's a couple tutorials on radix sort:
- [Radix Sort Algorithm Introduction in 5 Minutes](https://www.youtube.com/watch?v=XiuSW_mEn7g)
- [Radix Sort Part 1 - Intro to Parallel Programming](https://www.youtube.com/watch?v=dPwAA7j-8o4)

Radix sort is an algorithm that builds on top of bucket sort or counting sort.

1. Start with the least significant digit.
2. Apply the counting sort algorithm to all the elements for this digit.
3. Move to the next most significant digit and repeat the process.
4. Continue until all digits have been processed.

### Choosing a base

Using a larger base increases the "space" required for the sort, but decreases
the number of digits required to sort the numbers, which corresponds to a
decrease in the number of times we need to apply the counting sort. All that is
to say that the base `k` should be smaller than array size `n` for this to be
efficient.

To calculate `d` from `k`, `d = ceil(bit_width / log2(k))`. For example:
- Base 2: `d = ceil(32 / 1) = 32`
- Base 32: `d = ceil(32 / 5) = 7`
- Base 256: `d = ceil(32 / 8) = 4`

Maybe I could use a base 2 radix sort since I'm parsing the input as `u32`
values, so then `k=2` and `d=32`, therefore `O(32(1e3+2)) = O(32064)`. That's
worse. A base 32 radix sort would be `k=32, d=7`, so `O(7(1e3+32)) = O(7224)`.
If `k=256` then `d=4`, so `O(4(1e3+256)) = O(4224)`. This is the best one so
far in terms of time complexity but how does it perform with memory usage?

Each unsigned integer is 4 bytes, so for 1000 elements thats 4000 bytes. The
memory footprint scales as `n * 2^k`. A `u32` would require `2^32 * 4000` bytes,
or 16 GB! `u8` would require `2^8 * 4000` bytes, or 1024000 bytes. Rust lets me
use `u64`, `u32`, `u16`, or `u8`, so that is about as far as I can go. `u8::MAX`
is 255, so we're back at `k=256`, nice.

### Parsing the input file

The input file is read as a string using `include_str!()`. In order to parse the
contents of the file into two lists of unsigned integers, I split the string on
three spaces, then parse the two strings as unsigned integers. The input file
always has 3 spaces between the two numbers, and there are equal numbers of
numbers in each list, so this shouldn't fail. Rust makes us account for the
error case, though, where the split could result in `Some` or `None` for each
element of the tuple.

After splitting the lines, we filter out any lines that contain `None` for either
element of the tuple. Then we unwrap the `Some` values since we know only valid
lines make it through.

Finally, we unzip the tuples into two vectors.

### Implementing the parallel radix sort
