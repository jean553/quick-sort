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
}
