// PlayJS Rust Example: Sorting Algorithms
// Demonstrates various sorting algorithms and performance comparisons

use std::time::Instant;

fn main() {
    println!("=== Sorting Algorithms Demo ===\n");

    let mut data = vec![64, 34, 25, 12, 22, 11, 90, 88, 45, 50, 23, 36, 18, 77];
    println!("Original array: {:?}\n", data);

    // Bubble Sort
    let mut bubble_data = data.clone();
    let start = Instant::now();
    bubble_sort(&mut bubble_data);
    let duration = start.elapsed();
    println!("Bubble Sort: {:?}", bubble_data);
    println!("Time: {:?}\n", duration);

    // Quick Sort
    let mut quick_data = data.clone();
    let start = Instant::now();
    quick_sort(&mut quick_data, 0, quick_data.len() as isize - 1);
    let duration = start.elapsed();
    println!("Quick Sort: {:?}", quick_data);
    println!("Time: {:?}\n", duration);

    // Merge Sort
    let mut merge_data = data.clone();
    let start = Instant::now();
    merge_sort(&mut merge_data);
    let duration = start.elapsed();
    println!("Merge Sort: {:?}", merge_data);
    println!("Time: {:?}\n", duration);

    // Built-in Sort
    let start = Instant::now();
    data.sort();
    let duration = start.elapsed();
    println!("Rust Built-in Sort: {:?}", data);
    println!("Time: {:?}", duration);
}

fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn quick_sort<T: Ord>(arr: &mut [T], low: isize, high: isize) {
    if low < high {
        let pivot = partition(arr, low, high);
        quick_sort(arr, low, pivot - 1);
        quick_sort(arr, pivot + 1, high);
    }
}

fn partition<T: Ord>(arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot_idx = high as usize;
    let mut i = low - 1;

    for j in low..high {
        if arr[j as usize] <= arr[pivot_idx] {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap((i + 1) as usize, pivot_idx);
    i + 1
}

fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut merged = Vec::with_capacity(len);
    let (left, right) = arr.split_at(mid);

    let mut left_iter = left.iter().cloned();
    let mut right_iter = right.iter().cloned();
    let mut left_next = left_iter.next();
    let mut right_next = right_iter.next();

    loop {
        match (left_next.clone(), right_next.clone()) {
            (Some(l), Some(r)) => {
                if l <= r {
                    merged.push(l);
                    left_next = left_iter.next();
                } else {
                    merged.push(r);
                    right_next = right_iter.next();
                }
            }
            (Some(l), None) => {
                merged.push(l);
                merged.extend(left_iter);
                break;
            }
            (None, Some(r)) => {
                merged.push(r);
                merged.extend(right_iter);
                break;
            }
            (None, None) => break,
        }
    }

    arr.clone_from_slice(&merged);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = vec![5, 2, 8, 1, 9];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 5, 8, 9]);
    }

    #[test]
    fn test_quick_sort() {
        let mut arr = vec![5, 2, 8, 1, 9];
        quick_sort(&mut arr, 0, 4);
        assert_eq!(arr, vec![1, 2, 5, 8, 9]);
    }

    #[test]
    fn test_merge_sort() {
        let mut arr = vec![5, 2, 8, 1, 9];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 5, 8, 9]);
    }
}
