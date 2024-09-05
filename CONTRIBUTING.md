# Contributing to `objc2`

Thank you for your interest in contributing! There are many ways to contribute and we appreciate all of them.


## A note for Windows users

This repository relies heavily on symlinks, so make sure that your system is
set up to enable those (see [this StackOverflow answer][so-symlink] for
details on how to do that).

[so-symlink]: https://stackoverflow.com/a/59761201/5203369


## Framework crates

`objc2` uses a custom tool called `header-translator` to generate the framework
crates. Head over to [the README](./crates/header-translator/README.md) for
more detailed instructions on how to use this tool, and generate new bindings.

Note that crates that use `CoreFoundation`-like functionality is likely to not
yet be translatable, see [#556](https://github.com/madsmtm/objc2/issues/556)
for progress on this front.


### `git` submodules

The framework crates uses a submodule pointing to [`madsmtm/objc2-generated`].
This exists to keep the primary repository free from the clutter inherently
associated with storing generated files in `git`, while still allowing
change-tracking of said files.

You should consider whether you want a shallow or a full checkout of the
generated files - the default is to do a shallow clone, since the repository
can become quite large over time.

```sh
# Shallow clone
git submodule update --init
```

```sh
# Full clone
git submodule update --init --no-recommend-shallow
```

See the excellent `git` [documentation on submodules][submodule-docs], or the
manpage for [`git-submodule`] for further details on how submodules work.


[`madsmtm/objc2-generated`]: https://github.com/madsmtm/objc2-generated
[submodule-docs]: https://git-scm.com/docs/gitsubmodules
[`git-submodule`]: https://git-scm.com/docs/git-submodule


### Updating the `git` submodule

When making changes to `header-translator`, you must update the `git`
submodule manually and push your changes to a fork of `objc2-generated` (you
do not need to submit a PR to that repo).

We try to maintain a linear history in that repo, in a fashion such that every
ref that is ever referenced from the `HEAD` branch in this repository, is
reachable from the `HEAD` branch that repository. This will probably mean
you'll have to rebase more often.

If you're confused about this, don't worry too much about it, if you enable
"Allow edits by maintainers" we can fix it for you.


## Release checklist

Copy and fill out the following checklist into the release PR:

```markdown
- [ ] The branch is named `new-versions`, such that the full CI will run.
- [ ] Changelogs have only been modified under the `Unreleased` header.
- [ ] Version numbers are bumped in the following order:
    - `objc2-proc-macros`
    - `objc2-exception-helper`
    - `objc2-encode`
    - `objc2`
    - `block2`
    - Framework crates
- Local tests have been run (see `helper-scripts/test-local.fish`):
    - [ ] macOS 10.14.6 32bit
    - [ ] iOS 9.3.6, 1st generation iPad Mini

Post merge:
- [ ] A tag is created on the merge commit for each new version.
```
