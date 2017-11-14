# quick-sort

Quick sort implementation in Rust.

## Table of contents

- [Run the tests](#run-the-tests)
- [Generate documentation](#generate-documentation)
- [Algorithm](#algorithm)
    * [Generalities](#generalities)
    * [Left and right indices](#left-and-right-indices)
    * [Set the pivot index](#set-the-pivot-index)

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

Quicksort is a divide and conquer alogorithm.

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
