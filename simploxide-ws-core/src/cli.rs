pub use simploxide_core::SimplexVersion;

use tokio::process::{Child, Command};

use std::{
    ffi::OsString,
    io,
    iter::{Chain, Empty, Once},
    process::Stdio,
};

/// An instance representing the running SimpleX CLI. Ensure to call [`SimplexCli::kill`] manually
/// to avoid zombie processes on Linux. The Drop impl tries its best to reap the process if it
/// wasn't killed by the user but it is not guarnteed to succeed.
pub struct SimplexCli {
    handle: Option<Child>,
    port: u16,
    version: SimplexVersion,
}

impl SimplexCli {
    const MIN_SUPPORTED_VERSION: SimplexVersion = SimplexVersion::new(6, 5, 0, 9);
    const MAX_SUPPORTED_VERSION: SimplexVersion = SimplexVersion::new(6, 5, 1, 0);

    /// Begin building a [`SimplexCli`] that will spawn a `simplex-chat` process.
    ///
    /// Call [`SimplexCliBuilder::spawn`] to launch the process after configuring the builder.
    pub fn builder(default_bot_name: impl Into<String>, port: u16) -> SimplexCliBuilder {
        SimplexCliBuilder {
            port,
            default_bot_name: default_bot_name.into(),
            db_path: "bot".into(),
            db_key: None,
            extra_args: std::iter::empty(),
        }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn version(&self) -> &SimplexVersion {
        &self.version
    }

    /// Kills the child process and waits for it to exit.
    pub async fn kill(&mut self) -> io::Result<()> {
        if let Some(mut handle) = self.handle.take() {
            handle.kill().await?;
        }

        Ok(())
    }
}

impl Drop for SimplexCli {
    fn drop(&mut self) {
        if let Some(ref mut handle) = self.handle {
            // Reap the process if it has already exited to avoid a zombie.
            // If it is still running, send SIGKILL and attempt an immediate reap
            // on the happy path where the process exits quickly after the signal.
            if handle.try_wait().ok().flatten().is_none() {
                let _ = handle.start_kill();
                let _ = handle.try_wait();
            }
        }
    }
}

/// Builder for [`SimplexCli`].
///
/// Obtained via [`SimplexCli::new`].
///
/// # Example
/// ```ignore
/// let cli = SimplexCli::new("Bot", 5225)
///     .db_path("/var/db/simplex")
///     .db_key(secret)
///     .arg("--smp-servers=smp://example.com")
///     .spawn()
///     .await?;
/// ```
pub struct SimplexCliBuilder<I = Empty<OsString>> {
    port: u16,
    default_bot_name: String,
    db_path: String,
    db_key: Option<String>,
    extra_args: I,
}

impl<I> SimplexCliBuilder<I>
where
    I: Iterator<Item = OsString>,
{
    /// Sets the path to the SimpleX database directory (defaults to `"."`).
    pub fn db_prefix(mut self, path: impl Into<String>) -> Self {
        self.db_path = path.into();
        self
    }

    /// Passes a database encryption key via the `-k` flag.
    pub fn db_key(mut self, key: impl Into<String>) -> Self {
        self.db_key = Some(key.into());
        self
    }

    /// Adds an extra command argument
    pub fn arg(self, arg: impl Into<OsString>) -> SimplexCliBuilder<Chain<I, Once<OsString>>> {
        SimplexCliBuilder {
            port: self.port,
            default_bot_name: self.default_bot_name,
            db_path: self.db_path,
            db_key: self.db_key,
            extra_args: self.extra_args.chain(std::iter::once(arg.into())),
        }
    }

    /// Adds multiple extra command arguments
    pub fn args<J>(self, args: J) -> SimplexCliBuilder<Chain<I, J::IntoIter>>
    where
        J: IntoIterator<Item = OsString>,
    {
        SimplexCliBuilder {
            port: self.port,
            default_bot_name: self.default_bot_name,
            db_path: self.db_path,
            db_key: self.db_key,
            extra_args: self.extra_args.chain(args),
        }
    }

    /// Spawns the `simplex-chat` process and returns a [`SimplexCli`] handle.
    ///
    /// Checks the installed CLI version against the supported range before spawning.
    pub async fn spawn(self) -> io::Result<SimplexCli> {
        let sxc_cmd = if std::path::Path::new("./simplex-chat").exists() {
            "./simplex-chat"
        } else {
            "simplex-chat"
        };

        let version_output = Command::new(sxc_cmd).arg("--version").output().await?;

        let output_str = String::from_utf8(version_output.stdout)
            .map_err(|_| io::Error::other("simplex-chat --version returned invalid string"))?;

        let version_str = output_str
            .lines()
            .next()
            .and_then(|line| line.trim().strip_prefix("SimpleX Chat v"))
            .ok_or_else(|| {
                io::Error::other(format!("Cannot parse SimpleX Chat version: {output_str:?}"))
            })?;

        let version: SimplexVersion = version_str.parse().map_err(|_| {
            io::Error::other(format!(
                "Cannot parse SimpleX Chat version: {version_str:?}"
            ))
        })?;

        if version < SimplexCli::MIN_SUPPORTED_VERSION
            || version > SimplexCli::MAX_SUPPORTED_VERSION
        {
            return Err(io::Error::other(format!(
                "The Simplex CLI {version} is incompatible with current simploxide version\n\
                Supported CLI versions: {}...{}",
                SimplexCli::MIN_SUPPORTED_VERSION,
                SimplexCli::MAX_SUPPORTED_VERSION
            )));
        }

        let mut cmd = Command::new(sxc_cmd);
        cmd.stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .arg("-d")
            .arg(&self.db_path)
            .arg("-p")
            .arg(self.port.to_string())
            .arg("--create-bot-display-name")
            .arg(&self.default_bot_name);

        if let Some(ref key) = self.db_key {
            cmd.arg("-k").arg(key);
        }

        cmd.args(self.extra_args);

        let handle = cmd.spawn()?;

        Ok(SimplexCli {
            handle: Some(handle),
            port: self.port,
            version,
        })
    }
}
