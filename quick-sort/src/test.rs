mod test {

    use qs;

    #[test]
    fn test_quick_sort() {

        let source = [4, 2, 6, 5];
        let mut destination = [0, 0, 0, 0];

        qs::quick_sort(
            &source,
            &mut destination,
        );

        /* FIXME: #2 does not pass as the function is not defined yet */
        assert_eq!(
            destination,
            [2, 4, 5, 6],
            "The array is not sorted.",
        );
    }

    #[test]
    fn test_get_pivot_index() {

        let mut array = [5, 2, 6, 1, 3, 4];

        let pivot = qs::get_pivot_index(&mut array);

        const PIVOT_INDEX: usize = 4;
        assert_eq!(
            pivot,
            PIVOT_INDEX,
            "Unexpected pivot index.",
        );
    }

    #[test]
    fn test_pivot_position() {

        let mut array = [5, 2, 6, 1, 3, 4];

        qs::get_pivot_index(&mut array);

        assert_eq!(
            array,
            [4, 2, 3, 1, 5, 6], // expected pivot value is 5
            "Unexpected pivot position.",
        );
    }
}
