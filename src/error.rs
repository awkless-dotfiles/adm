// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

/// Enumeration of all error types for `adm::vcs` module.
#[derive(thiserror::Error, Debug)]
pub enum VcsError {
    /// Shell command failed to execute for whatever reason.
    #[error("Command '{cmd}' failed to execute - {errsrc}")]
    ShellCmdSpawnError { cmd: String, errsrc: std::io::Error },
}
