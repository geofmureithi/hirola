use std::borrow::Borrow;
use std::convert::TryFrom;
use std::fmt::{Display, Error, Formatter};
use std::ops::Deref;
use std::str::FromStr;
// use serde::{Serialize, Deserialize};
// use ron::ser::to_string;
use futures_signals::signal::Mutable;
// trait IReactive<A>

/// Reactive assumes data
#[derive(Clone, Debug)]
pub struct Reactive<T> {
    pub anchor: Option<String>,
    pub value: T,
}

impl<T> Display for Reactive<T>
where
    T: Into<String>,
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Display::fmt("Reactive", f)
    }
}

// impl Deref for Reactive {
//     type Target = String;
//     fn deref(&self) -> &Self::Target {
//         &self.anchor
//     }
// }

// impl From<&str> for Reactive<&str> {
//     fn from(value: &str) -> Self {
//         Reactive<'static> {
//             anchor: None,
//             value:  value,
//         }
//     }
// }

impl From<bool> for Reactive<bool> {
    fn from(b: bool) -> Self {
        Reactive {
            anchor: None,
            value: b,
        }
    }
}

// impl<T> From<Mutable<T>> for Reactive<T>
//     where T: Into<String>
// {
//     fn from(t: Mutable<T>) -> Self {
//         Reactive {
//             anchor: Some(String::from("anchor")),
//             value: t.deref(),
//         }
//     }
// }

// impl <T> From<&Mutable<T>> for Reactive {
//     fn from(t: &Mutable<T>) -> Self {
//         Reactive {
//             anchor: Some(String::from("anchor")),
//             value: Some(t.into())
//         }
//     }
// }
