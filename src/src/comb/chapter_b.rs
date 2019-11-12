use crate::comb::*;
use crate::fastset::*;
use std::cmp;

use crate::setlike::HFolds;

macro_rules! info {
    ($verb_cond:ident, $( $arg:tt )+) => {
        if $verb_cond {
            println!($($arg)+);
        }
    };
}

pub fn phi(n: u32, h: u32, verbose: bool) -> u32 {
    if n == 1 {
        return 1;
    }
    if h == 1 {
        return n;
    }
    let res = _phi_interval(n, (0, h), verbose);
    res + 1
}

pub fn phi_interval(n: u32, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    _phi_interval(n, (ia, ib), verbose)
}

fn _phi_interval(n: u32, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    let mut lower_bound = 1;
    // Proposition B.10
    if (ia, ib).0 == 0 {
        let s = (ia, ib).1;
        lower_bound = cmp::max(
            1,
            (((factorial(s) * n) as f32).powf(1f32 / (s as f32)).ceil() as i32) - (s as i32),
        ) as u32;
        info!(
            verbose,
            "(Proposition B.10) Using lower bound: {}", lower_bound
        );
    }

    for m in lower_bound.. {
        for a in each_set_exact(n, m) {
            if a.hfoldintervalsumset((ia, ib), n).isfull(n) {
                info!(verbose, "Found spanning set: {}", a);
                return m;
            }
        }
    }
    unreachable!();
}

// TODO: Maybe impliment f and g functions on page 132
//(need an upper bound on n though, maybe read paper?)

pub fn phi_signed(n: u32, h: u32, verbose: bool) -> u32 {
    if n == 1 {
        return 1;
    }
    for m in 2u32.. {
        for a in each_set_exact(n, m) {
            if a.hfoldsignedsumset(h, n).isfull(n) {
                info!(verbose, "Found spanning set: {}", a);
                return m;
            }
        }
    }
    unreachable!();
}

pub fn phi_signed_interval(n: u32, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    for m in 1u32.. {
        for a in each_set_exact(n, m) {
            if a.hfoldintervalsignedsumset((ia, ib), n).isfull(n) {
                info!(verbose, "Found spanning set: {}", a);
                return m;
            }
        }
    }
    unreachable!();
}

// Not a very researched function... (page 145)
pub fn phi_restricted(n: u32, h: u32, verbose: bool) -> u32 {
    if n == 1 {
        return 1;
    }
    if h == 1 {
        return n;
    }
    for m in 2u32.. {
        for a in each_set_exact(n, m) {
            if a.hfoldrestrictedsumset(h, n).isfull(n) {
                info!(verbose, "Found spanning set: {}", a);
                return m;
            }
        }
    }
    unreachable!();
}

pub fn phi_restricted_interval(n: u32, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    let mut lower_bound = 1u32;
    // Proposition B.73
    if (ia, ib) == (0, 2) {
        lower_bound = ((((8 * n - 7) as f32).sqrt() - 1.0) / 2.0).ceil() as u32;
        info!(
            verbose,
            "(Proposition B.73) Using lower bound: {}", lower_bound
        );
    }
    for m in lower_bound.. {
        for a in each_set_exact(n, m) {
            if a.hfoldintervalrestrictedsumset((ia, ib), n).isfull(n) {
                info!(verbose, "Found spanning set: {}", a);
                return m;
            }
        }
    }
    unreachable!();
}

pub fn phi_signed_restricted(n: u32, h: u32, verbose: bool) -> u32 {
    for m in 2u32.. {
        for a in each_set_exact(n, m) {
            if a.hfoldrestrictedsignedsumset(h, n).isfull(n) {
                info!(verbose, "Found spanning set: {}", a);
                return m;
            }
        }
    }
    unreachable!();
}

pub fn phi_signed_restricted_interval(n: u32, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    for m in 1u32.. {
        for a in each_set_exact(n, m) {
            if a.hfoldintervalrestrictedsignedsumset((ia, ib), n).isfull(n) {
                info!(verbose, "Found spanning set: {}", a);
                return m;
            }
        }
    }
    unreachable!();
}
