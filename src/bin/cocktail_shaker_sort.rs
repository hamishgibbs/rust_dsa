/*
How it works:

Iterate up and down the array & swap elements if they are >
than the following element.

Example iterations where x = [4, 3, 2, 1]

i == 0: x = [1, 3, 2, 4]
i == 1: x = [1, 2, 3, 4]

*/

pub fn cocktail_shaker_sort<T: Ord>(arr: &mut [T]) {
    // Define initial len of array
    let len = arr.len();

    // End when len of array remaining == 0
    if len == 0 {
        return;
    }

    loop {
        // Define whether we swapped values on this half of this iteration
        let mut swapped = false;

        // Work up the array (0 ... len)
        // clamp "clips" value between a min and max
        // i.e. 5.clamp(0, 4) == 4 & -5.clamp(0, 4) == 0
        for i in 0..(len - 1).clamp(0, len) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        // If a swap didn't occur (list is sorted), break
        if !swapped {
            break;
        }

        swapped = false;

        // Work down the array (len ... 0)
        for i in (0..(len - 1).clamp(0, len)).rev() {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut arr = vec![5, 2, 1, 3, 4, 6];
        cocktail_shaker_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn empty() {
        let mut arr = Vec::<i32>::new();
        cocktail_shaker_sort(&mut arr);
        assert_eq!(arr, vec![])
    }

    #[test]
    fn one_element() {
        let mut arr = vec![1];
        cocktail_shaker_sort(&mut arr);
        assert_eq!(arr, vec![1]);
    }

    #[test]
    fn pre_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        cocktail_shaker_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }

}
