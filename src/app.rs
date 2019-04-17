/// Clap App Constructor
use clap::{crate_authors, crate_version, App, AppSettings, Arg, SubCommand};

/// Construct the repository argument that is used in all subcommands.
fn repository_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("repository")
        .required(true)
        .takes_value(true)
        .help("Repository to backup")
        .long_help("Repository to backup, as set in the configuration file.")
}

/// Backup subcommand
fn backup<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("backup")
        .about("Backup the specified repository")
        .long_about(
            "\
Backup the specified repository, with the repository being as defined in the \
configuration file.

The order in which repositories are specified determines the order in which they \
are executed, so it is advisable to add ones requiring 'sudo' first in order to \
avoid timeouts.

If the repository has any of the 'remove-older-than', 'remove-all-but-n-full' or \
'remove-all-inc-of-but-n-full' options, successful completion of the backup \
(whether full or incremental) will automatically be followed by the appropriate \
commands above.",
        )
        .display_order(1)
        .arg(repository_arg())
}

/// Verify subcommand
fn verify<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("verify")
        .about("Verify the backup against the original files")
        .long_about(
            "
Verify the backup against the original files.  See duplicity's manual for more \
information about the options below.
",
        )
        .arg(repository_arg())
        .arg(
            Arg::with_name("compare-data")
                .long("compare-data")
                .help("Enables data comparison (refer to duplicity manual)."),
        )
        .arg(
            Arg::with_name("time")
                .long("time")
                .takes_value(true)
                .value_name("TIME")
                .multiple(false)
                .help(
                    "\
                     Selects a backup to verify against (refer to duplicity manual).",
                ),
        )
        .arg(
            Arg::with_name("file-to-restore")
                .long("file-to-resotre")
                .takes_value(true)
                .value_name("RELPATH")
                .multiple(false)
                .help(
                    "\
                     Restrict verify to that file or folder (refer to duplicity manual).",
                ),
        )
}

/// Collection status subcommand
fn collection_status<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("collection-status")
        .about("Summarize the stats of the backup repository")
        .long_about(
            "\
Summarize the status of the backup repository by printing the chains and sets \
found, and the number of volumes in each.
",
        )
        .arg(repository_arg())
        .arg(
            Arg::with_name("file-changed")
                .long("file-changed")
                .takes_value(true)
                .value_name("RELPATH")
                .multiple(false)
                .help(
                    "\
                     Causes only the specified path status to be collected instead of the entire \
                     contents of the backup archive (refer to duplicity manual).",
                ),
        )
}

/// Cleanup subcommand
fn cleanup<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("cleanup")
        .about("Delete extraneous duplicity files in the backup location")
        .long_about(
            "\
             Delete extraneous duplicity files in the backup location.  Non-duplicity files \
             and files in complete data sets will not be deleted.  This should only be \
             necessary after a duplicity sessions fails or is aborted.  Note that '--force' \
             is required to actually delete the files instead of just listing them.",
        )
        .arg(repository_arg())
        .arg(Arg::with_name("force").long("force").help(
            "\
             Delete the files instead of just listing them (refer to duplicity manual).",
        ))
        .arg(Arg::with_name("extra-clean").long("extra-clean").help(
            "\
             USE WITH CAUTION.  When cleaning up, be more aggressive about saving space \
             (refer to duplicity manual).",
        ))
}

/// List current files subcommand
fn list_current_files<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("list-current-files")
        .about("List the files contained in the backup")
        .long_about("List the files contained in the backup.")
        .arg(repository_arg())
        .arg(
            Arg::with_name("time")
                .long("time")
                .takes_value(true)
                .value_name("TIME")
                .multiple(false)
                .help(
                    "\
                     Selects a backup to list files from instead of the latest (refer to duplicity \
                     manual).",
                ),
        )
}

/// Final construct for everything
pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("duplicity-front")
        .author(crate_authors!())
        .version(crate_version!())
        .max_term_width(100)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .global_setting(AppSettings::VersionlessSubcommands)
        .about(
            "\
duplicity-front is a front end to the duplicity backup utility \
(http://duplicity.nongnu.org/), providing support for pre-configured remotes and \
making certain routine tasks easier.

Project home page: https://github.com/JP-Ellis/duplicity-front",
        )
        .long_about(
            "
duplicity-front is a front end to the duplicity backup utility \
(http://duplicity.nongnu.org/), providing support for pre-configured remotes and \
making certain routine tasks easier.

Typically, all options need to be specified every time duplicity is run, \
including the full source and target paths as well as which files to exclude (or \
include), and other possible options.  This front to duplicity allows for these \
common options to be specified in a YAML config file.

An example YAML configuration file is provided in the Github repository and \
should be used as a reference.  This program will parse the YAML file and make \
sure it is sane before proceeding, but it is always advisable to first use the \
'--dry-run' option to ensure that nothing unexpected happens before making \
permanent changes.

Project home page: https://github.com/JP-Ellis/duplicity-front",
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .global(true)
                .value_name("FILE")
                .takes_value(true)
                .default_value("~/.config/duplicity-front.yml")
                .number_of_values(1)
                .help("Configuration file name")
                .long_help(
                    "\
Specify the configuration file name containing information about backup \
repositories.",
                ),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .global(true)
                .multiple(true)
                .takes_value(false)
                .help("Increase the verbosity of errors")
                .long_help(
                    "\
Increase the verbosity of errors.  The errors are outputted to the standard \
error stream and thus do not appear in the output.  This option can be specified \
multiple times for increasing levels of verbosity.",
                ),
        )
        .arg(
            Arg::with_name("dry-run")
                .short("n")
                .long("dry-run")
                .global(true)
                .takes_value(false)
                .help("Perform a dry run")
                .long_help(
                    "\
Perform a run and calculate what will be changed, but take no action.",
                ),
        )
        .subcommand(backup())
        .subcommand(verify())
        .subcommand(collection_status())
        .subcommand(list_current_files())
        .subcommand(cleanup())
}
