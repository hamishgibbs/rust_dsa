/*

How it works:

A way to sort a mutable object based on a max heap (a binary tree where children are <= to their parents)

Note that a binary tree can be represented as an array and accessed with:

parent(i) = (i - 1) / 2
left_child(i) = 2*i + 1
right_child(i) = 2*i + 2

For example, with indices (0, 1, 2):

parent(0) = -1 / 2 = 0
left_child(0) = 1
left_child(0) = 2

parent(1) = 0 / 2 = 0
left_child(1) = 3
left_child(1) = 4

parent(2) = 1 / 2 = 0
left_child(2) = 5
left_child(2) = 6


Heap sort steps:

1. Convert input array into a max heap.
2. Divide array into heap part and sorted part.
3. Swap the root of the heap (the highest value) with the last element of the heap
   and increase the sorted part by one. (to the root).
4. Update heap to make it a max heap again.
5. Repeat until heap is empty.

*/

pub fn heap_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    heapify(arr);

    for end in (1..arr.len()).rev() {
        arr.swap(0, end);
        move_down(&mut arr[..end], 0)
    }
}

fn heapify<T: Ord>(arr: &mut [T]) {
    let last_parent = (arr.len() - 2) / 2;
    for i in (0..=last_parent).rev() {
        move_down(arr, i);
    }
}

fn move_down<T: Ord>(arr: &mut [T], mut root: usize) {

    let last = arr.len() - 1;

    loop {
        // left_child(i) = 2*i + 1
        let left = 2 * root + 1;

        if left > last {
            break;
        }

        // right_child(i) = 2*i + 2 = left_child(i) + 1
        let right = left + 1;

        let max = if right <= last && arr[right] > arr[left] {
            right
        } else {
            left
        };

        if arr[max] > arr[root] {
            arr.swap(root, max);
        }

        root = max

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut arr: Vec<i32> = Vec::new();
        heap_sort(&mut arr);
        assert_eq!(&arr, &[]);
    }

    #[test]
    fn single_element() {
        let mut arr = vec![1];
        heap_sort(&mut arr);
        assert_eq!(&arr, &[1]);
    }

    #[test]
    fn sorted_array() {
        let mut arr = vec![1, 2, 3, 4];
        heap_sort(&mut arr);
        assert_eq!(&arr, &[1, 2, 3, 4]);
    }

    #[test]
    fn unsorted_array() {
        let mut arr = vec![3, 4, 2, 1];
        heap_sort(&mut arr);
        assert_eq!(&arr, &[1, 2, 3, 4]);
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr = vec![3, 4, 2, 1, 7];
        heap_sort(&mut arr);
        assert_eq!(&arr, &[1, 2, 3, 4, 7]);
    }

    #[test]
    fn repeated_elements() {
        let mut arr = vec![542, 542, 542, 542];
        heap_sort(&mut arr);
        assert_eq!(&arr, &vec![542, 542, 542, 542]);
    }
}
