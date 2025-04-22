# Contributing to `objc2`

Thank you for your interest in contributing to `objc2`! The project is
currently run mostly by me, [@madsmtm](https://github.com/madsmtm), but I am
developing this project for the Rust community, and I am very open to
contributions.

For small changes, just file a PR. For larger changes, please file an issue
first, such that we can discuss the implementation plan before you potentially
waste a bunch of work.

If you prefer to have a more synchronous and less "formal" discussion, we have
[a Matrix workspace](https://matrix.to/#/#objc2:matrix.org), feel free to
discuss implenentation details in the "Developers" room.


## Scope

The aim of the project is to map all of Apple's C and Objective-C frameworks,
in [a manner that is as nice to use as possible][nice-to-use].

Access to Swift-only frameworks or functionality is [out of scope][swift-oos]
because of complexity, and should be handled in another project (though I'm
open to adding functionality in `objc2` to facilitate that).

[nice-to-use]: https://github.com/madsmtm/objc2/issues/429
[swift-oos]: https://github.com/madsmtm/objc2/issues/524


## Windows users

This repository relies heavily on symlinks, so make sure that your system is
set up to enable those (see [this StackOverflow answer][so-symlink] for
details on how to do that).

[so-symlink]: https://stackoverflow.com/a/59761201/5203369


## Framework crates

`objc2` uses a custom tool called `header-translator` to generate the crates
in `framework-crates/*`. The plan is to eventually upstream this tooling into
[`bindgen`](https://github.com/rust-lang/rust-bindgen), though doing that will
be a lengthy process.

Head over to [the README](./crates/header-translator/README.md) for more
detailed instructions on how to use this tool and generate new bindings.


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

When making changes framework crates, you must update the `git` submodule
manually and push your changes to a fork of `objc2-generated` (you do not need
to submit a PR to that repo).

We try to maintain a linear history in that repo, in a fashion such that every
ref that is ever referenced from the `HEAD` branch in this repository, is
reachable from the `HEAD` branch that repository. This will probably mean
you'll have to rebase more often.

If you're confused about this, don't worry too much about it, if you enable
"Allow edits by maintainers", we can fix it for you.


## Licensing

When contributing, you agree to license your work under the trio
`Zlib OR Apache-2.0 OR MIT` license (regardless of what the crate's current
license is).

See also [the license document](./LICENSE.md).


## Release checklist

Copy and fill out the following checklist into the release PR:

```markdown
- [ ] The branch is named `ci-full`, such that the full CI will run.
- [ ] Changelogs have only been modified under the `Unreleased` header.
- [ ] Version numbers are bumped in the following order:
    - `objc2-proc-macros`
    - `objc2-exception-helper`
    - `objc2-encode`
    - `objc2`
    - `block2`
    - `dispatch2`
    - Framework crates
- [ ] Tests have been run on macOS 10.12.
- [ ] Tests have been run on macOS 10.12 32bit.

Post merge:
- [ ] Run `cargo release`.
- [ ] Create a tag on the merge commit for each new version.
```

### Changelogs and tags

The changelog is shared for the framework crates, and is public at
[this link][frameworks-changelog], the other crates have their own changelogs.

The framework crates share the tag prefix "frameworks", while the other crates
also have their own tag prefix.

[frameworks-changelog]: https://docs.rs/objc2/latest/objc2/topics/about_generated/changelog/index.html
