<!--
SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
SPDX-License-Identifier: MIT
-->

# Awklesses' Dotfile Manager

## Introduction

The `adm` project is a simple dotfile manager application written in Rust. It
uses Git to manage dotfiles by treating `$HOME` as the working tree (or any
other local directory of choice), and storing any changes in a local bare
repository in a side directory (typically in `$HOME/.local/share/dotfiles`).
Accessing and managing your version-controlled dotfiles only requires you to
pass Git commands directly to `adm` itself.

## Background

The `adm` project is an evolution of a simple shell script named `dm`. The `dm`
script took inspiration from Nicola Paolucci's article about [managing dotfiles
using a bare Git repository][durdn-article]. The `dm` script allowed you to
selectively deploy or undeploy groups of dotfiles via Git's [sparse
checkout][sparse-checkout] feature. However, the poorly written structure of the
`dm` script made it very difficult and time-consuming to add more deployable
dotfile configurations.  Thus, `adm` exists as a replacement for the `dm`
script. Plus, `adm` acts as my first-ever Rust coding project!

## Installation

You will need the following pieces of software:

1. Git [>= 2.20.0].
1. Rust [>= 1.56.0].

Clone this repository and use cargo like so:

```
# git clone https://github.com/awkless-dotfiles/adm.git
# cd adm
# cargo build
# cargo install
```

Enjoy!

## Usage

The `adm` command functions through a special configuration TOML file. This TOML
file can be stored in your `$HOME` either as `.admrc.toml` or
`.config/adm/config.toml`.  Inside this TOML file, you need to configure your
repository with the following format:

```
[setup]
    remote = "https://url/to/dotfiles"
    git_dir = "$HOME/.local/share/dotfiles"
    work_tree = "$HOME"
    backup = "$HOME/.cache/dotfiles"
```

Once complete, you can now use `adm` by passing Git commands to it. You can
change the values above to whatever you like. In general, `adm` allows you to
treat your `$HOME` (or any other location specified by `work_tree`) as a Git
repository.

After you have placed several dotfiles under version control with `adm`, you can
group them as a collection. This collection can then be physically removed or
added into your machine from your target working tree using `adm` deployment
commands. Whatever collection of dotfiles gets deployed or undeployed will
remain in your target working tree's history. This allows you to selectively
pick what dotfiles you want present on your machine at any given time, while
still keeping them version controlled.

To define a dotfile collection, add the following to the `adm` TOML
configuration file:

```
[[dotfiles]]
    name = "vim"
    description = "Configuration for VIM editor"
    files = ".vim/*"
```

Now you can use `adm deploy vim` or `adm undeploy vim` to deploy or undeploy a
dotfile collection named `vim` defined in your `adm` TOML configuration.

If you need to include an external repository into your configuration, then add
the following format to your `adm` TOML configuration:

```
[[repository]]
    name = "st"
    description = "Patched ST"
    remote = "https://github.com/awkless/st.git
    path = "$HOME/.local/share/"
```

Now use the `adm` deployment commands again to deploy or undeploy the `st`
external repository to or from your machine.

If you use the Git `checkout` command, then any file that conflicts with this
command will be automatically backed up to whatever location you specified in
`setup.backup` of your `adm` TOML configuration. No file will be overwritten.
Thus, you can still have access to untracked dotfiles on your working tree.

If you already have a dotfile repository that uses `adm`, then you can transfer
your configurations to a new computer with the following command:

```
# adm install "https://url/to/adm/dotfiles"
```

See the provided man page for more information about using the various
features `adm` offers, including additional information about writing your `adm`
TOML configuration file.

[durdn-article]: https://www.atlassian.com/git/tutorials/dotfiles
[sparse-checkout]: https://git-scm.com/docs/git-sparse-checkout
[contributing]: CONTRIBUTING.md
