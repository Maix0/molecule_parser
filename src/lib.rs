use once_cell;
use once_cell::sync::Lazy;
use regex;
use regex::Regex;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

static_regex!(BASE, "(?:meth|eth|prop|but|pent|hex|hept|oct)");
static_regex!(SUFFIX, "(?:di|tri|tetra)");
static_regex!(NUM, r"\d+");
static_regex!(
    SINGLE_ALKYL,
    &format!(r"(\d+(?:,\d+)*-{}?{}yl)", SUFFIX.as_str(), BASE.as_str(),)
);
static_regex!(
    ALKYLS,
    &format!(
        r"(?P<ALL_ALKYLS>{}(:?-{})*)",
        SINGLE_ALKYL.as_str(),
        SINGLE_ALKYL.as_str()
    )
);
static_regex!(FUNCTION, r"-(\d+)-(one|ol)");
static_regex!(
    ALKANE,
    &format!(
        r"((?P<ALKANE>{})an(?P<FUNCTION>e|oïque|al|(?:-\d+-(?:one|ol))))",
        BASE.as_str()
    )
);
static_regex!(ALL, &format!("^{}?{}$", ALKYLS.as_str(), ALKANE.as_str()));

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Compound {
    pub alkyls: Alkyls,
    pub alkane: Base,
    pub function: Function,
}

impl Compound {
    pub fn parse(src: &str) -> Option<Self> {
        if !ALL.is_match(src) {
            return None;
        }
        let caps = ALL.captures(src).unwrap();
        let alkane = Base::from_str(caps.name("ALKANE").unwrap().as_str());
        let function = Function::from_str(caps.name("FUNCTION").unwrap().as_str());
        let alkyls = Alkyls::from_str(match caps.name("ALL_ALKYLS") {
            Some(m) => m.as_str(),
            None => "",
        });
        Some(Self {
            alkane,
            function,
            alkyls,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Alkyls(Vec<([Option<u8>; 4], Base)>);

impl Alkyls {
    fn from_str(src: &str) -> Self {
        let mut out = Vec::new();
        for alkyle in SINGLE_ALKYL.find_iter(src) {
            let nums: Vec<u8> = NUM
                .find_iter(alkyle.as_str())
                .map(|m| m.as_str().parse::<u8>().unwrap())
                .collect();
            let mut nums_final = [None; 4];
            for (i, n) in nums.iter().enumerate() {
                if i < 4 {
                    nums_final[i] = Some(*n);
                }
            }
            let base_str = BASE.find(alkyle.as_str()).unwrap().as_str();
            out.push((nums_final, Base::from_str(base_str)));
        }
        Self(out)
    }
}
impl std::ops::Deref for Alkyls {
    type Target = Vec<([Option<u8>; 4], Base)>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for Alkyls {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum Function {
    None,              // "e"
    Carboxylic,        // "oïque"
    Aldehyde,          // "al"
    Ketones { n: u8 }, // "$x-one"
    Alcohol { n: u8 }, // "$x-ol"
}

impl Function {
    fn from_str(src: &str) -> Self {
        match src {
            "e" => Self::None,
            "oïque" => Self::Carboxylic,
            "al" => Self::Aldehyde,
            _ => {
                if FUNCTION.is_match(src) {
                    match {
                        let caps = FUNCTION.captures(src).unwrap();
                        (
                            caps.get(1).unwrap().as_str().parse::<u8>().unwrap(),
                            caps.get(2).unwrap().as_str(),
                        )
                    } {
                        (n, "one") => Self::Ketones { n },
                        (n, "ol") => Self::Alcohol { n },
                        _ => Self::None,
                    }
                } else {
                    Self::None
                }
            }
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Base {
    Methane,
    Ethane,
    Propane,
    Butane,
    Pentane,
    Hexane,
    Heptane,
    Octane,
    None,
}
impl Base {
    fn from_str(src: &str) -> Self {
        match src {
            "meth" => Self::Methane,
            "eth" => Self::Ethane,
            "prop" => Self::Propane,
            "but" => Self::Butane,
            "pent" => Self::Pentane,
            "hex" => Self::Hexane,
            "hept" => Self::Heptane,
            "oct" => Self::Octane,
            _ => Self::None,
        }
    }
}

#[cfg(test)]
mod test;

#[macro_export]
macro_rules! static_regex {
    ($name:ident , $regex:expr) => {
        static $name: Lazy<Regex> = Lazy::new(|| {
            regex::RegexBuilder::new($regex)
                .case_insensitive(true)
                .build()
                .unwrap()
        });
    };
}
