# Rn

This is a command-line tool intented to be used with terminals. Rn stands for Rust notes and it is a simple tool for taking notes in terminal. This tool is really as simple as it gets with a few features.

## Example use

    rn ssh_notes "eval $(ssh-agent)"

    rn ssh_notes l
    0: My first note
    1: eval $(ssh-agent)

    rn r 0

    rn ssh_notes ö
    0: eval $(ssh-agent)

## Commands

Please replace filename with anything you want.

### Show help

    rn help
    // or
    rn h
    // or
    rn

---

### Add a new note

    rn filename "This is my note"
    // or
    rn filename add "This is my note"
    // or
    rn filename a "This is my note

---

### List all note files

    rn list
    // or
    rn l

---

### List all entries in a note file

    rn filename list
    // or
    rn filename l
    // or
    rn filename

---

### Remove entry in a note file

    // Removes note with id of 5
    rn filename remove 5
    // or
    rn filename r 5

---

### Edit note inside a note file

    rn filename edit 5 "This is my new text for note with id of 5"
    // or
    rn filename e 5 "This is my new text for note with id of 5"

---

### Open note file in a text editor

    rn open filename
    // or
    rn o filename

---

### Remove note file

    rn remove filename
    // or
    rn r filename
