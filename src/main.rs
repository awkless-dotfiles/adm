// SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
// SPDX-License-Identifier: MIT

use adm::vcs::Git;
use env_logger::Builder;
use log::{error, info, LevelFilter};
use std::path::Path;

/// Start of execution.
///
/// # Preconditions
///
/// None.
///
/// # Postconditions
///
/// None.
///
/// # Since
///
/// 0.1.0
fn main() {
    Builder::new().format_timestamp(None).filter_level(LevelFilter::max()).init();

    let remote = "git@github.com:awkless-dotfiles/adm.git";
    let git_dir = Path::new("./.git/");
    let work_dir = Path::new("./");
    let gitcmd = match Git::new(remote, &git_dir, &work_dir) {
        Ok(git) => git,
        Err(error) => {
            error!("{}", error);
            return;
        }
    };

    let gitout = match gitcmd.execute(&["status"]) {
        Ok(out) => out,
        Err(error) => {
            error!("{}", error);
            return;
        }
    };

    info!("{}", gitout);
    return;
}
