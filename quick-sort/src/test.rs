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
    fn test_move_pivot_from_left_to_right_invert_indices_values() {

        let mut array = [5, 2, 6, 1, 3, 4];
        let mut left = 0;
        let mut right = array.len() - 1;

        const PIVOT_INDEX_AT_LEFT: usize = 0;
        qs::move_pivot_and_update_indices(
            &mut array,
            &mut left,
            &mut right,
            PIVOT_INDEX_AT_LEFT,
        );

        assert_eq!(
            array,
            [4, 2, 6, 1, 3, 5],
            "Unexpected array order after pivot movement.",
        );
    }

    #[test]
    fn test_move_pivot_from_right_to_left_invert_indices_values() {

        let mut array = [5, 2, 6, 1, 3, 4];
        let mut left = 0;
        let mut right = array.len() - 1;

        const PIVOT_INDEX_AT_RIGHT: usize = 5;
        qs::move_pivot_and_update_indices(
            &mut array,
            &mut left,
            &mut right,
            PIVOT_INDEX_AT_RIGHT,
        );

        assert_eq!(
            array,
            [4, 2, 6, 1, 3, 5],
            "Unexpected array order after pivot movement.",
        );
    }

    #[test]
    fn test_move_pivot_update_pivot_position_to_the_right() {

        let mut array = [5, 2, 6, 1, 3, 4];
        let mut left = 0;
        let mut right = array.len() - 1;

        const PIVOT_POSITION_AT_LEFT: usize = 0;
        let new_pivot = qs::move_pivot_and_update_indices(
            &mut array,
            &mut left,
            &mut right,
            PIVOT_POSITION_AT_LEFT,
        );

        const EXPECTED_NEW_PIVOT: usize = 5;
        assert_eq!(
            new_pivot,
            EXPECTED_NEW_PIVOT,
            "Unexpected pivot index after pivot movement.",
        );
    }

    #[test]
    fn test_move_pivot_update_pivot_position_to_the_left() {

        let mut array = [5, 2, 6, 1, 3, 4];
        let mut left = 0;
        let mut right = array.len() - 1;

        const PIVOT_POSITION_AT_RIGHT: usize = 5;
        let new_pivot = qs::move_pivot_and_update_indices(
            &mut array,
            &mut left,
            &mut right,
            PIVOT_POSITION_AT_RIGHT,
        );

        const EXPECTED_NEW_PIVOT: usize = 0;
        assert_eq!(
            new_pivot,
            EXPECTED_NEW_PIVOT,
            "Unexpected pivot index after pivot movement.",
        );
    }

    #[test]
    fn test_move_pivot_keep_pivot_at_same_position() {

        let mut array = [4, 2, 6, 1, 3, 5];
        let mut left = 0;
        let mut right = array.len() - 1;

        const INITIAL_PIVOT_INDEX: usize = 0;
        let pivot = qs::move_pivot_and_update_indices(
            &mut array,
            &mut left,
            &mut right,
            INITIAL_PIVOT_INDEX,
        );

        assert_eq!(
            pivot,
            INITIAL_PIVOT_INDEX,
            "The pivot index has changed.",
        );
    }

    #[test]
    fn test_move_pivot_update_left_index() {

        const INITIAL_LEFT_INDEX: usize = 0;
        const EXPECTED_LEFT_INDEX: usize = 1;

        let mut array = [4, 2, 6, 1, 3, 5];
        let mut left = INITIAL_LEFT_INDEX;
        let mut right = array.len() - 1;

        const PIVOT_INDEX: usize = 5;
        qs::move_pivot_and_update_indices(
            &mut array,
            &mut left,
            &mut right,
            PIVOT_INDEX,
        );

        assert_eq!(
            left,
            EXPECTED_LEFT_INDEX,
            "Unexpected left index.",
        );
    }

    #[test]
    fn test_move_pivot_update_right_index() {

        const INITIAL_RIGHT_INDEX: usize = 5;
        const EXPECTED_RIGHT_INDEX: usize = 4;

        let mut array = [2, 2, 6, 1, 3, 6];
        let mut left = 0;
        let mut right = INITIAL_RIGHT_INDEX;

        const PIVOT_INDEX: usize = 0;
        qs::move_pivot_and_update_indices(
            &mut array,
            &mut left,
            &mut right,
            PIVOT_INDEX,
        );

        assert_eq!(
            right,
            EXPECTED_RIGHT_INDEX,
            "Unexpected right index.",
        );
    }
}
