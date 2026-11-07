//! Language version parsing and formatting.
//!
//! Versions use dotted unsigned numeric components. One or more components
//! are accepted so simple definitions using 1.0 and definitions using
//! 1.0.0 can both be represented without changing the authored component
//! count during display.

use std::fmt;
use std::str::FromStr;

/// A dotted numeric language version.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LanguageVersion {
    components: Vec<u64>,
}

/// An invalid version declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionError {
    pub input: String,
    pub reason: String,
}

impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid version '{}': {}", self.input, self.reason)
    }
}

impl std::error::Error for VersionError {}

impl LanguageVersion {
    /// Creates a version from numeric components.
    pub fn new(components: Vec<u64>) -> Result<Self, VersionError> {
        if components.is_empty() {
            return Err(VersionError {
                input: String::new(),
                reason: "at least one component is required".into(),
            });
        }
        Ok(Self { components })
    }

    /// Returns the numeric components.
    pub fn components(&self) -> &[u64] {
        &self.components
    }

    /// Returns the major component.
    pub fn major(&self) -> u64 {
        self.components.first().copied().unwrap_or_default()
    }

    /// Returns the minor component, or zero when omitted.
    pub fn minor(&self) -> u64 {
        self.components.get(1).copied().unwrap_or_default()
    }

    /// Returns the patch component, or zero when omitted.
    pub fn patch(&self) -> u64 {
        self.components.get(2).copied().unwrap_or_default()
    }

    /// Formats the components without adding omitted zero components.
    pub fn to_dotted_string(&self) -> String {
        self.components
            .iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(".")
    }
}

impl FromStr for LanguageVersion {
    type Err = VersionError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.trim().is_empty() {
            return Err(VersionError {
                input: input.to_owned(),
                reason: "version is empty".into(),
            });
        }
        let mut components = Vec::new();
        for part in input.split('.') {
            if part.is_empty() {
                return Err(VersionError {
                    input: input.to_owned(),
                    reason: "empty version component".into(),
                });
            }
            let value = part.parse::<u64>().map_err(|_| VersionError {
                input: input.to_owned(),
                reason: format!("'{part}' is not an unsigned integer"),
            })?;
            components.push(value);
        }
        Self::new(components).map_err(|mut error| {
            error.input = input.to_owned();
            error
        })
    }
}

impl fmt::Display for LanguageVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_dotted_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_three_components() {
        let version: LanguageVersion = "1.2.3".parse().unwrap();
        assert_eq!(version.major(), 1);
        assert_eq!(version.minor(), 2);
        assert_eq!(version.patch(), 3);
    }

    #[test]
    fn rejects_non_numeric_components() {
        assert!("1.alpha.0".parse::<LanguageVersion>().is_err());
    }
}
