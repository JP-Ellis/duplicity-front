// duplicity-front
// Copyright (C) 2017  Joshua Ellis <josh@jpellis.me>
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// this program.  If not, see <http://www.gnu.org/licenses/>.

#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

mod app;
mod config;
mod error;
mod repository;

use std::process::Command;
use std::process::exit;

use config::Config;
use error::Error;
use repository::Repository;

/// Initialize the logger based on the desired level of verbosity.
fn initialize_logger(level: u64) {
    let mut log_builder = env_logger::LogBuilder::new();
    match level {
        0 => log_builder.filter(None, log::LogLevelFilter::Error),
        1 => log_builder.filter(None, log::LogLevelFilter::Warn),
        2 => log_builder.filter(None, log::LogLevelFilter::Info),
        3 | _ => log_builder.filter(None, log::LogLevelFilter::Debug),
    };
    log_builder.format(|record: &log::LogRecord| {
        format!("{}: {}", record.level(), record.args())
    });
    if let Err(e) = log_builder.init() {
        eprintln!("Error when initializing the logger: {}.", e);
        exit(1);
    }

    debug!("Verbosity set to Debug.");
}

/// Load a particular repository from the configuration.
///
/// If the repository could not be found in the config, an error is returned.
fn load_repository<S>(name: S, config: &Config) -> Result<&Repository, Error>
where
    S: Into<String>,
{
    let name: String = name.into();
    config.repositories.get(&name).ok_or_else(|| {
        Error::new(format!(
            "Repository {} could not be loaded from the configuration.",
            name
        ))
    })
}

/// Construct the initial duplicity command.
///
/// This ensures that the environment is correctly set up, and uses `sudo` if
/// needed.
fn duplicity_cmd(repository: &Repository) -> Command {
    let mut cmd = if repository.sudo {
        let mut cmd = Command::new("sudo");
        cmd.arg("--preserve-env=PASSPHRASE");
        cmd.arg("duplicity");
        cmd
    } else {
        Command::new("duplicity")
    };

    if let Some(ref passphrase) = repository.passphrase {
        cmd.env("PASSPHRASE", passphrase);
    }

    cmd
}

/// Run the specified duplicity command and check that it exits correctly, or
/// returns an error as appropriate.
fn run_and_check_command(cmd: &mut Command) -> Result<(), Error> {
    info!("command: {:?}", cmd);
    let mut child = cmd.spawn().map_err(|e| {
        Error::new(format!("Error when spawning subprocess: {}", e))
    })?;

    let ecode = child.wait().map_err(|e| {
        Error::new(format!("Error when waiting subprocess: {}", e))
    })?;

    if ecode.success() {
        Ok(())
    } else {
        Err(Error::new("Subprocess encountered an error."))
    }
}

/// Run a backup
fn backup(matches: &clap::ArgMatches, config: &Config, name: Option<&str>) -> Result<(), Error> {
    let repository = load_repository(
        name.or_else(|| matches.value_of("repository")).expect(
            "Unable to unwrap repository name.",
        ),
        config,
    )?;

    // If we have sub-repositories, recurse into them.  Note that if this has
    // no sub-repositories, there will be nothing to iterate over.
    for subname in &repository.sub_repositories {
        backup(matches, config, Some(subname))?;
    }

    if let (&Some(ref source), &Some(ref remote)) = (&repository.source, &repository.remote) {
        let mut cmd = duplicity_cmd(repository);
        if matches.is_present("dry-run") {
            cmd.arg("--dry-run");
        }

        cmd.args(repository.construct_flags()).arg(source).arg(
            remote,
        );
        run_and_check_command(&mut cmd)?;

        if let Some(ref arg) = repository.remove_older_than {
            let mut cmd = duplicity_cmd(repository);
            if matches.is_present("dry-run") {
                cmd.arg("--dry-run");
            }
            cmd.arg("remove_older_than");
            cmd.arg(arg);
            cmd.arg("--force");
            cmd.arg(remote);

            run_and_check_command(&mut cmd)?;
        }

        if let Some(arg) = repository.remove_all_inc_of_but_n_full {
            let mut cmd = duplicity_cmd(repository);
            if matches.is_present("dry-run") {
                cmd.arg("--dry-run");
            }
            cmd.arg("remove-all-inc-of-but-n-full");
            cmd.arg(arg.to_string());
            cmd.arg("--force");
            cmd.arg(remote);

            run_and_check_command(&mut cmd)?;
        }

        if let Some(arg) = repository.remove_all_but_n_full {
            let mut cmd = duplicity_cmd(repository);
            if matches.is_present("dry-run") {
                cmd.arg("--dry-run");
            }
            cmd.arg("remove-all-but-n-full");
            cmd.arg(arg.to_string());
            cmd.arg("--force");
            cmd.arg(remote);

            run_and_check_command(&mut cmd)?;
        }
    }

    Ok(())
}

fn cleanup(matches: &clap::ArgMatches, config: &Config, name: Option<&str>) -> Result<(), Error> {
    let repository = load_repository(
        name.or_else(|| matches.value_of("repository")).expect(
            "Unable to unwrap repository name.",
        ),
        config,
    )?;

    // If we have sub-repositories, recurse into them.  Note that if this has
    // no sub-repositories, there will be nothing to iterate over.
    for subname in &repository.sub_repositories {
        cleanup(matches, config, Some(subname))?;
    }

    if let (&Some(_), &Some(ref remote)) = (&repository.source, &repository.remote) {
        let mut cmd = duplicity_cmd(repository);
        if matches.is_present("dry-run") {
            cmd.arg("--dry-run");
        }

        cmd.arg("cleanup");
        if matches.is_present("force") {
            cmd.arg("--force");
        }
        if matches.is_present("extra-clean") {
            cmd.arg("--extra-clean");
        }
        cmd.arg(remote);

        run_and_check_command(&mut cmd)?;
    }

    Ok(())
}

fn collection_status(
    matches: &clap::ArgMatches,
    config: &Config,
    name: Option<&str>,
) -> Result<(), Error> {
    let repository = load_repository(
        name.or_else(|| matches.value_of("repository")).expect(
            "Unable to unwrap repository name.",
        ),
        config,
    )?;

    // If we have sub-repositories, recurse into them.  Note that if this has
    // no sub-repositories, there will be nothing to iterate over.
    for subname in &repository.sub_repositories {
        collection_status(matches, config, Some(subname))?;
    }

    if let (&Some(_), &Some(ref remote)) = (&repository.source, &repository.remote) {
        let mut cmd = duplicity_cmd(repository);
        if matches.is_present("dry-run") {
            cmd.arg("--dry-run");
        }

        cmd.arg("collection-status");
        if matches.is_present("file-changed") {
            cmd.arg("--file-changed");
            cmd.arg(matches.value_of("file-changed").unwrap());
        }
        cmd.arg(remote);

        run_and_check_command(&mut cmd)?;
    }

    Ok(())
}

fn list_current_files(
    matches: &clap::ArgMatches,
    config: &Config,
    name: Option<&str>,
) -> Result<(), Error> {
    let repository = load_repository(
        name.or_else(|| matches.value_of("repository")).expect(
            "Unable to unwrap repository name.",
        ),
        config,
    )?;

    // If we have sub-repositories, recurse into them.  Note that if this has
    // no sub-repositories, there will be nothing to iterate over.
    for subname in &repository.sub_repositories {
        list_current_files(matches, config, Some(subname))?;
    }

    if let (&Some(_), &Some(ref remote)) = (&repository.source, &repository.remote) {
        let mut cmd = duplicity_cmd(repository);
        if matches.is_present("dry-run") {
            cmd.arg("--dry-run");
        }

        cmd.arg("list-current-files");
        if matches.is_present("time") {
            cmd.arg("--time");
            cmd.arg(matches.value_of("time").unwrap());
        }
        cmd.arg(remote);

        run_and_check_command(&mut cmd)?;
    }

    Ok(())
}

fn verify(matches: &clap::ArgMatches, config: &Config, name: Option<&str>) -> Result<(), Error> {
    let repository = load_repository(
        name.or_else(|| matches.value_of("repository")).expect(
            "Unable to unwrap repository name.",
        ),
        config,
    )?;

    // If we have sub-repositories, recurse into them.  Note that if this has
    // no sub-repositories, there will be nothing to iterate over.
    for subname in &repository.sub_repositories {
        verify(matches, config, Some(subname))?;
    }

    if let (&Some(_), &Some(ref remote)) = (&repository.source, &repository.remote) {
        let mut cmd = duplicity_cmd(repository);
        if matches.is_present("dry-run") {
            cmd.arg("--dry-run");
        }

        cmd.arg("verify");
        if matches.is_present("compare-data") {
            cmd.arg("--compare-data");
        }
        if matches.is_present("time") {
            cmd.arg("--time");
            cmd.arg(matches.value_of("time").unwrap());
        }
        if matches.is_present("file-to-restore") {
            cmd.arg("--file-to-restore");
            cmd.arg(matches.value_of("file-to-restore").unwrap());
        }
        cmd.arg(remote);

        run_and_check_command(&mut cmd)?;
    }

    Ok(())
}

/// Main function
fn main() {
    // Parse the arguments, and immediately initialize the logger.
    let matches = app::app().get_matches();
    initialize_logger(matches.occurrences_of("verbose"));

    // Load the configuration and make sure it is all fine.
    let config = match Config::parse_file(&matches.value_of("config").unwrap()) {
        Ok(c) => c,
        Err(e) => {
            error!("Error when loading configuration: {}", e);
            exit(1)
        }
    };

    if let Err(e) = match matches.subcommand() {
        ("backup", Some(sub_matches)) => backup(sub_matches, &config, None),
        ("cleanup", Some(sub_matches)) => cleanup(sub_matches, &config, None),
        ("collection-status", Some(sub_matches)) => collection_status(sub_matches, &config, None),
        ("list-current-files", Some(sub_matches)) => list_current_files(sub_matches, &config, None),
        ("verify", Some(sub_matches)) => verify(sub_matches, &config, None),
        (s, sub_matches) => {
            error!(
                "\
Unhandled sub-command {} with matches {:?}.  This is a bug and should be \
reported.",
                s,
                sub_matches
            );
            exit(1)
        }
    }
    {
        error!("{}", e);
        exit(1)
    }
}
