
use error::Error;

/// Simple function used to determine whether a particular flag should be
/// serialized or not through `skip_serializing_if`.
///
/// By default, all flags are set to `false` and thus are not serialized.
fn is_false(arg: &bool) -> bool {
    !arg
}


/// Repository options.
///
/// This is a (very ugly) struct containing all the various options which can be
/// used for a repository.  Most of them are simply flags from `duplicity`
/// though there are a couple of custom options.
///
/// In particular, `sub_repositories` can specify a list of names of other
/// repositories which should be run.
///
/// Note that deserializing a repository need not result in a valid repository
/// as the sanity checks are more complicated.  To do this, the
/// `Repository::check()` function must be used.
#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields, default)]
pub struct Repository {
    // Custom Options
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sub_repositories: Vec<String>,

    #[serde(skip_serializing_if = "is_false")]
    pub sudo: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passphrase: Option<String>,

    // Default options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_older_than: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_all_but_n_full: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_all_inc_of_but_n_full: Option<u64>,
    #[serde(skip_serializing_if = "is_false")]
    pub asynchronous_upload: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_retry_delay: Option<u64>,
    #[serde(skip_serializing_if = "is_false")]
    pub compare_data: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub copy_links: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt_secret_keyring: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt_sign_key: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub exclude_device_files: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exclude_filelist: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exclude_if_present: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_older_than: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub exclude_other_filesystems: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exclude_regexp: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_prefix_manifest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_prefix_archive: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_prefix_signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_if_older_than: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub ftp_passive: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub ftp_regular: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub gio: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_encrypt_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imap_full_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imap_mailbox: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpg_binary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpg_options: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub include_filelist: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub include_regexp: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_blocksize: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub no_compression: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub no_encryption: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub no_print_statistics: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub null_separator: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub numeric_owner: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_retries: Option<u64>,
    #[serde(skip_serializing_if = "is_false")]
    pub old_filenames: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par2_options: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par2_redundancy: Option<u64>,
    #[serde(skip_serializing_if = "is_false")]
    pub progress: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<u64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rename: Vec<(String, String)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsync_options: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub short_filenames: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_key: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub ssh_askpass: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_options: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tempdir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_separator: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
    #[serde(skip_serializing_if = "is_false")]
    pub use_agent: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volsize: Option<u64>,
}

impl Repository {
    /// Check that a repository has self-consistent options.  The first invalid
    /// option is returned as an error.
    #[allow(dead_code)]
    pub fn check(&self) -> Result<(), Error> {
        match (
            self.source.is_some(),
            self.remote.is_some(),
            self.sub_repositories.len(),
        ) {
            // No source or remote
            (false, false, 0) => Err(Error::new(
                "Repository must either specify both a source and a remote, or list sub-repositories.",
            )),
            (false, false, _) => self.check_sub_repository_options(),

            // Source xor remote
            (false, true, 0) | (true, false, 0) => Err(Error::new(
                "Both source and remote must be simultaneously specified.",
            )),
            (false, true, _) | (true, false, _) => Err(Error::new(
                "Both source and remote must be simultaneously specified, and sub-repositories cannot be simultaneously listed.",
            )),

            (true, true, 0) => self.check_repository_options(),
            (true, true, _) => Err(Error::new(
                "Sub-repositories cannot be simultaneously specified with source and remote.",
            )),
        }
    }

    /// Check that a repository not containing sub-repositories has
    /// self-consistent options.
    fn check_repository_options(&self) -> Result<(), Error> {
        if !self.source.is_some() {
            Err(Error::new("Repository must specify a source."))
        } else if !self.remote.is_some() {
            Err(Error::new(
                "Repository must specify a remote location for backups.",
            ))
        } else if !self.sub_repositories.is_empty() {
            Err(Error::new(
                "Repository cannot simultaneously have a source and/or remote with sub-repositories.",
            ))
        } else {
            Ok(())
        }
    }

    /// Check that a repository containing sub-repositories has self-consistent options.
    fn check_sub_repository_options(&self) -> Result<(), Error> {
        if self.source.is_some() {
            Err(Error::new(
                "Sub-repositories cannot be specified alongside 'source'.",
            ))
        } else if self.remote.is_some() {
            Err(Error::new(
                "Sub-repositories cannot be specified alongside 'remote'.",
            ))
        } else if self.sub_repositories.is_empty() {
            Err(Error::new(
                "Sub-repositories must specify at least one sub-repository.",
            ))
        } else {
            Ok(())
        }
    }

    /// Check whether the repository lists sub repositories.
    pub fn has_sub_repositories(&self) -> bool {
        !self.sub_repositories.is_empty()
    }

    #[allow(cyclomatic_complexity)]
    pub fn construct_flags(&self) -> Vec<String> {
        let mut flags: Vec<String> = Vec::new();

        // First handle the includes
        for arg in &self.include {
            flags.push("--include".into());
            flags.push(arg.to_string());
        }
        for arg in &self.include_filelist {
            flags.push("--include-filelist".into());
            flags.push(arg.to_string());
        }
        for arg in &self.include_regexp {
            flags.push("--include-regexp".into());
            flags.push(arg.to_string());
        }
        // Then handle the exclusions
        for arg in &self.exclude {
            flags.push("--exclude".into());
            flags.push(arg.to_string());
        }
        if self.exclude_device_files {
            flags.push("--exclude-device-files".into());
        }
        for arg in &self.exclude_filelist {
            flags.push("--exclude-filelist".into());
            flags.push(arg.to_string());
        }
        for arg in &self.exclude_if_present {
            flags.push("--exclude-if-present".into());
            flags.push(arg.to_string());
        }
        if let Some(ref arg) = self.exclude_older_than {
            flags.push("--exclude-older-than".into());
            flags.push(arg.to_string());
        }
        if self.exclude_other_filesystems {
            flags.push("--exclude-other-filesystems".into());
        }
        for arg in &self.exclude_regexp {
            flags.push("--exclude-regexp".into());
            flags.push(arg.to_string());
        }

        // And finally, all the remaining arguments
        if self.asynchronous_upload {
            flags.push("--asynchronous-upload".into());
        }
        if let Some(arg) = self.backend_retry_delay {
            flags.push("--backend-retry-delay".into());
            flags.push(arg.to_string());
        }
        if self.compare_data {
            flags.push("--compare-data".into());
        }
        if self.copy_links {
            flags.push("--copy-links".into());
        }
        if let Some(ref arg) = self.encrypt_key {
            flags.push("--encrypt-key".into());
            flags.push(arg.to_string())
        }
        if let Some(ref arg) = self.encrypt_secret_keyring {
            flags.push("--encrypt-secret-keyring".into());
            flags.push(arg.to_string())
        }
        if let Some(ref arg) = self.encrypt_sign_key {
            flags.push("--encrypt-sign-key".into());
            flags.push(arg.to_string());
        }
        if let Some(ref arg) = self.file_prefix {
            flags.push("--file-prefix".into());
            flags.push(arg.to_string());
        }
        if let Some(ref arg) = self.file_prefix_manifest {
            flags.push("--file-prefix-manifest".into());
            flags.push(arg.to_string());
        }
        if let Some(ref arg) = self.file_prefix_archive {
            flags.push("--file-prefix-archive".into());
            flags.push(arg.to_string());
        }
        if let Some(ref arg) = self.file_prefix_signature {
            flags.push("--file-prefix-signature".into());
            flags.push(arg.to_string());
        }
        if let Some(ref arg) = self.full_if_older_than {
            flags.push("--full-if-older-than".into());
            flags.push(arg.to_string());
        }
        if self.ftp_passive {
            flags.push("--ftp-passive".into());
        }
        if self.ftp_regular {
            flags.push("--ftp-regular".into());
        }
        if self.gio {
            flags.push("--gio".into());
        }
        if let Some(ref arg) = self.hidden_encrypt_key {
            flags.push("--hidden-encrypt-key".into());
            flags.push(arg.to_string());
        }
        if let Some(ref arg) = self.imap_full_address {
            flags.push("--imap-full-address".into());
            flags.push(arg.to_string());
        }
        if let Some(ref arg) = self.imap_mailbox {
            flags.push("--imap-mailbox".into());
            flags.push(arg.to_string());
        }
        if let Some(ref arg) = self.gpg_binary {
            flags.push("--gpg-binary".into());
            flags.push(arg.to_string());
        }
        if let Some(ref arg) = self.gpg_options {
            flags.push("--gpg-options".into());
            flags.push(arg.to_string());
        }
        if let Some(ref arg) = self.log_file {
            flags.push("--log-file".into());
            flags.push(arg.to_string());
        }
        if let Some(ref arg) = self.max_blocksize {
            flags.push("--max-blocksize".into());
            flags.push(arg.to_string());
        }
        if let Some(ref arg) = self.name {
            flags.push("--name".into());
            flags.push(arg.to_string());
        }
        if self.no_compression {
            flags.push("--no-compression".into());
        }
        if self.no_encryption {
            flags.push("--no-encryption".into());
        }
        if self.no_print_statistics {
            flags.push("--no-print-statistics".into());
        }
        if self.null_separator {
            flags.push("--null-separator".into());
        }
        if self.numeric_owner {
            flags.push("--numeric-owner".into());
        }
        if let Some(arg) = self.num_retries {
            flags.push("--num-retries".into());
            flags.push(arg.to_string());
        }
        if self.old_filenames {
            flags.push("--old-filenames".into());
        }
        if let Some(ref arg) = self.par2_options {
            flags.push("--par2-options".into());
            flags.push(arg.to_string());
        }
        if let Some(arg) = self.par2_redundancy {
            flags.push("--par2-redundancy".into());
            flags.push(arg.to_string());
        }
        if self.progress {
            flags.push("--progress".into());
        }
        if let Some(arg) = self.progress_rate {
            flags.push("--progress-rate".into());
            flags.push(arg.to_string());
        }
        for &(ref arg1, ref arg2) in &self.rename {
            flags.push("--rename".into());
            flags.push(arg1.to_string());
            flags.push(arg2.to_string());
        }
        if let Some(ref arg) = self.rsync_options {
            flags.push("--rsync-options".into());
            flags.push(arg.to_string());
        }
        if self.short_filenames {
            flags.push("--short-filenames".into());
        }
        if let Some(ref arg) = self.sign_key {
            flags.push("--sign-key".into());
            flags.push(arg.to_string())
        }
        if self.ssh_askpass {
            flags.push("--ssh-askpass".into());
        }
        if let Some(ref arg) = self.ssh_options {
            flags.push("--ssh-options".into());
            flags.push(arg.to_string());
        }
        if let Some(ref arg) = self.tempdir {
            flags.push("--tempdir".into());
            flags.push(arg.to_string());
        }
        if let Some(arg) = self.time_separator {
            flags.push("--time-separator".into());
            flags.push(arg.to_string());
        }
        if let Some(arg) = self.timeout {
            flags.push("--timeout".into());
            flags.push(arg.to_string());
        }
        if self.use_agent {
            flags.push("--use-agent".into());
        }
        if let Some(arg) = self.volsize {
            flags.push("--volsize".into());
            flags.push(arg.to_string());
        }

        flags
    }
}

#[cfg(test)]
mod test {
    use super::Repository;
    use serde_yaml;
    use std::collections::HashMap;

    #[test]
    fn serialization() {
        let mut default_repository1 = Repository::default();
        let yml = serde_yaml::to_string(&default_repository1).unwrap();
        assert_eq!(
            &serde_yaml::to_string(&default_repository1).unwrap(),
            "---\n{}"
        );

        let mut default_repository2 = Repository::default();
        default_repository2.source = Some("~/".to_owned());
        default_repository2.remote = Some("ssh://user@host//backup/location".to_owned());
        assert_eq!(
            &serde_yaml::to_string(&default_repository2).unwrap(),
            r#"---
source: ~/
remote: "ssh://user@host//backup/location""#
        );

        let mut default_repository3 = Repository::default();
        default_repository3.sub_repositories =
            vec!["foo".to_owned(), "foo:bar".to_owned(), "foo:baz".to_owned()];
        assert_eq!(
            &serde_yaml::to_string(&default_repository3).unwrap(),
            r#"---
sub_repositories: 
  - foo
  - "foo:bar"
  - "foo:baz""#
        );

        let mut hm = HashMap::new();
        hm.insert("foo".to_owned(), default_repository1);
        hm.insert("foo:bar".to_owned(), default_repository2);
        hm.insert("foo:baz".to_owned(), default_repository3);
        assert_eq!(
            &hm,
            &serde_yaml::from_str(
                r#"
foo: {}
foo:baz:
  sub_repositories:
    - foo
    - foo:bar
    - foo:baz
foo:bar:
  source: ~/
  remote: ssh://user@host//backup/location"#,
            ).unwrap()
        );
    }
}
