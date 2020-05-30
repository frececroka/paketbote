use std::cmp::Ordering;
use std::fmt::Display;
use std::str::FromStr;

use alpm::vercmp;
use regex::Regex;
use serde::export::Formatter;

use crate::error::Error;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Spec {
    pub name: String,
    pub version: Option<Version>,
}

impl Spec {
    pub fn new(name: String, version: Version) -> Spec {
        Spec { name, version: Some(version) }
    }

    pub fn new_without_version(name: String) -> Spec {
        Spec { name, version: None }
    }

    pub fn fallback_version(self, default: Version) -> Spec {
        let version = self.version.unwrap_or(default);
        Spec { version: Some(version), ..self }
    }

    pub fn satisfies(&self, depends: &Spec) -> bool {
        let provides = self;

        if depends.name != provides.name {
            return false;
        }

        match (&depends.version, &provides.version) {
            (Some(depends), Some(provides)) => provides.satisfies(depends),
            _ => true
        }
    }
}

impl FromStr for Spec {
    type Err = Error;
    fn from_str(spec: &str) -> Result<Self, Self::Err> {
        let re = Regex::new("([^<>=]*)([<>]?=|[<>]=?)(.*)").unwrap();
        match re.captures(spec) {
            Some(captures) => {
                let name = captures.get(1).unwrap().as_str().to_owned();
                let relation: Relation = captures.get(2).unwrap().as_str().parse()?;
                let version = captures.get(3).unwrap().as_str().to_owned();
                let version = Version::new(version, relation);
                Ok(Spec::new(name, version))
            }
            None =>
                Ok(Spec::new_without_version(spec.to_owned()))
        }
    }
}

impl Display for Spec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(version) = &self.version {
            write!(f, "{}{}", self.name, version)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Version {
    pub version: String,
    pub relation: Relation,
}

impl Version {
    pub fn new(version: String, relation: Relation) -> Version {
        Version { version, relation }
    }

    pub fn new_eq(version: String) -> Version {
        Version::new(version, Relation::Eq)
    }

    pub fn satisfies(&self, depends: &Version) -> bool {
        use Relation::*;
        assert_ne!(self.relation, Less, "Provides version with less than relation.");
        assert_ne!(self.relation, Greater, "Provides version with greater than relation.");
        let ord = vercmp(&self.version, &depends.version);
        match (&self.relation, &depends.relation) {
            (_, Eq) =>
                ord == Ordering::Equal,
            (LessEq, Greater) =>
                ord == Ordering::Greater,
            (LessEq, GreaterEq) =>
                ord == Ordering::Greater || ord == Ordering::Equal,
            (Eq, Less) =>
                ord == Ordering::Less,
            (Eq, LessEq) =>
                ord == Ordering::Less || ord == Ordering::Equal,
            (Eq, Greater) =>
                ord == Ordering::Greater,
            (Eq, GreaterEq) =>
                ord == Ordering::Greater || ord == Ordering::Equal,
            (GreaterEq, Less) =>
                ord == Ordering::Less,
            (GreaterEq, LessEq) =>
                ord == Ordering::Less || ord == Ordering::Equal,
            _ => true
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.relation, self.version)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Relation {
    Less,
    LessEq,
    Eq,
    GreaterEq,
    Greater,
}

impl FromStr for Relation {
    type Err = Error;
    fn from_str(relation: &str) -> Result<Self, Self::Err> {
        use Relation::*;
        match relation {
            "<" => Ok(Less),
            "<=" => Ok(LessEq),
            "=" => Ok(Eq),
            ">=" => Ok(GreaterEq),
            ">" => Ok(Greater),
            _ => Err(format!("unknown relation {}", relation).into())
        }
    }
}

impl Display for Relation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Relation::*;
        match self {
            Less => write!(f, "<"),
            LessEq => write!(f, "<="),
            Eq => write!(f, "="),
            GreaterEq => write!(f, ">="),
            Greater => write!(f, ">")
        }
    }
}

#[test]
fn test_satisfies() {
    fn test(depends: &str, provides: &str, expected: bool) {
        let depends: Spec = depends.parse().unwrap();
        let provides: Spec = provides.parse().unwrap();
        let msg = if expected {
            format!("expected that {} satisfies {}", provides, depends)
        } else {
            format!("expected that {} doesn't satisfy {}", provides, depends)
        };
        assert_eq!(provides.satisfies(&depends), expected, "{}", msg);
    }

    test("package", "package", true);
    test("package", "package>=1.0", true);
    test("package>=1.0", "package", true);
    test("package>=1.0", "package>=0.3", true);
    test("package>=1.0", "package>=1.3", true);
    test("package<=1.0", "package>=1.3", false);
    test("package<=1.3", "package>=1.3", true);
    test("package<1.3", "package>=1.3", false);
    test("package>=1.3", "package=1.3.2", true);
    test("package>1.3", "package=1.3.2", true);
    test("package>1.3.2", "package=1.3.2", false);
}
