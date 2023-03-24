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

fn bubble_sort(arr: &mut [isize]) {
    for i in 0..arr.len() {
        for j in 0..(arr.len() - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[test]
fn test_binary_search() {
    let haystack = [1, 2, 3];
    let (low, high) = (0, haystack.len() - 1);
    assert_eq!(binary_search(3, &haystack, low, high), true);
    assert_eq!(binary_search(4, &haystack, low, high), false);
}

#[test]
fn test_bubble_sort() {
    let mut arr = vec![1, 7, 3, 2];
    bubble_sort(&mut arr);
    assert_eq!(arr, vec![1, 2, 3, 7]);
}
