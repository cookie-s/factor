extern crate core;
extern crate num;
extern crate factor;

use num::bigint::BigUint;
use num::One;
use factor::*;

enum Method {
    Fermat,
    PMinus1,
}

fn main() {
    loop {
        let mut method = Method::Fermat;
        let mut n = BigUint::one();

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        let it = line.trim().split_whitespace();
        for s in it {
            if let Some(m) = BigUint::parse_bytes(s.as_bytes(), 10) {
                n = m;
            } else {
                method = match s {
                    "f" => Method::Fermat,
                    "pm1" => Method::PMinus1,
                    _ => method,
                }
            }
        }

        let p = match method {
            Method::Fermat => fermat(&n),
            Method::PMinus1 => pm1(&n, 1000),
        };

        println!("{}", p);
        println!("{}", n / p);
    }
}
