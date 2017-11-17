mod qs {

    /// Takes one source array and sorts the result into a destination array.
    ///
    /// # Arguments:
    ///
    /// `source` - the source array to sort
    /// `destination` - the destination array used to output the sorted result
    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn quick_sort(
        source: &[u8],
        destination: &mut [u8],
    ) {
    }

    /// Returns pivot index and modifies items positions
    ///
    /// # Arguments:
    ///
    /// `array` - the source array for pivot calculation and pivot position set
    ///
    /// # Returns:
    ///
    /// the pivot index
    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn get_pivot_index(array: &mut [u8]) -> usize {

        /* TODO: to define */
        0
    }

    /// Returns the pivot new index and update left and right indices
    ///
    /// # Arguments:
    ///
    /// `array` - the source array for pivot calculation and pivot position set
    /// `left_index` - the current left index
    /// `right_index` - the current right index
    /// `pivot_index` - the current pivot index
    ///
    /// # Returns:
    ///
    /// updated pivot index
    #[allow(dead_code)]
    pub fn move_pivot_and_update_indices(
        array: &mut [u8],
        left_index: &mut usize,
        right_index: &mut usize,
        pivot_index: usize,
    ) -> usize {

        let next_pivot_index = get_next_pivot_index(
            array,
            *left_index,
            *right_index,
            pivot_index,
        );

        /* TODO: #24 use explicit functions for the following steps
           instead of defining everything at the same place */

        if pivot_index == *left_index &&
            array[pivot_index] > array[*right_index] {

            array.swap(
                *right_index,
                *left_index,
            );
        }
        else if pivot_index == *right_index &&
            array[pivot_index] > array[*left_index] {

            *left_index += 1;
        }
        else if pivot_index == *right_index &&
            array[pivot_index] < array[*left_index] {

            array.swap(
                *right_index,
                *left_index,
            );
        }
        else if pivot_index == *left_index &&
            array[pivot_index] < array[*right_index] {

            *right_index -= 1;
        }

        next_pivot_index
    }

    /// Calculate the next pivot index according to the current indices.
    ///
    /// # Arguments:
    ///
    /// `array` - the source array for pivot calculation and pivot position set
    /// `left_index` - the current left index
    /// `right_index` - the current right index
    /// `pivot_index` - the current pivot index
    ///
    /// # Returns:
    ///
    /// next expected pivot index
    #[allow(dead_code)]
    fn get_next_pivot_index(
        array: &[u8],
        left_index: usize,
        right_index: usize,
        pivot_index: usize,
    ) -> usize {

        if pivot_index == left_index &&
            array[pivot_index] > array[right_index] {
            return right_index;
        }
        else if pivot_index == right_index &&
            array[pivot_index] < array[left_index] {
            return left_index;
        }

        pivot_index
    }
}

#[cfg(test)]
mod test;
