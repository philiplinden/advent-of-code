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

[Radix sort](https://brilliant.org/wiki/radix-sort/) is an algorithm that builds
on top of bucket sort or counting sort.

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

A simple rule of thumb to find an optimal base is to find the smallest power of
2 greater than the array length `n`. I found this in multiple places as
anecdotal evidence, but I never found a source that proved it.

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

### Implementing the radix sort

The numbers we read in are at most `u32`. Rust processes use `usize` for
operations but Rust also uses 32-bit integers for the `usize` type. So we can
omit `as usize` and `as u32` and allow them to be implicit. Much cleaner.

In series, we use loops to iterate through the digits and sort the numbers.

Fortunately, the sorting algorithm itself does not use comparisons so it is
parallelizable. When each iteration doesn't depend on the previous iteration,
we can parallelize that process too.

The parallel approach might actually be slower for this problem because in
computing terms the input array is pretty small. The overhead of spawning
threads could outweigh the benefits of parallelization.

```
$ cargo bench

day_01_bench         fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ part1_parallel    28.18 ms      │ 32.34 ms      │ 29.16 ms      │ 29.56 ms      │ 100     │ 100
╰─ part1_sequential  379.4 µs      │ 530.1 µs      │ 403.9 µs      │ 407.6 µs      │ 100     │ 100
```

### Calculating the sum of distances

This part is pretty straightforward. The distance between the numbers in the
left list and the right list is just the absolute difference between the two
numbers.

In linear algebra, this is just subtraction.

```math
\begin{bmatrix}
1 \\
2 \\
3 \\
3 \\
3 \\
4
\end{bmatrix}
-
\begin{bmatrix}
3 \\
3 \\
3 \\
4 \\
5 \\
9
\end{bmatrix}
=
\begin{bmatrix}
-2 \\
-1 \\
0 \\
-1 \\
-2 \\
-5
\end{bmatrix}
```

Then we take the absolute value of each element in the resulting vector, and
compute the sum.
