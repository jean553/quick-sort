mod qs {

    /// Takes one array and sorts it using quick sort algorithm.
    ///
    /// # Arguments:
    ///
    /// `array` - the array to sort
    #[allow(dead_code)]
    pub fn quick_sort(
        array: &mut [u8],
        mut left: usize,
        mut right: usize,
    ) {

        let mut pivot = left;
        let previous_left = left;
        let previous_right = right;

        while left != right {

            let next_pivot_index = get_next_pivot_index(
                array,
                left,
                right,
                pivot,
            );

            update_indices(
                array,
                &mut left,
                &mut right,
                pivot,
            );

            pivot = next_pivot_index;
        }

        if left == previous_left && right == previous_right {
            return;
        }

        quick_sort(
            array,
            previous_left,
            pivot,
        );

        if pivot != array.len() - 1 {
            pivot += 1;
        }

        quick_sort(
            array,
            pivot,
            previous_right,
        );
    }

    /// Updates left and right indices
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
    pub fn update_indices(
        array: &mut [u8],
        left_index: &mut usize,
        right_index: &mut usize,
        pivot_index: usize,
    ) {

        if pivot_index == *left_index &&
            array[pivot_index] > array[*right_index] ||
            pivot_index == *right_index &&
            array[pivot_index] < array[*left_index] {

            array.swap(
                *right_index,
                *left_index,
            );
        }
        else if pivot_index == *right_index &&
            array[pivot_index] >= array[*left_index] {

            *left_index += 1;
        }
        else if pivot_index == *left_index &&
            array[pivot_index] <= array[*right_index] {

            *right_index -= 1;
        }
    }

    /// Calculates the next pivot index according to the current indices.
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
    pub fn get_next_pivot_index(
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
