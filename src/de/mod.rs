//! Module containing all the deserialization implementations
//! of the types that need them.
#[cfg(test)]
use anyhow::{Ok as Aok, Result};
use serde::{
    Deserialize,
    de::{self, Visitor},
};

use crate::prelude::*;

mod line;
mod text_line;
mod toml_node;
mod toml_node_map;

#[cfg(test)]
fn de_test<'de, D>(test_cases: Vec<(D, &'de str, &'de str)>) -> Result<()>
where
    D: Deserialize<'de> + std::fmt::Debug + PartialEq,
{
    test_cases.into_iter().try_for_each(|(want, s, desc)| {
        let got = toml::from_str::<D>(s)?;
        assert_eq!(want, got, "Test deserializing {desc}");
        Aok(())
    })
}

#[cfg(test)]
fn de_error_test<'de, D>(test_cases: Vec<(&'de str, &'de str)>)
where
    D: Deserialize<'de>,
{
    test_cases.into_iter().for_each(|(s, desc)| {
        let Err(_) = toml::from_str::<D>(s) else {
            let expect = format!("Test \"{desc}\" expected an error!");
            let toml = format!("Passed in toml: {s}");
            panic!("\n{expect}\n{toml}\n")
        };
    });
}
