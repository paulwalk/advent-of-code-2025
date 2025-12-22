# Advent-of-Code 2025

This is the result of my playing with *[Advent of Code 2025](https://adventofcode.com)*, an excellent annual set of puzzles for programmers, set by [Eric Wastl](https://was.tl) (who has been doing this for 10 years!).

Because Eric creates these puzzles to be manageable in any reasonable programming language, I have used them over the years to teach myself new languages and techniques. This year, for me, it was the turn of Rust.

There are 12 days of puzzles. Most days have two parts. This year I got as far as Day 8 part 2. I coded a basic brute-force "solution" for part 2: this works for the small test data-set but it doesn't scale to the main puzzle data-set. I think that I probably lack the maths to find a real solution to this that scales to deal with the main data-set. In any case, I can't commit any more time to this so I'm going to leave it there.

I learned a lot about Rust from this which was the main point for me, and I had fun doing it :-)

Each day's puzzle has a Rust module under `src`, and a puzzle directory under `puzzles`. There are small test-data sets and (usually large) full data-sets for each day. The Rust modules for each day contain tests for both test data-sets and full data-sets. The answers for the full data-sets (the ones I have solved) are read in from a `.env` file. If you want to run the tests for the full data-sets, you will need to create your own `.env` file with the appropriate answers (see `.env.template` for the format).