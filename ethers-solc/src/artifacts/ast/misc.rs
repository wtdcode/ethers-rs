use serde::{Deserialize, Serialize};
use std::{fmt, fmt::Write, str::FromStr};

/// Represents the source location of a node: `<start byte>:<length>:<source index>`.
///
/// The `start`, `length` and `index` can be -1 which is represented as `None`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLocation {
    pub start: Option<usize>,
    pub length: Option<usize>,
    pub index: Option<usize>,
}

impl FromStr for SourceLocation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let invalid_location = move || format!("{s} invalid source location");

        let mut split = s.split(':');
        let start = split
            .next()
            .ok_or_else(invalid_location)?
            .parse::<isize>()
            .map_err(|_| invalid_location())?;
        let length = split
            .next()
            .ok_or_else(invalid_location)?
            .parse::<isize>()
            .map_err(|_| invalid_location())?;
        let index = split
            .next()
            .ok_or_else(invalid_location)?
            .parse::<isize>()
            .map_err(|_| invalid_location())?;

        let start = if start < 0 { None } else { Some(start as usize) };
        let length = if length < 0 { None } else { Some(length as usize) };
        let index = if index < 0 { None } else { Some(index as usize) };

        Ok(Self { start, length, index })
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(start) = self.start {
            start.fmt(f)?;
        } else {
            f.write_str("-1")?;
        }
        f.write_char(':')?;
        if let Some(length) = self.length {
            length.fmt(f)?;
        } else {
            f.write_str("-1")?;
        }
        f.write_char(':')?;
        if let Some(index) = self.index {
            index.fmt(f)?;
        } else {
            f.write_str("-1")?;
        }
        Ok(())
    }
}

/// Function mutability specifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StateMutability {
    Payable,
    Pure,
    Nonpayable,
    View,
}

/// Variable mutability specifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mutability {
    Mutable,
    Immutable,
    Constant,
}

/// Storage location specifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StorageLocation {
    Calldata,
    Default,
    Memory,
    Storage,
}

/// Visibility specifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    External,
    Public,
    Internal,
    Private,
}

/// A type description.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeDescriptions {
    pub type_identifier: Option<String>,
    pub type_string: Option<String>,
}

// TODO: do it!
// #[derive(PartialEq, Debug)]
// pub struct ParsedTypes {
//     pub identifier: TypeId,
//     pub visibility: Option<Visibility>,
//     pub returns: Vec<String>,
// }

// impl FromStr for Visibility {

// }

// impl From<TypeDescriptions> for ParsedTypes {
//     fn from(value: TypeDescriptions) -> Result<Self> {
//         let identifiers: Vec<String> = match type_descriptions.type_identifier {
//             Some(val) => {
//                 let val = val.split("$");
//                 val.map(|i| i.to_string()).collect()
//             }
//             None => return None,
//         };

//         dbg!(&identifiers);

//         let ids: Vec<String> = identifiers[0].split('_').map(|i| i.to_string()).collect();
//         dbg!(&ids);
//         let identifier = ids[1].clone().into();
//         dbg!(&identifier, &ids[2]);
//         let visibility = serde_json::from_str(&ids[2]).unwrap_or(None);
//         dbg!(&visibility);

//         Some(ParsedTypes { identifier, visibility, returns: vec![identifiers[2].clone().into()]
// })     }
// }
