use crate::fastset::*;

use crate::setlike::HFolds;

use crate::setlike::{SetLike, Group};

macro_rules! info {
    ($verb_cond:ident, $( $arg:tt )+) => {
        if $verb_cond {
            println!($($arg)+);
        }
    };
}

pub fn tau<S: SetLike>(n: S::Group, h: u32, verbose: bool) -> u32 {
    for m in (1..=n.gsize()).rev() {
        let mut found = false;
        for a in S::each_set_exact_no_zero(n.clone(), m) {
            if a.hfoldsumset(h, n.clone()).zero_free(n.clone()) {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(verbose, "(gives:) {:?}", a.hfoldsumset(h, n.clone()));
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    unreachable!();
}

pub fn tau_interval<S: SetLike>(n: S::Group, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    for m in (1..=n.gsize()).rev() {
        let mut found = false;
        for a in S::each_set_exact_no_zero(n.clone(), m) {
            if a.hfoldintervalsumset((ia, ib), n.clone()).zero_free(n.clone()) {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(verbose, "(gives:) {:?}", a.hfoldintervalsumset((ia, ib), n.clone()));
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    unreachable!();
}

pub fn tau_restricted<S: SetLike>(n: S::Group, h: u32, verbose: bool) -> u32 {
    // Theorem F.88
    // if n >= 12 && n % 2 == 0 && (3 <= h) && (h <= n - 1) && (h % 2 == 1) {
    //     if h == 1 {
    //         return n - 1;
    //     }
    //     if (3 <= h) && (h <= n / 2 - 2) {
    //         return n / 2;
    //     }
    //     if h == n / 2 - 1 {
    //         return n / 2 + 1;
    //     }
    //     if (n / 2 <= h) && (h <= n - 2) {
    //         return h + 1;
    //     }
    //     // h = n - 1 (guaranteed)
    //     return n - 1;
    // }
    // if n == 1 {
    //     return 1;
    // }
    for m in (1..=n.gsize()).rev() {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if a.hfoldrestrictedsumset(h, n.clone()).zero_free(n.clone()) {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(verbose, "(gives:) {:?}", a.hfoldrestrictedsumset(h, n.clone()));
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    unreachable!();
}

pub fn tau_restricted_interval<S: SetLike>(n: S::Group, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    for m in (1..=n.gsize()).rev() {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if a.hfoldintervalrestrictedsumset((ia, ib), n.clone()).zero_free(n.clone()) {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(
                    verbose,
                    "(gives:) {:?}",
                    a.hfoldintervalrestrictedsumset((ia, ib), n.clone())
                );
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    unreachable!();
}

pub fn tau_signed<S: SetLike>(n: S::Group, h: u32, verbose: bool) -> u32 {
    for m in (1..=n.gsize()).rev() {
        let mut found = false;
        for a in S::each_set_exact_no_zero(n.clone(), m) {
            if a.hfoldsignedsumset(h, n.clone()).zero_free(n.clone()) {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(verbose, "(gives:) {:?}", a.hfoldsignedsumset(h, n.clone()));
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    unreachable!();
}

pub fn tau_signed_interval<S: SetLike>(n: S::Group, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    for m in (1..n.gsize()).rev() {
        let mut found = false;
        for a in S::each_set_exact_no_zero(n.clone(), m) {
            if a.hfoldintervalsignedsumset((ia, ib), n.clone()).zero_free(n.clone()) {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(
                    verbose,
                    "(gives:) {:?}",
                    a.hfoldintervalsignedsumset((ia, ib), n.clone())
                );
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    unreachable!();
}

pub fn tau_signed_restricted<S: SetLike>(n: S::Group, h: u32, verbose: bool) -> u32 {
    for m in (1..=n.gsize()).rev() {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if a.hfoldrestrictedsignedsumset(h, n.clone()).zero_free(n.clone()) {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(verbose, "(gives:) {:?}", a.hfoldrestrictedsignedsumset(h, n.clone()));
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    unreachable!();
}

pub fn tau_signed_restricted_interval<S: SetLike>(n: S::Group, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    for m in (1..=n.gsize()).rev() {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if a.hfoldintervalrestrictedsignedsumset((ia, ib), n.clone()).zero_free(n.clone()) {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(
                    verbose,
                    "(gives:) {:?}",
                    a.hfoldintervalrestrictedsignedsumset((ia, ib), n.clone())
                );
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    // Page 297
    #[test]
    fn test_tau_restricted() {
        let correct_table: Vec<u32> = vec![1, 2, 2, 3, 4, 4, 4, 5, 6, 6, 6, 6, 6, 7, 8, 8, 8, 9];
        let actual_table: Vec<u32> = (1..=18).map(|n| tau_restricted::<FastSet>(n, 3, false)).collect();
        assert_eq!(correct_table, actual_table);
    }
}
