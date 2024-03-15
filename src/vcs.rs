// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::error::VcsError;
use std::path::{Path, PathBuf};
use std::process::Command;
use log::debug;

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
/// * `cmd`[in] - Name of shell command to execute.
/// * `args`[in] - Arguments to pass to shell command.
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

        let mut badutf8 = execute("sh", &["-c", "printf \"\\x0\\x9F\" >&2; exit 1"]);
        assert!(matches!(badutf8, Err(VcsError::ShellCmdUtf8Error { .. })));

        badutf8 = execute("sh", &["-c", "printf \"\\x0\\x9F\"; exit 0"]);
        assert!(matches!(badutf8, Err(VcsError::ShellCmdUtf8Error { .. })));

        let cmdfail = execute("ls", &["--bad-option"]);
        assert!(matches!(cmdfail, Err(VcsError::ShellCmdError { .. })));

        let goodcmd = execute("echo", &["hello world"]);
        assert_eq!("hello world\n", goodcmd.unwrap());
    }
}
