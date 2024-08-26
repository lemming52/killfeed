# Killfeed

Attempting to build a lightweight CLI Worklog tool.

## Usage

### Installation

Currently only installation supported is by local cargo install:
```
cargo install --path .
```

The tool defaults to storing the notes in a file in the home directory, this can be overridden by setting `$KILLFEED_FILE` in your environment.
With default usage, the tool will open the system default editor configured by the `$EDITOR` envvar. If that is unset, it will default to a system
appropriate editor (i.e. on OSX, `pico`).

### Usage

#### Add a note

To add a note to the log, using the tool without arguments to open an editor and record a message:

```
$ kf
```

The `-m` flag can be used to skip the use of an editor.

```
$ kf -m "Headshot the bug!"
```

#### Reviewing

To view the recent worklog, use the head command:

```
$ kf head
```