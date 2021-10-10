/*

How it works:

Extended Euclidean Algorithm takes 2 integers and computes:

1. Greatest Common Divisor (gcd)
2-3. coefficients of Bezout's Identity

Where gcd for two integers (a, b) is the largest common integer that is a divisor of both a and b.

i.e.

gcd(6, 12) = 6

And where coefficients of Bezout's Identity for two integers (a, b) are two integers (x, y) such that:

ax + by = gcd(a, b)

i.e.

(6)(x) + (12)(y) = 6

(6)(1) + (12)(0) = 6

6 + 0 = 6

Therefore (x, y) = (1, 0)

*/


// `&` is a reference (`&mut` is a mutable reference).
// `*` dereferences a reference.
// here, using &mut for `a` means that changes are made to `a` (`a` exists out of
// the `update_step` function's scope).
// To "set the value of `a` to whatever" the reference to `a` must be dereferenced with *
fn update_step(a: &mut i32, old_a: &mut i32, quotient: i32) {
    let temp = *a;
    *a = *old_a - quotient * temp;
    *old_a = temp;
}

pub fn extended_euclidean_algorithm(a: i32, b: i32) -> (i32, i32, i32) {
    let (mut old_r, mut rem) = (a, b);
    let (mut old_s, mut coeff_s) = (1, 0);
    let (mut old_t, mut coeff_t) = (0, 1);

    while rem != 0 {
        let quotient = old_r / rem;

        update_step(&mut rem, &mut old_r, quotient);
        update_step(&mut coeff_s, &mut old_s, quotient);
        update_step(&mut coeff_t, &mut old_t, quotient);

    }

    (old_r, old_s, old_t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
       assert_eq!(extended_euclidean_algorithm(101, 13), (1, 4, -31));
       assert_eq!(extended_euclidean_algorithm(123, 19), (1, -2, 13));
       assert_eq!(extended_euclidean_algorithm(25, 36), (1, 13, -9));
       assert_eq!(extended_euclidean_algorithm(69, 54), (3, -7, 9));
       assert_eq!(extended_euclidean_algorithm(55, 79), (1, 23, -16));
       assert_eq!(extended_euclidean_algorithm(33, 44), (11, -1, 1));
       assert_eq!(extended_euclidean_algorithm(50, 70), (10, 3, -2));
    }
}
