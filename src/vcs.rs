// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::error::VcsError;
use log::debug;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Run a shell command from user's system.
///
/// # Preconditions
///
/// 1. Shell command exists.
///
/// # Postconditions
///
/// 1. Execute shell command.
/// 2. Retrieve shell command's output.
///
/// # Arguments
///
/// * `cmd`(in) - Name of shell command to execute.
/// * `args`(in) - Arguments to pass to shell command.
///
/// # Returns
///
/// * `Result<String, VcsError>` - String of stdout on success, or VcsError on
///   failure.
///
/// # Since
///
/// 0.2.0
fn execute(cmd: &str, args: &[&str]) -> Result<String, VcsError> {
    let fullcmd: String = format!("{} {}", cmd, args.join(" ")).into();
    debug!("Execute {}", fullcmd);

    let output = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|error| VcsError::ShellCmdSpawnError { cmd: fullcmd.clone(), errsrc: error })?;

    if !output.status.success() {
        let errmsg = String::from_utf8(output.stderr)
            .map_err(|error| VcsError::ShellCmdUtf8Error { cmd: fullcmd.clone(), errsrc: error })?;

        return Err(VcsError::ShellCmdError { cmd: fullcmd.clone(), stderr: errmsg });
    }

    let outmsg = String::from_utf8(output.stdout)
        .map_err(|error| VcsError::ShellCmdUtf8Error { cmd: fullcmd.clone(), errsrc: error })?;

    Ok(outmsg)
}

/// Git VCS API.
pub struct Git {
    /// URL to remote repository to push and pull changes too and from.
    remote: String,

    /// Path to git directory (.git/).
    git_dir: PathBuf,

    /// Path to working tree to stage changes from.
    work_tree: PathBuf,
}

/// Git VCS manipulation API.
///
/// Simple API to manipulate the Git VCS command-line tool that the user (should) have installed on
/// their system.
///
/// # Since
///
/// 0.2.0
impl Git {
    /// Constructor to create new Git VCS instance.
    ///
    /// # Preconditions
    ///
    /// 1. Valid remote URL.
    /// 2. Valid path to git directory.
    /// 3. Valid path to working tree.
    ///
    /// # Postconditions
    ///
    /// 1. Valid Git VCS instance given.
    ///
    /// # Arguments
    ///
    /// * `remote`(in) - URL to remote repository.
    /// * `git_dir`(in) - Path to git directory (.git).
    /// * `work_tree`(in) - Path to working tree.
    ///
    /// # Returns
    ///
    /// * Valid Git VCS instance to use.
    ///
    /// # Since
    ///
    /// 0.2.0
    pub fn new(remote: &str, git_dir: &Path, work_tree: &Path) -> Result<Git, VcsError> {
        let gitout = execute("git", &["ls-remote", remote])?;
        debug!("Results of verifying remote url - {}", gitout);

        if !git_dir.is_dir() {
            return Err(VcsError::BadPathError { path: git_dir.to_path_buf() });
        }

        if !work_tree.is_dir() {
            return Err(VcsError::BadPathError { path: work_tree.to_path_buf() });
        }

        Ok(Git {
            remote: remote.to_string(),
            git_dir: git_dir.to_path_buf(),
            work_tree: work_tree.to_path_buf(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test execute() function.
    ///
    /// # Expectations
    ///
    /// 1. Get VcsError::ShellCmdSpawnError for invalid command.
    /// 2. Get VcsError::ShellCmdUtf8Error for invalid UTF-8 in stderr.
    /// 3. Get VcsError::ShellCmdUtf8Error for invalid UTF-8 in stdout.
    /// 4. Get VcsError::ShellCmdError for command exiting non-successfully.
    /// 5. Execute shell command without failure.
    ///
    /// # Since
    ///
    /// 0.2.0
    #[test]
    fn test_execute() {
        let badcmd = execute("deadbeef", &["--bad"]);
        assert!(matches!(badcmd, Err(VcsError::ShellCmdSpawnError { .. })));

        let mut badutf8 = execute("sh", &["-c", "printf \"\\000\\237\" 1>&2 && exit 1"]);
        assert!(matches!(badutf8, Err(VcsError::ShellCmdUtf8Error { .. })));

        badutf8 = execute("sh", &["-c", "printf \"\\000\\237\" && exit 0"]);
        assert!(matches!(badutf8, Err(VcsError::ShellCmdUtf8Error { .. })));

        let cmdfail = execute("ls", &["--bad-option"]);
        assert!(matches!(cmdfail, Err(VcsError::ShellCmdError { .. })));

        let goodcmd = execute("echo", &["hello world"]);
        assert_eq!("hello world\n", goodcmd.unwrap());
    }

    /// Test `Git::new()` method.
    ///
    /// # Expectations
    ///
    /// 1. Get VcsError::ShellCmdError for invalid remote URL.
    /// 2. Get VcsError::BadPathError for invalid git directory path.
    /// 3. Get VcsError::BadPathError for invalid work tree directory path.
    ///
    /// # Since
    ///
    /// 0.2.0
    #[test]
    fn test_git_new() {
        let badrmt = Git::new(
            "https://unknown/url/to/nonexistant/git/repo.git",
            Path::new("./src"),
            Path::new("./src")
        );
        assert!(matches!(badrmt, Err(VcsError::ShellCmdError { .. })));

        let no_gitdir = Git::new(
            "https://github.com/awkless-dotfiles/adm.git",
            Path::new("./bad"),
            Path::new("./src"),
        );
        assert!(matches!(no_gitdir, Err(VcsError::BadPathError { .. })));

        let no_workdir = Git::new(
            "https://github.com/awkless-dotfiles/adm.git",
            Path::new("./src"),
            Path::new("./bad"),
        );
        assert!(matches!(no_workdir, Err(VcsError::BadPathError { .. })));
    }
}
