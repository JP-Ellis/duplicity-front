use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::path;

use serde_yaml;

use repository::Repository;
use error::Error;

pub struct Config {
    pub repositories: HashMap<String, Repository>,
}

impl Config {
    /// Load configuration from file.
    ///
    /// This reads the configuration from the specified file and checks that it
    /// is sane.
    pub fn parse_file<S>(s: &S) -> Result<Config, Error>
    where
        S: AsRef<path::Path>,
    {
        info!("Loading configuration from file: {}", s.as_ref().display());

        // If the path starts with '~', we have to strip that and replace it
        // with the home_dir path, except that we have to ensure that `home_dir`
        // is well defined.
        let p =
            if s.as_ref().starts_with("~") {
                env::home_dir()
                    .ok_or_else(|| {
                        Error::new(
                    "Config path starts with '~' but the home directory could not be located.",
                )
                    })
                    .map(|home_path| {
                        home_path.join(s.as_ref().strip_prefix("~").expect(
                            "Unable to strip prefix.  This is a bug and should be reported.",
                        ))
                    })?
            } else {
                s.as_ref().to_path_buf()
            };

        Config::check_permissions(&p)?;

        let p = p.canonicalize().map_err(|e| {
            Error::new(format!(
                "Error when canonicalizing configuration path: {}",
                e
            ))
        })?;

        File::open(&p)
            .map_err(|e| Error::new(format!("Error when opening configuration file: {}", e)))
            .and_then(Config::from_reader)
    }

    fn from_reader<I>(reader: I) -> Result<Self, Error>
    where
        I: io::Read,
    {
        serde_yaml::from_reader(reader)
            .map(|repositories| Config { repositories })
            .map_err(|e| Error::new(format!("Error when parsing configuration file: {}", e)))
            .and_then(|config| config.check().and(Ok(config)))
    }

    /// Check that the configuration is sane.
    ///
    /// This will check that each repository is sane, and that sub-repositories
    /// listed in one exist within the same configuration.
    pub fn check(&self) -> Result<(), Error> {
        debug!("Checking configuration is sane.");

        for (name, repository) in &self.repositories {
            if let Err(e) = repository.check() {
                return Err(Error::new(format!("Error in repository {}: {}", name, e)));
            }

            if repository.has_sub_repositories() {
                for sub_name in &repository.sub_repositories {
                    if !self.repositories.contains_key(sub_name) {
                        return Err(Error::new(format!(
                            "Repository {} lists {} as a sub-repository, but it could not be located within config file.",
                            name,
                            sub_name
                        )));
                    }
                }
            }
        }

        Ok(())
    }

    #[cfg(target_family = "unix")]
    /// Check the permissions on the config file and warn the user if they are
    /// readable to anyone else but the user only.
    ///
    /// This currently only works on Unix.
    fn check_permissions<S>(s: &S) -> Result<(), Error>
    where
        S: AsRef<path::Path>,
    {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;

        let mode = fs::metadata(s)
            .map_err(|e| Error::new(format!("Error when getting config permissions: {}", e)))?
            .permissions()
            .mode();

        // Check that
        if mode & 0o077 != 0 {
            warn!(
                "It is recommended that your configuration file be not readable to anyone except for the user."
            );
        }

        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    /// Check the permissions on the config file and warn the user if they are
    /// readable to anyone else but the user only.
    ///
    /// This currently only works on Unix.
    fn check_permissions<S>(s: &S) -> Result<(), Error>
    where
        S: AsRef<path::Path>,
    {
        // Do nothing on non-linux
        Ok(())
    }
}
