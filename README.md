# Rgit

This repo contains an implementation of a "Git-like version control system (VCS)".
It was made by following the step by step tutorial created by `nikita` (which is fantastic btw), as a learning exercise, first to understand Git internals and also to practice a bit with the Rust programing language.

You can find the original `μgit` tutorial here: https://www.leshenko.net/p/ugit/

The code on this repository was not written to be idiomatic, clear, and/or beautiful, the only concern was to follow the provided Python code as closely as possible.

**Note:** At the moment, the program is not yet finished. The last 3 steps related to the `add` (staging) feature are still missing.

**Note 2:** Do not use it for any meaningful work.

## Current commands

```
$ rgit --help
rgit vcs 0.1.0
Gonçalo Valério <gon@ovalerio.net>
A watered-down git clone

USAGE:
    rgit [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add            Add files to the index
    branch         Create a new branch
    cat-file       outputs the original object from the provided hash
    checkout       Move the current content and HEAD to given commit
    commit         writes a named snapshot of the current tree
    diff           Compare the working tree with the given commit
    fetch          Fetch refs and objects from another repository
    hash-object    created an hash for an object
    help           Prints this message or the help of the given subcommand(s)
    init           creates new repository
    k              visualize refs and commits
    log            List all commits
    merge          Merge changes of a different commit/branch
    merge-base     Find the common ancestor between two commits
    push           Push refs and objects to another repository
    read-tree      writes a given tree to the working directory
    reset          Move the current content and HEAD to given commit with dereferencing
    show           Show diff from a commit
    status         check current branch
    tag            Create a tag for a given commit
    write-tree     write the current working directory to the database
```
