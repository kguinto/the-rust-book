use intavg::{get_mean, get_median, get_mode};

#[test]
fn test_all() {
    let nums = vec![5, 4, 2, 3, 1, 5, 0, 5, 0, 5];

    let mean = get_mean(&nums);
    let median = get_median(&nums);
    let mode = get_mode(&nums);

    assert_eq!(3.0, mean);
    assert_eq!(3.5, median);
    assert_eq!(5, mode);
}

#[test]
fn test_mean() {
    {
        let nums = vec![11, 12, 13, 14, 15];

        let mean = get_mean(&nums);
        assert_eq!(13.0, mean);
    }

    {
        let nums = vec![0, 0, 0, 0, 0];

        let mean = get_mean(&nums);
        assert_eq!(0.0, mean);
    }

    {
        let nums = vec![-2, 0, 0, 1, 1];

        let mean = get_mean(&nums);
        assert_eq!(0.0, mean);
    }

    {
        let nums = vec![];

        let mean = get_mean(&nums);
        assert_eq!(mean.is_nan(), true);
    }
}

#[test]
fn test_median() {
    {
        let nums = vec![1, 12, 13, 14, 9001];

        let median = get_median(&nums);
        assert_eq!(13.0, median);
    }

    {
        let nums = vec![-9000, -9000, -9000, 1, 2];

        let median = get_median(&nums);
        assert_eq!(-9000.0, median);
    }

    {
        let nums = vec![1, 2, 3, 3, 4, 5];

        let median = get_median(&nums);
        assert_eq!(3.0, median);
    }

    {
        let nums = vec![1, 2, 3, 4, 4, 5];

        let median = get_median(&nums);
        assert_eq!(3.5, median);
    }

    {
        let nums = vec![2, 4];

        let median = get_median(&nums);
        assert_eq!(3.0, median);
    }

    {
        let nums = vec![10, 0, 90000, 12];

        let median = get_median(&nums);
        assert_eq!(11.0, median);
    }

    {
        let nums = vec![];

        let median = get_median(&nums);
        assert_eq!(median.is_nan(), true);
    }
}

#[test]
fn test_mode() {
    {
        let nums = vec![11, 12, 13, 14, 15, 15];

        let mode = get_mode(&nums);
        assert_eq!(15, mode);
    }

    {
        let nums = vec![0];

        let mode = get_mode(&nums);
        assert_eq!(0, mode);
    }

    {
        let nums = vec![-2, 0, 0, 1, 1, -2, 1];

        let mode = get_mode(&nums);
        assert_eq!(1, mode);
    }

    {
        let nums = vec![];

        let mode = get_mode(&nums);
        assert_eq!(mode, 0);
    }
}
