//! Starting from `0.4.0` this crate is repurposed to supply types shared between
//! [simploxide-ws-core](https://docs.rs/simploxide-core/latest/simploxide_ws_core/) and
//! [simploxide-ffi-core](https://docs.rs/simploxide-core/latest/simploxide_ws_core/). Check the
//! documentation of the corresponding crates for actual functionality

use std::str::FromStr;

/// Parses SimpleX version numbers in the form `MAJOR.MINOR.PATCH.HOTFIX`.
#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub struct SimplexVersion {
    major: u8,
    minor: u8,
    patch: u8,
    hotfix: u8,
}

impl SimplexVersion {
    pub const fn new(major: u8, minor: u8, patch: u8, hotfix: u8) -> Self {
        Self {
            major,
            minor,
            patch,
            hotfix,
        }
    }

    pub fn major(&self) -> u8 {
        self.major
    }

    pub fn minor(&self) -> u8 {
        self.minor
    }

    pub fn patch(&self) -> u8 {
        self.patch
    }

    pub fn hotfix(&self) -> u8 {
        self.hotfix
    }
}

impl FromStr for SimplexVersion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num_iter = s.split('.');

        fn get_num<'a, 'b>(iter: &'a mut impl Iterator<Item = &'b str>) -> Result<u8, ()> {
            iter.next()
                .ok_or(())
                .and_then(|s| s.parse().map_err(|_| ()))
        }

        Ok(Self {
            major: get_num(&mut num_iter)?,
            minor: get_num(&mut num_iter)?,
            patch: get_num(&mut num_iter)?,
            hotfix: get_num(&mut num_iter)?,
        })
    }
}

impl std::fmt::Display for SimplexVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "v{}.{}.{}.{}",
            self.major, self.minor, self.patch, self.hotfix
        )
    }
}

impl std::fmt::Debug for SimplexVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SimplexVersion(")?;
        write!(f, "{self}")?;
        write!(f, ")")
    }
}

#[cfg(test)]
mod tests {
    use super::SimplexVersion;

    #[test]
    fn simplex_version_parse() {
        let current: SimplexVersion = "6.4.9.0".parse().unwrap();
        let old: SimplexVersion = "6.3.2.8".parse().unwrap();

        let min_supported = SimplexVersion::new(6, 4, 5, 2);
        let max_supported = SimplexVersion::new(6, 4, 10, 0);

        assert!(current >= min_supported && current <= max_supported);
        assert!(!(old >= min_supported && old <= max_supported));
    }
}
