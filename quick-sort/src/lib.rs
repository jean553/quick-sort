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
}

#[cfg(test)]
mod test;
