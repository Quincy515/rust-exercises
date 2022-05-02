// HRTB(Higher-ranked trait bounds)
// Type bounds may be higher ranked over lifetimes.
// These bounds specify a bound is true for all lifetimes.
// For example, a bound such as for<'a> &'a T: PartialEq<i32>
// would require an implementation like:

#![allow(unused)]

// impl<'a> PartialEq<i32> for &'a T {}

// and could then be used to compare a &'a T with any lifetime to an i32.

// Only a higher-ranked bound can be used here,
// because the lifetime of the reference is shorter than
// any possible lifetime parameter on the function。

// 4、🌟🌟🌟
/* Adding HRTB to make it work!*/
fn call_on_ref_zero<F>(f: F)
where
    for<'a> F: Fn(&'a i32),
{
    let zero = 0;
    f(&zero);
}

fn call_on_ref_zero_1<F>(f: F)
where
    F: for<'a> Fn(&'a i32),
{
    let zero = 0;
    f(&zero);
}

fn main() {
    println!("Success!")
}
