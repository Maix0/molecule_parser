
use crate::*;
#[test]
fn all() {
    println!("{}", ALL.as_str());
    assert!(ALL.is_match("1-methylpropane"));
    assert!(ALL.is_match("1,5-diethyloctan-2-ol"));
    assert!(ALL.is_match("1-ethyl-2,5-methylethanoïque"));
}

#[test]
fn alkane() {
    println!("{}", ALKANE.as_str());
    assert!(ALKANE.is_match("methane"));
    assert!(ALKANE.is_match("ethan-2-ol"));
    assert!(ALKANE.is_match("pentanoïque"));
}
#[test]
fn alkyls() {
    println!("{}", ALKYLS.as_str());
    assert!(ALKYLS.is_match("1-methyl"));
    assert!(ALKYLS.is_match("1,5-dipentyl"));
    assert!(ALKYLS.is_match("1,5-dipentyl-5,1,1-triethyl"));
}
#[test]
fn single_alkyls() {
    println!("{}", SINGLE_ALKYL.as_str());
    assert!(SINGLE_ALKYL.is_match("1-methyl"));
    assert!(SINGLE_ALKYL.is_match("1,5-dipentyl"));
}
