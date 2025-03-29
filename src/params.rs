use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Category {
    Any,
    Misc,
    Programming,
    Dark,
    Pun,
    Spooky,
    Christmas,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Flag {
    Nsfw,
    Religious,
    Political,
    Racist,
    Sexist,
    Explicit,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Format {
    Json,
    Xml,
    Yaml,
    Txt,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum JokeType {
    Single,
    Twopart,
}

pub(crate) struct Selector<T> {
    selected: HashSet<T>,
    singleset: HashSet<T>, // Single choice option
}

impl<T: Eq + Hash + Copy> Selector<T> {
    pub fn new(singleset: &[T]) -> Self {
        Self {
            selected: HashSet::new(),
            singleset: singleset.iter().cloned().collect(),
        }
    }

    pub fn select(&mut self, value: T) {
        if self.singleset.contains(&value) {
            self.selected.clear();
        } else {
            self.selected.retain(|v| !self.singleset.contains(v));
        }
        self.selected.insert(value);
    }
}

impl<T: Display> Display for Selector<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            self.selected
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let text = match self {
            Self::Any => "Any",
            Self::Misc => "Misc",
            Self::Programming => "Programming",
            Self::Dark => "Dark",
            Self::Pun => "Pun",
            Self::Spooky => "Spooky",
            Self::Christmas => "Christmas",
        };
        write!(f, "{}", text)
    }
}

impl Display for Flag {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let text = match self {
            Self::Nsfw => "nsfw",
            Self::Religious => "religious",
            Self::Political => "political",
            Self::Racist => "racist",
            Self::Sexist => "sexist",
            Self::Explicit => "explicit",
        };
        write!(f, "{}", text)
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let text = match self {
            Self::Json => "json",
            Self::Xml => "xml",
            Self::Yaml => "yaml",
            Self::Txt => "txt",
        };
        write!(f, "{}", text)
    }
}

impl Display for JokeType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let text = match self {
            Self::Single => "single",
            Self::Twopart => "twopart",
        };
        write!(f, "{}", text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_display() {
        assert_eq!(Category::Any.to_string(), "Any");
        assert_eq!(Category::Misc.to_string(), "Misc");
    }

    #[test]
    fn test_flag_display() {
        assert_eq!(Flag::Nsfw.to_string(), "nsfw");
        assert_eq!(Flag::Religious.to_string(), "religious");
    }

    #[test]
    fn test_selector_category_type() {
        let mut categories = Selector::<Category>::new(&[Category::Any]);

        categories.select(Category::Misc);
        assert!(categories.selected.contains(&Category::Misc));
        assert!(!categories.selected.contains(&Category::Any));
        assert!(!categories.selected.contains(&Category::Programming));
        assert_eq!(categories.to_string(), "Misc");

        categories.select(Category::Misc);
        assert!(categories.selected.contains(&Category::Misc));
        assert!(!categories.selected.contains(&Category::Any));
        assert!(!categories.selected.contains(&Category::Programming));
        assert_eq!(categories.to_string(), "Misc");

        categories.select(Category::Any);
        assert!(!categories.selected.contains(&Category::Misc));
        assert!(!categories.selected.contains(&Category::Programming));
        assert!(categories.selected.contains(&Category::Any));
        assert_eq!(categories.to_string(), "Any");

        categories.select(Category::Any);
        assert!(!categories.selected.contains(&Category::Misc));
        assert!(!categories.selected.contains(&Category::Programming));
        assert!(categories.selected.contains(&Category::Any));
        assert_eq!(categories.to_string(), "Any");

        categories.select(Category::Programming);
        assert!(!categories.selected.contains(&Category::Any));
        assert!(!categories.selected.contains(&Category::Misc));
        assert!(categories.selected.contains(&Category::Programming));
        assert_eq!(categories.to_string(), "Programming");
    }

    #[test]
    fn test_selector_flag_type() {
        let mut flags = Selector::<Flag>::new(&[]);

        flags.select(Flag::Nsfw);
        assert!(flags.selected.contains(&Flag::Nsfw));
        assert!(!flags.selected.contains(&Flag::Religious));
        assert_eq!(flags.to_string(), "nsfw");

        flags.select(Flag::Religious);
        assert!(flags.selected.contains(&Flag::Nsfw));
        assert!(flags.selected.contains(&Flag::Religious));
        let result_set: HashSet<String> = flags
            .to_string()
            .split(",")
            .map(|s| s.to_string())
            .collect();
        let expected_set: HashSet<String> =
            HashSet::from(["nsfw".to_string(), "religious".to_string()]);
        assert_eq!(result_set, expected_set);
    }
}
