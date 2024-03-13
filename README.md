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
