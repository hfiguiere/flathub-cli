Flathub CLI
===========

## Goals

A single command line tool to manage Flatpag packages for Flathub.

It should allow creating and submitting packages to Flathub, as well
as building and checking them.

### Tasks

- Create a package from scratch
- Build the package
- Add a dependency
  - Automatic shared-modules
  - Automatic dependencies from a know list
  - Custom dependency (interactive option)
  - Verify against the runtime
- Branch package
- Update runtime
- Regenerate dependencies with flatpak-builder-tools
- Convert manifest between JSON and YAML
- Configure the tool:
  - default user identity (for git commits)
  - github tokens
  - default manifest format
  - list of package you manage

### Interactive

Some commands can be used in an interactive mode with a TUI to help
guiding users.

## Features

Features are organised into subcommands.

### Configuration

There should be a way to have an automatic configuration.
- default runtime versions
- know packages list
- linter exceptions

The settings are stored in the user home directory in a standard XDG
location.

The first time the tool is launched it check for them and as to update
them.

Periodically it might alert the user that they are stale.

### Init

The init command initialize the directory to host a new flathub
package.

It will create a file `flathub-project.toml` and `git init`.

Optionally it can help creating a new manifest.

It can only be run once for a specified directory. And should check
based on git repository root for the presence of the config in parent
directories.

### Clone

Clone a flathub repository by app-id.

### Create manifest

Will assist the user in creating a manifest. Something like
`flatpak-manifest-generator`.

## Implementation

### Git

Supporting Git is necessary to manipulate the repository.

There is also probably a need to support Github for some commands like
submission or Pull Requests.

### JSON

Flatpak use a JSON parser that support non standard features like
C-style comments. Some tooling reject these.

### External commands

Shall we run external commands or shall we try to integrate python in
the Rust code.

## Example

```shell
$ flathub-cli clone org.gnome.Devhelp
...
$ cd org.gnome.Devhelp
$ flathub-cli build
```
