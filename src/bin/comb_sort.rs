/*

How it works:

This is very similar to bubble sort except that bubble sort
compares elements that are 1 element away from eachother while
Comb sort compares elements that are > 1 element away

The sort starts by comparing the top and bottom elements of the
array, then "shrinks" the comparison distance until it is 1.

This improves the speed of bubble sort for arrays with small values
at the end of the list ("rabbits"). These values greatly increase the
time of bubble sort because they have to be "swapped" all the way up
the array.

*/

fn comb_sort<T: Ord>(arr: &mut [T]) {
    // Set the initial comparison distance to the array length.
    let mut gap = arr.len();

    // Set a parameter that determines the amount of "shrinkage" of
    // the comparison distance at each iteration.
    let shrink = 1.3;
    let mut sorted = false;

    while !sorted {

        // Shrink the gap by the chosen value at each iteration.
        gap = (gap as f32 / shrink).floor() as usize;

        // If the gap is <= 1 the array is sorted.
        if gap <= 1 {
            gap = 1;
            sorted = true;
        }
        // If the array is not sorted, iterate up it and swap any
        // elements that are not in the correct position
        for i in 0..arr.len() - gap {
            let j = i + gap;
            if arr[i] > arr[j] {
                arr.swap(i, j);
                sorted = false;
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descending() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        comb_sort(&mut ve1);
        for i in 0..ve1.len() - 1 {
            assert!(ve1[i] <= ve1[i + 1]);
        }
    }

    #[test]
    fn ascending() {
        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        comb_sort(&mut ve2);
        for i in 0..ve2.len() - 1 {
            assert!(ve2[i] <= ve2[i + 1]);
        }
    }
}
