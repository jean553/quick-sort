# quick-sort

Quick sort implementation in Rust.

## Table of contents

- [Run the tests](#run-the-tests)
- [Generate documentation](#generate-documentation)
- [Algorithm](#algorithm)
    * [Generalities](#generalities)

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
