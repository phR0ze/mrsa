use failure::Fail;
use fungus::prelude::*;
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::fmt;

// An error indicating that something went wrong with a component operation
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Fail)]
pub enum ComponentError {
    /// An error indicating that an invalid component name was given
    InvalidName,
}

impl fmt::Display for ComponentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ComponentError::InvalidName => write!(f, "invalid component name was given"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Component {
    All,
    Config,
    None,
}

impl Component {
    /// Return a vector of all Component values except Component::All and Component::None.
    pub fn all() -> Vec<Component> {
        vec![Component::Config]
    }

    /// Return a String of all the given components joined over a ', '
    pub fn join<T: AsRef<[Component]>>(components: T) -> String {
        let result: Vec<String> = components.as_ref().iter().map(|x| x.to_string()).collect();
        result.join(", ")
    }
}

impl From<&str> for Component {
    fn from(val: &str) -> Self {
        match val.to_lowercase().as_ref() {
            "all" => Component::All,
            "config" => Component::Config,
            _ => Component::None,
        }
    }
}

impl From<String> for Component {
    fn from(val: String) -> Self {
        Component::from(val.as_str())
    }
}

// Implement format! support
impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => write!(f, "{}", format!("{:?}", self).to_lowercase()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_errors() {
        assert_eq!(format!("{}", ComponentError::InvalidName), "invalid component name was given");
    }

    #[test]
    fn test_all() {
        assert_eq!(Component::all(), vec![Component::Config,]);
    }

    #[test]
    fn test_join() {
        assert_eq!(Component::join(vec![Component::Config,]), "config");
    }

    #[test]
    fn test_component_from() {
        // str
        assert_eq!(Component::from("All"), Component::All);
        assert_eq!(Component::from("conFig"), Component::Config);
        assert_eq!(Component::from("foo"), Component::None);

        // String
        assert_eq!(Component::from("All".to_string()), Component::All);
        assert_eq!(Component::from("confIG".to_string()), Component::Config);
        assert_eq!(Component::from("foo".to_string()), Component::None);

        assert_eq!(Component::All.to_string(), "all");
        assert_eq!(Component::Config.to_string(), "config");
        assert_eq!(Component::None.to_string(), "none");
    }

    #[test]
    fn test_debug_string() {
        assert_eq!(format!("{}", Component::All), "all");
        assert_eq!(format!("{}", Component::Config), "config");
        assert_eq!(format!("{}", Component::None), "none");
    }
}
