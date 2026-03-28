use std::{
    io,
    process::{Child, Command, Stdio},
    str::FromStr,
};

pub struct SimplexCli {
    handle: Option<Child>,
    port: u16,
}

impl SimplexCli {
    const MIN_SUPPORTED_VERSION: SimplexCliVersion = SimplexCliVersion::new(6, 4, 5, 0);
    const MAX_SUPPORTED_VERSION: SimplexCliVersion = SimplexCliVersion::new(6, 4, 10, 0);

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn kill(&mut self) -> io::Result<()> {
        if let Some(mut handle) = self.handle.take() {
            handle.kill()?;
            handle.wait()?;
        }

        Ok(())
    }

    pub fn external(port: u16) -> Self {
        Self { handle: None, port }
    }

    pub fn spawn(args: SimplexCliArgs) -> io::Result<Self> {
        let sxc_cmd = if std::path::Path::new("./simplex-chat").exists() {
            "./simplex-chat"
        } else {
            "simplex-chat"
        };

        let cli_version = SimplexCliVersion::read(sxc_cmd)?;

        if cli_version < Self::MIN_SUPPORTED_VERSION || cli_version > Self::MAX_SUPPORTED_VERSION {
            return Err(io::Error::other(format!(
                "The Simplex CLI {cli_version} is incompatible with current simploxide version\n\
                Supported CLI versions: {}...{}",
                Self::MIN_SUPPORTED_VERSION,
                Self::MAX_SUPPORTED_VERSION
            )));
        }

        let handle = Command::new(sxc_cmd)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .arg("-d")
            .arg(args.db_path)
            .arg("-p")
            .arg(args.port.to_string())
            .arg("--create-bot-display-name")
            .arg(args.default_bot_name)
            .spawn()?;

        Ok(Self {
            handle: Some(handle),
            port: args.port,
        })
    }
}

impl Drop for SimplexCli {
    fn drop(&mut self) {
        let _ = self.kill();
    }
}

pub struct SimplexCliArgs {
    pub db_path: String,
    pub default_bot_name: String,
    pub db_key: Option<String>,
    pub port: u16,
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
struct SimplexCliVersion {
    major: u8,
    minor: u8,
    patch: u8,
    hotfix: u8,
}

impl SimplexCliVersion {
    const fn new(major: u8, minor: u8, patch: u8, hotfix: u8) -> Self {
        Self {
            major,
            minor,
            patch,
            hotfix,
        }
    }

    fn read(sxc_cmd: &str) -> io::Result<Self> {
        let handle = Command::new(sxc_cmd).arg("--version").output()?;
        let output: String = handle
            .stdout
            .try_into()
            .map_err(|_| io::Error::other("simplex-chat --version returned invalid string"))?;

        let version = output
            .parse()
            .map_err(|_| io::Error::other("Cannot parse SimpleX Chat version: {output:?}"))?;

        Ok(version)
    }
}

impl FromStr for SimplexCliVersion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_line = s.lines().next().map(|line| line.trim()).ok_or(())?;
        let version_str = first_line.strip_prefix("SimpleX Chat v").ok_or(())?;

        let mut num_iter = version_str.split('.');

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

impl std::fmt::Display for SimplexCliVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "v{}.{}.{}.{}",
            self.major, self.minor, self.patch, self.hotfix
        )
    }
}

impl std::fmt::Debug for SimplexCliVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SimplexCliVersion(")?;
        write!(f, "{self}")?;
        write!(f, ")")
    }
}

#[cfg(test)]
mod tests {
    use super::SimplexCliVersion;

    #[test]
    fn simplex_cli_version() {
        let current: SimplexCliVersion = "SimpleX Chat v6.4.9.0".parse().unwrap();
        let old: SimplexCliVersion = "SimpleX Chat v6.3.2.8".parse().unwrap();

        let min_supported = SimplexCliVersion::new(6, 4, 5, 2);
        let max_supported = SimplexCliVersion::new(6, 4, 10, 0);

        assert!(current >= min_supported && current <= max_supported);
        assert!(!(old >= min_supported && old <= max_supported));
    }
}
