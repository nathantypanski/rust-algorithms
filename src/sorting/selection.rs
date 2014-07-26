use std::ptr;
use std::mem;
use std::vec::Vec;

/// Implementation of selection sort.
/// CLRS pg 29. `O(n^2)`.
pub fn selection_sort<T: Ord>(mut a: Vec<T>) -> Vec<T> {
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
