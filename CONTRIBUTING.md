<!--
SPDX-FileCopyrightText: 2024 Jason Pena <jasonpena@awkless.com>
SPDX-License-Identifier: MIT
-->

# Contributing

Thanks for taking the time to contribute! Remember, the information in this
documentation are just _guidelines_. Always use your best judgement!

## Expected Forms of Contribution

The `adm` project is open to the following forms of contribution:

1. Improvements or additions to production code.
1. Improvements or additions to test code.
1. Improvements or additions to build system.
1. Improvements or additions to documentation.
1. Improvements or additions to CI/CD pipelines.

Use the provided templates for bug reports, feature requests, and pull requests.
Please only use the bug tracker for reporting bug reports, or feature request
submissions.

Pull request submissions must occur on a separate branch, and compared by the
current state of the `main` branch. Keep your commit history linear. Linear
commit history makes it easier to perform rebase merging. This project does not
like merge commits.

## Coding Style

Use `rustfmt` to format your code. Here is some additional conventions that
`rustfmt` cannot cover:

- Maximum line width is 80 characters.
- Use `//` for normal comments.
    - Describe the _why_, not the _how_ for code.
- Use `///` for documentation comments.
    - Include the preconditions and postconditions of any function you add.
- Special tagged comments you can use are `// TODO:` and `// FIXME:`.
- Do not use out parameters in any function you create.
