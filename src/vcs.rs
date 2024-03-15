// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use crate::error::VcsError;
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
/// * `name`[in] - Name of shell command to execute.
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
