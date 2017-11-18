mod test {

    use qs;

    #[test]
    fn test_quick_sort_with_small_array() {

        let mut source = [4, 2, 6, 5];
        let left = 0;
        let right = source.len() - 1;

        qs::quick_sort(
            &mut source,
            left,
            right,
        );

        assert_eq!(
            source,
            [2, 4, 5, 6],
            "The small array is not sorted.",
        );
    }

    #[test]
    fn test_quick_sort_with_long_array() {

        let mut source = [7, 12, 3, 1, 24, 26, 13, 0, 2, 8, 4, 9, 6];
        let left = 0;
        let right = source.len() - 1;

        qs::quick_sort(
            &mut source,
            left,
            right,
        );

        assert_eq!(
            source,
            [0, 1, 2, 3, 4, 6, 7, 8, 9, 12, 13, 24, 26],
            "The array is not sorted.",
        );
    }

    #[test]
    fn test_quick_sort_with_small_array_with_same_values() {

        let mut source = [4, 4, 3, 3, 2, 1, 2, 5, 5];
        let left = 0;
        let right = source.len() - 1;

        qs::quick_sort(
            &mut source,
            left,
            right,
        );

        assert_eq!(
            source,
            [1, 2, 2, 3, 3, 4, 4, 5, 5],
            "The array with identical values is not sorted.",
        );
    }

    #[test]
    fn test_move_pivot_from_left_to_right_invert_indices_values() {

        let mut array = [5, 2, 6, 1, 3, 4];
        let mut left = 0;
        let mut right = array.len() - 1;

        const PIVOT_INDEX_AT_LEFT: usize = 0;
        qs::update_indices(
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
        qs::update_indices(
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
        let new_pivot = qs::get_next_pivot_index(
            &array,
            left,
            right,
            PIVOT_POSITION_AT_LEFT,
        );

        qs::update_indices(
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
        let new_pivot = qs::get_next_pivot_index(
            &array,
            left,
            right,
            PIVOT_POSITION_AT_RIGHT,
        );

        qs::update_indices(
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
        let pivot = qs::get_next_pivot_index(
            &array,
            left,
            right,
            INITIAL_PIVOT_INDEX,
        );

        qs::update_indices(
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
        qs::update_indices(
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
        qs::update_indices(
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

    #[test]
    fn test_pivot_position_after_first_partitioning() {

        const INITIAL_PIVOT_INDEX: usize = 0;
        const EXPECTED_PIVOT_INDEX: usize = 4;

        let mut array = [5, 2, 6, 1, 3, 4];
        let mut left = 0;
        let mut right = array.len() - 1;
        let mut pivot = INITIAL_PIVOT_INDEX;

        while left != right {

            let next_pivot = qs::get_next_pivot_index(
                &array,
                left,
                right,
                pivot,
            );

            qs::update_indices(
                &mut array,
                &mut left,
                &mut right,
                pivot,
            );

            pivot = next_pivot;
        }

        assert_eq!(
            pivot,
            EXPECTED_PIVOT_INDEX,
            "Unexpected pivot index after the first partitioning.",
        );
    }

    #[test]
    fn test_sub_arrays_content_after_first_partitioning() {

        const INITIAL_ARRAY: [u8; 6] = [5, 2, 6, 1, 3, 4];
        const EXPECTED_FIRST_SUB_ARRAY: [u8; 4] = [4, 2, 3, 1];
        const EXPECTED_SECOND_SUB_ARRAY: [u8; 1] = [6];

        let mut array = INITIAL_ARRAY;
        let mut left = 0;
        let mut right = array.len() - 1;
        let mut pivot = 0;

        while left != right {

            let next_pivot = qs::get_next_pivot_index(
                &array,
                left,
                right,
                pivot,
            );

            qs::update_indices(
                &mut array,
                &mut left,
                &mut right,
                pivot,
            );

            pivot = next_pivot;
        }

        const FIRST_ARRAY_LEFT_INDEX: usize = 0;
        const FIRST_ARRAY_EXCLUDED_RIGHT_INDEX: usize = 4;
        assert_eq!(
            &array[FIRST_ARRAY_LEFT_INDEX..FIRST_ARRAY_EXCLUDED_RIGHT_INDEX],
            EXPECTED_FIRST_SUB_ARRAY,
            "Unexpected first sub-array after first partitioning.",
        );

        const SECOND_ARRAY_LEFT_INDEX: usize = 5;
        const SECOND_ARRAY_EXCLUDED_RIGHT_INDEX: usize = 6;
        assert_eq!(
            &array[SECOND_ARRAY_LEFT_INDEX..SECOND_ARRAY_EXCLUDED_RIGHT_INDEX],
            EXPECTED_SECOND_SUB_ARRAY,
            "Unexpected second sub-array after first partitioning.",
        );
    }

    #[test]
    fn test_pivot_position_after_second_partitioning() {

        const INITIAL_PIVOT_INDEX: usize = 0;
        const EXPECTED_PIVOT_INDEX: usize = 3;

        let mut array = [5, 2, 6, 1, 3, 4];
        let mut left = 0;
        let mut right = array.len() - 1;
        let mut pivot = INITIAL_PIVOT_INDEX;

        const FIRST_PARTITION_INDEX: usize = 0;
        const PARTITIONS_AMOUNT: usize = 2;
        for partition in FIRST_PARTITION_INDEX..PARTITIONS_AMOUNT {

            while left != right {

                let next_pivot = qs::get_next_pivot_index(
                    &array,
                    left,
                    right,
                    pivot,
                );

                qs::update_indices(
                    &mut array,
                    &mut left,
                    &mut right,
                    pivot,
                );

                pivot = next_pivot;
            }

            if partition != FIRST_PARTITION_INDEX {
                continue;
            }

            const SECOND_PARTITION_INITIAL_LEFT_INDEX: usize = 0;
            const SECOND_PARTITION_INITIAL_RIGHT_INDEX: usize = 3;
            const SECOND_PARTITION_INITIAL_PIVOT_INDEX: usize = 0;
            left = SECOND_PARTITION_INITIAL_LEFT_INDEX;
            right = SECOND_PARTITION_INITIAL_RIGHT_INDEX;
            pivot = SECOND_PARTITION_INITIAL_PIVOT_INDEX;
        }

        assert_eq!(
            pivot,
            EXPECTED_PIVOT_INDEX,
            "Unexpected pivot index after the second partitioning.",
        );
    }

    #[test]
    fn test_sub_arrays_content_after_second_partitioning() {

        const INITIAL_ARRAY: [u8; 6] = [5, 2, 6, 1, 3, 4];
        const EXPECTED_FIRST_SUB_ARRAY: [u8; 3] = [1, 2, 3];

        let mut array = INITIAL_ARRAY;
        let mut left = 0;
        let mut right = array.len() - 1;
        let mut pivot = 0;

        const FIRST_PARTITION_INDEX: usize = 0;
        const PARTITIONS_AMOUNT: usize = 2;
        for partition in FIRST_PARTITION_INDEX..PARTITIONS_AMOUNT {

            while left != right {

                let next_pivot = qs::get_next_pivot_index(
                    &array,
                    left,
                    right,
                    pivot,
                );

                qs::update_indices(
                    &mut array,
                    &mut left,
                    &mut right,
                    pivot,
                );

                pivot = next_pivot;
            }

            if partition != FIRST_PARTITION_INDEX {
                continue;
            }

            const SECOND_PARTITION_INITIAL_LEFT_INDEX: usize = 0;
            const SECOND_PARTITION_INITIAL_RIGHT_INDEX: usize = 3;
            const SECOND_PARTITION_INITIAL_PIVOT_INDEX: usize = 0;
            left = SECOND_PARTITION_INITIAL_LEFT_INDEX;
            right = SECOND_PARTITION_INITIAL_RIGHT_INDEX;
            pivot = SECOND_PARTITION_INITIAL_PIVOT_INDEX;
        }

        const FIRST_ARRAY_LEFT_INDEX: usize = 0;
        const FIRST_ARRAY_EXCLUDED_RIGHT_INDEX: usize = 3;
        assert_eq!(
            &array[FIRST_ARRAY_LEFT_INDEX..FIRST_ARRAY_EXCLUDED_RIGHT_INDEX],
            EXPECTED_FIRST_SUB_ARRAY,
            "Unexpected first sub-array after second partitioning.",
        );
    }

    #[test]
    fn test_step_by_step_sorting_process() {

        const INITIAL_ARRAY: [u8; 4] = [2, 4, 6, 5];
        const EXPECTED_SORTED_ARRAY: [u8; 4] = [2, 4, 5, 6];

        const INITIAL_LEFT_INDEX: usize = 1;
        const INITIAL_RIGHT_INDEX: usize = 3;
        const INITIAL_PIVOT: usize = 1;

        let mut array = INITIAL_ARRAY;
        let mut left = INITIAL_LEFT_INDEX;
        let mut right = INITIAL_RIGHT_INDEX;
        let mut pivot = INITIAL_PIVOT;

        qs::update_indices(
            &mut array,
            &mut left,
            &mut right,
            pivot,
        );

        const EXPECTED_FIRST_PIVOT_INDEX: usize = 1;
        assert_eq!(
            pivot,
            EXPECTED_FIRST_PIVOT_INDEX,
            "Unexpected pivot index.",
        );

        const EXPECTED_FIRST_LEFT_INDEX: usize = 1;
        assert_eq!(
            left,
            EXPECTED_FIRST_LEFT_INDEX,
            "Unexpected left index.",
        );

        const EXPECTED_FIRST_RIGHT_INDEX: usize = 2;
        assert_eq!(
            right,
            EXPECTED_FIRST_RIGHT_INDEX,
            "Unexpected right index.",
        );

        qs::update_indices(
            &mut array,
            &mut left,
            &mut right,
            pivot,
        );

        const EXPECTED_SECOND_PIVOT_INDEX: usize = 1;
        assert_eq!(
            pivot,
            EXPECTED_SECOND_PIVOT_INDEX,
            "Unexpected pivot index.",
        );

        const EXPECTED_SECOND_LEFT_INDEX: usize = 1;
        assert_eq!(
            left,
            EXPECTED_SECOND_LEFT_INDEX,
            "Unexpected left index.",
        );

        const EXPECTED_SECOND_RIGHT_INDEX: usize = 1;
        assert_eq!(
            right,
            EXPECTED_SECOND_RIGHT_INDEX,
            "Unexpected right index.",
        );

        /* left = right, so these indices have to be refreshed with new values,
           we test the right sub-array, so left is incremented and right is equal
           to the array size minus 1; as always, pivot is equal to the left index */
        const UPDATED_PIVOT: usize = 2;
        const UPDATED_LEFT_INDEX: usize = 2;
        const UPDATED_RIGHT_INDEX: usize = 3;
        pivot = UPDATED_PIVOT;
        left = UPDATED_LEFT_INDEX;
        right = UPDATED_RIGHT_INDEX;

        let next_pivot = qs::get_next_pivot_index(
            &array,
            left,
            right,
            pivot,
        );

        qs::update_indices(
            &mut array,
            &mut left,
            &mut right,
            pivot,
        );

        pivot = next_pivot;

        const FIRST_ARRAY_INDEX: usize = 0;
        const ARRAY_ITEMS_AMOUNT: usize = 4;
        assert_eq!(
            &array[FIRST_ARRAY_INDEX..ARRAY_ITEMS_AMOUNT],
            EXPECTED_SORTED_ARRAY,
            "Unexpected array.",
        );

        const EXPECTED_THIRD_PIVOT_INDEX: usize = 3;
        assert_eq!(
            pivot,
            EXPECTED_THIRD_PIVOT_INDEX,
            "Unexpected pivot index.",
        );

        const EXPECTED_THIRD_LEFT_INDEX: usize = 2;
        assert_eq!(
            left,
            EXPECTED_THIRD_LEFT_INDEX,
            "Unexpected left index.",
        );

        const EXPECTED_THIRD_RIGHT_INDEX: usize = 3;
        assert_eq!(
            right,
            EXPECTED_THIRD_RIGHT_INDEX,
            "Unexpected right index.",
        );
    }
}
