# Name

flathub-cli - HHelp maintaining flathub packages on Flathub

# Synopsis

flatpak-cli init [-e] [-i ID] DIRECTORY

flatpak-cli cleanup [-n] [-v] COMMAND

# Description

## Options

## Commands

### Cleanup

flatpak-cli cleanup [-n] [-v] COMMAND

Run cleanup in the current project.

\-n, --dry-run: Show what will be done.

\-v, --verbose: More vervose output.

COMMAND:

- downloads: cleanup the downloads that are no longer need based on
the manifest.


### Clone

### Init

flatpak-cli init [-e] [-i ID] DIRECTORY

Initialise the project in DIRECTORY.

\-i ID, --id ID: the id of the project. Otherwise it is guessed based
on the directory name.

\-e, --existing: Required if there is already a git repository
setup. It will guess the manifest name based on the id.

### Manifest

# Examples

# See also

flatpak(1), flatpak-builder(1), flatpak-manifest(1)
