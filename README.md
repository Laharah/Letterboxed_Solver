# Letter Boxed Solver

This is a simple solver for the [New York Times Letter Boxed](https://www.nytimes.com/puzzles/letter-boxed) puzzle. The puzzle consists of a 12-letter box shaped board, split into 3-letter sides. The goal is to create a complete cover of the given letters in as few words as possible.

You are restricted by the following rules:

- Consecutive letters must not be on the same side of the box.
- The first letter of a new word must be the same as the last letter of the previous word.
- Letters may be used multiple times.

## Purpose

This project was written as a way to practice my rust skills and solve a puzzle that I really suck at :)

## Details

The solver uses the [2of12](http://wordlist.aspell.net/12dicts-readme/) wordlist arranged into a trie, and uses an A* search for efficient solutions. It currently assumes a solution can always be found in 5 words or less, which has held true so far.
