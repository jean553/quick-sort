[![Build Status](https://travis-ci.org/jean553/quick-sort.svg?branch=master)](https://travis-ci.org/jean553/quick-sort)

# quick-sort

Quick sort implementation in Rust.

## Table of contents

- [Usage](#usage)
- [Run the tests](#run-the-tests)
- [Generate documentation](#generate-documentation)
- [Algorithm](#algorithm)
    * [Generalities](#generalities)
    * [Left and right indices](#left-and-right-indices)
    * [Set the pivot index](#set-the-pivot-index)
- [Example](#example)

## Usage

```rust
let mut array: [u8; 4] = [10, 5, 7, 3];
qs::quick_sort(&mut array, 0, 3);
```

## Run the tests

```sh
cargo test
```

## Generate documentation

```bash
cargo rustdoc -- --no-defaults
```

## Algorithm

### Generalities

Quicksort is a divide and conquer algorithm.

Quicksort uses a pivot (specific array item):
all the items before the pivot must be lesser than the pivot,
all the items after the pivot must be higher than the pivot.

First, the pivot is the first array index.
The pivot is then set at the correct position
by exchanging values of some items until
all the items before the pivot are lesser than the pivot and
all the items after the pivot are higher than the pivot.

Once the pivot has been found, the array is divided into two sub-arrays
(each side of the pivot) and the same process is applied again recursively.

### `left` and `right` indices

There are two indices used to establish the pivot position.
The `left` index is used to browse the items from the left to the right,
the `right` index is used to browse the items from the right to the left.

### Set the `pivot index`

At the beginning, the `pivot index` is the first array item index (0).
The left index is the first index (most on the left),
and the right index is the last index (most on the right).

The `pivot index` is equal to the `left index`,
so we compare the `pivot index` value with the `right index` value.

If the `pivot index` value is higher than the `right index` values,
then the two values are inverted and the `pivot index`
becomes equal to the `right index`. Then, the `left index` is incremented
(would be decremented if the index was the `right index`),
and the process starts again by comparing the `left index` with the `pivot index`.

For the first comparison, if the `pivot index` value is lesser than the `right index`,
then the `right index` is decremented and the process restarts.

## Example

Let's take the following unsorted array:

```
4 3 7 2 1
```

The `left index` (L) is at the index 0.
The `right index` (R) is at the last index.
The `pivot` (P) is equal to the `left index`.

```
4 3 7 2 1
^       ^
L       R
P
```

As the `pivot` is equal to the `left index`, we compare the `pivot` with the `right index`.
Is `4` < `1` ? No, so the `left index` value and the `right index` value are inverted
and the `pivot` is now equal to the `right index`. The pivot is now equal to the `right index`,
so the `left index` is incremented.

```
1 3 7 2 4
  ^     ^
  L     R
        P
```

As the `pivot` is equal to the `right index`, we compare the `pivot` with the `left index`.
Is `3` < `4` ? Yes, so the `left index` is incremented.

```
1 3 7 2 4
    ^   ^
    L   R
        P
```

As the `pivot` is equal to the `right index`, we compare the `pivot` with the `left index`.
Is `7` < `4` ? No, so the `left index` value and the `right index` value are inverted
and the `pivot` is now equal to the `left index`. The `pivot` is now equal to the `left index`,
so the `right index` is decremented.

```
1 3 4 2 7
    ^ ^
    L R
    P
```

As the `pivot` is equal to the `left index`, we compare the `pivot` with the `right index`.
Is `4` < `2` ? No, so the `left index` value and the `right index` value are inverted
and the `pivot` is now equal to the `right index`. The `pivot` is now equal to the `right index`,
so the `left index` is incremented.

```
1 3 2 4 7
      ^
      L
      R
      P
```

The `left index` and the `right index` are equals, so `4` is at its final position
and divide the initial array into two sub-arrays. The left array is sorted first.

The `left index` (L) is at the index 0.
The `right index` (R) is at the last index.
The `pivot` (P) is equal to the `left index`.

```
1 3 2 - 7
^   ^
L   R
P
```

As the `pivot` is equal to the `left index`, we compare the `pivot` with the `right index`.
Is `1` < `2` ? Yes, so the `right index` is decremented.

```
1 3 2 - 7
^ ^
L R
P
```

As the `pivot` is equal to the `left index`, we compare the `pivot` with the `right index`.
Is `1` < `3` ? Yes, so the `right index` is decremented.

```
1 3 2 - 7
^
L
R
P
```

The `left index` and the `right index` are equals, so `1` is at its final position
and divide the initial array into two sub-arrays.

There is no left array, so the right array is sorted.

The `left index` (L) is at the index 0.
The `right index` (R) is at the last index.
The `pivot` (P) is equal to the `left index`.

```
- - - - 7
- 3 2 - -
  ^ ^
  L R
  P
```

As the `pivot` is equal to the `left index`, we compare the `pivot` with the `right index`.
Is `3` < `2` ? No, so the `left index` value and the `right index` value are inverted
and the `pivot` is now equal to the `right index`. The `pivot` is now equal to the `right index`,
so the `left index` is incremented.

```
- - - - 7
- 2 3 - -
    ^
    L
    R
    P
```

The `left index` and the `right index` are equals, so `3` is at its final position
and divide the initial array into two sub-arrays. The left array is sorted first.

The `left index` (L) is at the index 0.
The `right index` (R) is at the last index.
The `pivot` (P) is equal to the `left index`.

```
- - - - 7
- 2 - - -
  ^
  L
  R
  P
```

The `left index` and the `right index` are equals, so `2` is at its final position
and divide the initial array into two sub-arrays.

We now considere previous unsorted arrays.

The `left index` (L) is at the index 0.
The `right index` (R) is at the last index.
The `pivot` (P) is equal to the `left index`.

```
- - - - 7
        ^
        L
        R
        P
```

The `left index` and the `right index` are equals, so `7` is at its final position.

The final sorted array is:

```
1 2 3 4 7
```
