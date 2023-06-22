# refer: a simple reference manager

`refer` is a really simple bibliographic database manager. It doesn't use an sqlite database, but just a text file in a hidden file in your home directory.

References can be added, edited, deleted, or styled (limited styling at the moment).

Not production quality or anything, just a side project.

# Install

Currently only available here on GitHub. Might put on crates.io soon.

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
rc 1.0.0
Max Brown <euphrasiamax@gmail.com>
https://github.com/euphrasiologist/refer

USAGE:
    rc [-h] [subcommand] [options]

    rc add [-jbe -s <string>] - add an entry to the database
                              - [-j] flag. is a journal
                              - [-b] flag. is a book
                              - [-e] flag. use an editor to add an entry
                              - [-s] option. provide a string as an arg
    rc edit [-a <keywords>]   - edit/remove an entry in the database
                              - [-a] flag. select from all entries
    rc status                 - some stats on the database. Mainly for
                                debugging.
    rc setup                  - initialise an empty database. Should 
                                only be run once upon installing.
```