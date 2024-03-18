// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use adm::error::VcsError;
use adm::vcs::Git;
use std::path::Path;

/// Test `Git::new()` method.
///
/// # Expectations
///
/// 1. Get VcsError::ShellCmdError for invalid remote URL.
/// 2. Get VcsError::BadPathError for invalid git directory path.
/// 3. Get VcsError::BadPathError for invalid work tree directory path.
/// 4. No errors for proper arguments to Git::new().
///
/// # Since
///
/// 0.2.0
#[test]
fn test_git_new() {
    let badrmt = Git::new(
        "https://unknown/url/to/nonexistant/git/repo.git",
        Path::new("./src"),
        Path::new("./src"),
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

    let good_git = Git::new(
        "https://github.com/awkless-dotfiles/adm.git",
        Path::new("./src"),
        Path::new("./src"),
    );
    assert!(matches!(good_git, Ok(Git { .. })));
}
