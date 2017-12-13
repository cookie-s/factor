extern crate core;
extern crate num;

use num::bigint::BigUint;
use num::{Integer, One};

// r is prime bounder
fn pm1(n : &BigUint, r : usize) -> BigUint {
    let mut a = BigUint::from(2u32);

    for b in 2..r {
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

// binary search
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

fn fermat(n : &BigUint) -> BigUint {
    let mut x = sqrt(n) + 1u8;
    let mut y = &x * &x - n;

    loop {
        let sy = sqrt(&y);
        if y == &sy * &sy {
            break
        }
        y += 2u8 * &x + 1u8;
        x += 1u8;
    }
    x + sqrt(&y)
}

fn main() {
    let n = BigUint::parse_bytes(b"149767527975084886970446073530848114556615616489502613024958495602726912268566044330103850191720149622479290535294679429142532379851252608925587476670908668848275349192719279981470382501117310509432417895412013324758865071052169170753552224766744798369054498758364258656141800253652826603727552918575175830897", 10).unwrap();
    println!("{} = ", n);
    let r = 30;
    let g = pm1(&n, r);
    println!("{} * {}", &g, &n / &g);

    println!("");

    let n = BigUint::parse_bytes(b"1617923821376084316956374671710122260256179281048075915460870730419594082250104835141819768132765032313650625900061184449501919418007145585037630441586862069423279317533916381812603755633938528825792712027151967932041882225140808521270563581175747923069060476264642874964651131226761508883869085871266559270163", 10).unwrap();
    println!("{} =", n);
    let g = fermat(&n);
    println!("{} * {}", &g, &n / &g);
}
