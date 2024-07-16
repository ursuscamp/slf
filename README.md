# slf

Slf stands for "simple log file". It's basic tool and log file format intended to be used as a stream-of-consciousness
log format for my own personal use in various projects.

## File format

Basic text file is `.slf` file extension. Each log message is one line. Each message begins with a timestamp in this format:
`YYYY-MM-DD HH:MM: `. The timestamp is always in UTC. Following the final `: ` is a log message. The log message can be anything.
You can also use tags in the log message. Tags can be formatted like this: `#lower-case-letters-with-dashes`. Tags can be useful
for filtering and searching.

The log messages are generally expected to be in newest-to-oldest order in the log file, for quickly reviewing the latest logs in a text editor or the
`head` command.

## Usage

The slf tool supports one command currently:

```bash
$ slf -f log.slf log 'message here #tag'
```

This will log `"message here #tag"` in the `log.slf` file. It will create the file if necessary.

The `-f log.slf` is optional. It will use `log.slf` in the working folder by default if a file isn't specified.

## Configuration

You can create a global config for `slf` by putting a file in your user configuration folder:

```toml
path = "/path/to/global/log"
```

The path will be used for all log messages unless it is overridden by the `-f` flag.

The user configuration folders are:

| OS      | Folder                       |
| ------- | ---------------------------- |
| Lin/Mac | /home/bob/.config/slc        |
| Win     | C:\Users\Bob\AppData\Roaming |
