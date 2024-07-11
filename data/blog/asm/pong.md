[metadata]
title = "Build a Pong in x86 NASM assembly, without libc"
category = "asm"
tags = [ "asm", "game", "tutorial", "linux" ]
date = 1720700912
description = """
  Create a simple multiplayer Pong game in assembly, using only linux syscalls
  and ANSI terminal manipulation.
"""

---

Assembly is fascinating, but seams really complicated. In order to learn it in a funnier
way, let's create a game of **Pong** with it !

We'll learn a bit of Linux *syscalls* along the way, and how TUI interfaces are made using
[ANSI](wiki-ansi-code) codes

TODO Disclamer crappy game

# Hello world !

## Base setup

TODO Compilation text

talk about gdb to debug

## Assembly code

### Commands

TODO Commands text

link to opcode browser

### Constants

TODO Constants text

How storage works for different types of variables

### Functions

TODO Assembly functions text

### Include file

TODO Care of circular includes

### Kernel syscall

TODO Syscalls text

TODO Create the print function

# Get user inputs

TODO Read stdin

TODO Configure stdin with IOCTL

TODO Loop that reads char, quit if hit "q"

# Update with a timer

TODO Create timer using linux syscalls

TODO Add the required configuration based on what expected in C code

TODO Code that calls a handler every 1 secs, and increase counter

# Create the game

## Create game mecanisms

TODO Create the game mecanisms

## Draw the screen

TODO Draw the screen

[wiki-ansi-code]: todo
