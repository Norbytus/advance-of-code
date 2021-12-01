#[test]
fn test_count_increase() {
    let test_data: Vec<usize> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    assert_eq!(7, count_increase(&test_data));
}

#[test]
fn test_sliding_window_increase() {
    let test_data: Vec<usize> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    assert_eq!(5, sliding_window_increase(&test_data));
}

#[inline]
pub fn count_increase(data: &Vec<usize>) -> usize {
    let mut iter = data.into_iter();
    let mut v = iter.next().unwrap_or(&0);
    let mut counter = 0;

    while let Some(value) = iter.next() {
        if value > v {
            counter += 1;
        }
        v = value;
    }

    counter
}

#[inline]
pub fn sliding_window_increase(data: &Vec<usize>) -> usize {
    let mut iter = data.windows(3).into_iter();
    let mut v: usize = iter.next().unwrap_or(&[0, 0, 0]).into_iter().sum();
    let mut counter = 0;

    while let Some(value) = iter.next() {
        let sum = value.into_iter().sum();
        if sum > v {
            counter += 1;
        }
        v = sum;
    }

    counter
}
