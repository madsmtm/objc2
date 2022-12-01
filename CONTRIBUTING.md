# Contributing to `objc2`

Thank you for your interest in contributing! There are many ways to contribute and we appreciate all of them.


## `git` submodules

The `icrate` crate contains a submodule pointing to
[`madsmtm/icrate-generated`]. This exists to keep the primary repository free
from the clutter inherently associated with storing generated files in `git`,
while still allowing change-tracking of said files.

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


[`madsmtm/icrate-generated`]: https://github.com/madsmtm/icrate-generated
[submodule-docs]: https://git-scm.com/docs/gitsubmodules
[`git-submodule`]: https://git-scm.com/docs/git-submodule
