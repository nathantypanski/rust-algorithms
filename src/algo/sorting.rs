use std::ptr;
use std::mem;
use std::vec::Vec;

/// Implementation of insertion sort.
/// CLRS pg 18. `O(n^2)`.
fn insertion_sort<T: Ord>(mut a: Vec<T>) -> Vec<T> {
    // Break it apart so we can do tricky memory things.
    let len = a.len();
    let cap = a.capacity();
    let p = a.as_mut_ptr();
    unsafe {
        // Stop the memory manager from calling the destructor.
        mem::forget(a);
        for j in range(1, len as int) {
            let key_offset = p.offset(j) as *const T;
            let key = ptr::read(key_offset);
            let mut i = j - 1;
            while i > -1 && *p.offset(i) > key {
                let other_offset = p.offset(i) as *const T;
                ptr::write(p.offset(i + 1), ptr::read(other_offset));
                i = i - 1;
            }
            ptr::write(p.offset(i+1), key);
        }
        Vec::from_raw_parts(len, cap, p)
    }
}

/// Implementation of selection sort.
/// CLRS pg 29. `O(n^2)`.
fn selection_sort<T: Ord>(mut a: Vec<T>) -> Vec<T> {
    let len = a.len();
    let cap = a.capacity();
    let p = a.as_mut_ptr();
    unsafe {
        mem::forget(a);
        for j in range(0, (len - 1) as int) {
            let mut smallest = j;
            for i in range(j + 1, (len) as int) {
                let elem = ptr::read(p.offset(i) as *const T);
                let smallest_elem = ptr::read(p.offset(smallest) as *const T);
                if elem < smallest_elem {
                    smallest = i;
                }
            }
            if smallest != j {
                ptr::swap(p.offset(smallest), p.offset(j));
            }
        }
        Vec::from_raw_parts(len, cap, p)
    }
}

#[cfg(test)]
mod tests {
    use sorting;

    #[test]
    fn test_insertion_sort_example() {
        // Example from CLRS pg 18.
        let mut sorted = vec!(5i32, 2, 4, 6, 1, 3);
        sorted = sorting::insertion_sort(sorted);
        assert_eq!(sorted, vec!(1i32, 2, 3, 4, 5, 6));
    }

    #[test]
    fn test_insertion_sort_duplicates() {
        let mut sorted = vec!(5i32, 5, 4);
        sorted = sorting::insertion_sort(sorted);
        assert_eq!(sorted, vec!(4, 5, 5));
    }

    #[test]
    fn test_insertion_sort_single_element() {
        let mut sorted = vec!(0i32);
        sorted = sorting::insertion_sort(sorted);
        assert_eq!(sorted, vec!(0));
    }

    #[test]
    fn test_insertion_sort_empty() {
        let mut sorted = vec!();
        sorted = sorting::insertion_sort::<i32>(sorted);
        assert_eq!(sorted, vec!());
    }

    #[test]
    fn test_selection_sort_example() {
        // Example from CLRS pg 18 (originally for insertion sort).
        let mut sorted = vec!(5i32, 2, 4, 6, 1, 3);
        sorted = sorting::selection_sort(sorted);
        assert_eq!(sorted, vec!(1i32, 2, 3, 4, 5, 6));
    }

    #[test]
    fn test_selection_sort_duplicates() {
        let mut sorted = vec!(5i32, 5, 4);
        sorted = sorting::selection_sort(sorted);
        assert_eq!(sorted, vec!(4, 5, 5));
    }

    #[test]
    fn test_selection_sort_single_element() {
        let mut sorted = vec!(0i32);
        sorted = sorting::selection_sort(sorted);
        assert_eq!(sorted, vec!(0));
    }

    #[test]
    fn test_selection_sort_empty() {
        let mut sorted = vec!();
        sorted = sorting::selection_sort::<i32>(sorted);
        assert_eq!(sorted, vec!());
    }
}
