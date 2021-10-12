/*

How it works:

Find the contiguous subarray with the highest sum.

What is a contiguous subarray? A "section" of an array:

[-4, -5, 1] and [1, 6] are contiguous subarrays of [-4, -5, 1, 6, -8].

In the above case, [1, 6] is also the maximum subarray because sum([1, 6]) = 7.

There is no other contiguous subarray with a higher sum.


What is dynamic programming?

A technique of breaking a problem into subproblems. The solution to the problem will be the
solution to its subproblem. Example iterations from the code below:

- Setup -
arr = [-2, 4, 6]
dp = [0, 0, 0]

result = 0


- Iteration 0 -
dp = [-2]

result = -2

- Iteration 1 -
dp = [-2, 4]

result = 4

- Iteration 2 -
dp = [-2, 4, 10]

result = 10

The sum of the maximum subarray [4, 6] is 10.

The solution to the overall problem is the maximum solution to subproblems (addition between
array elements).

*/

fn maximum_subarray(array: &[i32]) -> i32 {

    let mut dp = vec![0; array.len()];

    dp[0] = array[0];

    let mut result = dp[0];

    for i in 1..array.len() {
        if dp[i - 1] > 0 {
            dp[i] = dp[i - 1] + array[i];
        } else {
            dp[i] = array[i];
        }
        result = result.max(dp[i]);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_negative() {
        //the maximum value: 1 + 0 + 5 + 8 = 14
        let array = vec![1, 0, 5, 8];
        assert_eq!(maximum_subarray(&array), 14);
    }

    #[test]
    fn negative() {
        //the maximum value: -1
        let array = vec![-3, -1, -8, -2];
        assert_eq!(maximum_subarray(&array), -1);
    }

    #[test]
    fn normal() {
        //the maximum value: 3 + (-2) + 5 = 6
        let array = vec![-4, 3, -2, 5, -8];
        assert_eq!(maximum_subarray(&array), 6);
    }

    #[test]
    fn single_element() {
        let array = vec![6];
        assert_eq!(maximum_subarray(&array), 6);
        let array = vec![-6];
        assert_eq!(maximum_subarray(&array), -6);
    }
}
