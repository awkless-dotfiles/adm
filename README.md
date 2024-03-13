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

[durdn-article]: https://www.atlassian.com/git/tutorials/dotfiles
[sparse-checkout]: https://git-scm.com/docs/git-sparse-checkout
