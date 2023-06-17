# refer: a simple reference manager

`refer` is a really simple bibliographic database manager. It doesn't use an sqlite database, but just a text file in a hidden file in your home directory.

References can be added, edited, deleted, or styled (limited styling at the moment).

# Install

Currently only available here on GitHub.

```bash
# do the usual stuff.
git clone https://github.com/Euphrasiologist/refer
cd refer
cargo install --path=.
```

# CLI API

Simple API, no helps currently within subcommands.

Run `rc -h` (5 chars!) to get this help.

```
rc 0.1.0
Max Brown <euphrasiamax@gmail.com>
https://github.com/euphrasiologist/refer

USAGE:
    rc [-h] [subcommand] [options]

    rc add [-jbe -s <string>] - add an entry to the database
                              - [-j] flag. is a journal
                              - [-b] flag. is a book
                              - [-e] flag. use an editor to add an entry
                              - [-s] option. provide a string as an arg
    rc remove <keywords>      - remove an entry from the database
    rc edit [-a <keywords>]   - edit an entry in the database
                              - [-a] flag. select from all entries
    rc status                 - some stats on the database
    rc setup                  - initialise an empty database. Should 
                                only be run once upon installing.
```