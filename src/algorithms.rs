fn binary_search(needle: isize, haystack: &[isize], low: usize, high: usize) -> bool {
    if low > high {
        return false;
    }

    let mid = (low + high) / 2;
    if haystack[mid] == needle {
        true
    } else if haystack[mid] < needle {
        binary_search(needle, haystack, mid + 1, high)
    } else {
        binary_search(needle, haystack, low, mid - 1)
    }
}

#[test]
fn test_binary_search() {
    let haystack = [1, 2, 3];
    let (low, high) = (0, haystack.len() - 1);
    assert_eq!(binary_search(3, &haystack, low, high), true);
    assert_eq!(binary_search(4, &haystack, low, high), false);
}
