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

    #[test]
    fn test_move_pivot_updates_indices() {

        let mut array = [5, 2, 6, 1, 3, 4];
        let mut left = 0;
        let mut right = array.len() - 1;
        let pivot = 0;

        qs::move_pivot_and_update_indices(
            &mut array,
            &mut left,
            &mut right,
            pivot,
        );

        assert_eq!(
            array,
            [4, 2, 6, 1, 3, 5],
            "Unexpected array order after pivot movement.",
        );
    }

    #[test]
    fn test_move_pivot_update_pivot_position() {

        let mut array = [5, 2, 6, 1, 3, 4];
        let mut left = 0;
        let mut right = array.len() - 1;
        let pivot = 0;

        let new_pivot = qs::move_pivot_and_update_indices(
            &mut array,
            &mut left,
            &mut right,
            pivot,
        );

        const EXPECTED_NEW_PIVOT: usize = 5;
        assert_eq!(
            new_pivot,
            EXPECTED_NEW_PIVOT,
            "Unexpected pivot index after pivot movement.",
        );
    }

    #[test]
    fn test_move_pivot_twice_update_indices_twice() {

        let mut array = [5, 2, 6, 1, 3, 4];
        let mut left = 0;
        let mut right = array.len() - 1;
        let mut pivot = 0;

        pivot = qs::move_pivot_and_update_indices(
            &mut array,
            &mut left,
            &mut right,
            pivot,
        );

        const EXPECTED_LEFT_FIRST_INDEX: usize = 0;
        assert_eq!(
            left,
            EXPECTED_LEFT_FIRST_INDEX,
            "Unexpected first value for the 'left' index.",
        );

        qs::move_pivot_and_update_indices(
            &mut array,
            &mut left,
            &mut right,
            pivot,
        );

        const EXPECTED_LEFT_SECOND_INDEX: usize = 1;
        assert_eq!(
            left,
            EXPECTED_LEFT_SECOND_INDEX,
            "Unexpected second value for the 'left' index.",
        );
    }

    #[test]
    fn test_move_pivot_twice_update_pivot_position() {

        let mut array = [5, 2, 6, 1, 3, 4];
        let mut left = 0;
        let mut right = array.len() - 1;
        let pivot = 0;

        let first_new_pivot = qs::move_pivot_and_update_indices(
            &mut array,
            &mut left,
            &mut right,
            pivot,
        );

        const EXPECTED_FIRST_NEW_PIVOT: usize = 5;
        assert_eq!(
            first_new_pivot,
            EXPECTED_FIRST_NEW_PIVOT,
            "Unexpected first pivot index after pivot movement.",
        );

        let second_new_pivot = qs::move_pivot_and_update_indices(
            &mut array,
            &mut left,
            &mut right,
            first_new_pivot,
        );

        const EXPECTED_SECOND_NEW_PIVOT: usize = 5;
        assert_eq!(
            second_new_pivot,
            EXPECTED_SECOND_NEW_PIVOT,
            "Unexpected second pivot index after pivot movement.",
        );
    }
}
