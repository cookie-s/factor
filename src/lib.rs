extern crate core;
extern crate num;

use num::bigint::BigUint;
use num::{Integer, One};

/// factor by Pollard's P-1 method.
/// r is prime bounder
pub fn pm1(n : &BigUint, r : usize) -> BigUint {
    let mut a = BigUint::from(2u32);

    for b in 2..r+1 {
        let mut q = BigUint::from(b);

        while &q * &q < *n {
            q = &q * &q;
        }
        while q < *n {
            q *= b;
        }

        a = a.modpow(&q, &n);
    }
    a -= 1u8;
    let g = n.gcd(&a);
    g
}

#[test]
pub fn pm1_test() {
    use num::Zero;

    let n = BigUint::from(299u32);
    // 299 = 13 * 23 = (2*2*3 + 1) * (2*11 + 1)

    let p = pm1(&n, 5);
    assert!(&p != &n && &p != &BigUint::one());
    assert_eq!(&n % &p, BigUint::zero());

    let p = pm1(&n, 2);
    assert!(&p == &n || &p == &BigUint::one());

    let p = pm1(&n, 11);
    assert!(&p == &n || &p == &BigUint::one());
}

/// the floor of sqrt by binary search
fn sqrt(n : &BigUint) -> BigUint {
    let mut a = BigUint::one();
    let mut b = n.clone();

    while &b - &a > BigUint::one() {
        let m = (&a + &b) / 2u8;

        if &m * &m > *n {
            b = m.clone();
        } else {
            a = m.clone();
        }
    }

    a
}

#[test]
pub fn sqrt_test() {
    assert_eq!(sqrt(&BigUint::from(143u8)), BigUint::from(11u8));
    assert_eq!(sqrt(&BigUint::from(144u8)), BigUint::from(12u8));
    assert_eq!(sqrt(&BigUint::from(145u8)), BigUint::from(12u8));
}

/// factor by Fermat method
pub fn fermat(n : &BigUint) -> BigUint {
    let mut x = sqrt(n) + 1u8;
    let mut y = &x * &x - n;

    loop {
        let sy = sqrt(&y);
        if y == &sy * &sy {
            return x + sy;
        }
        y += 2u8 * &x + 1u8;
        x += 1u8;
    }
}

#[test]
pub fn fermat_test() {
    use num::Zero;

    let n = BigUint::from(299u32);
    // 299 = 13 * 23

    let p = fermat(&n);
    assert!(&p != &n && &p != &BigUint::one());
    assert_eq!(&n % &p, BigUint::zero());
}
