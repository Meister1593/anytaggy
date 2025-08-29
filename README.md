# Anytaggy

## Project is in heavy development, expect major breaking changes!

File tagger. Allows placing arbitrary tags on files and store it database, without modifying files.

Project is heavily inspired by [TMSU](https://github.com/oniony/TMSU), but written in rust from scratch and with a bit smaller scope.

Initial idea was to have a portable database of file tags for media, but this cat work on any kind of files (as codebase for media or for random files will be literally the same) and i see no reason to restrict app to just media.

## Roadmap
- [x] Basic functionality - Implemented (create, delete, tag, find files)
- [ ] Repair (move, rename files and try to restore them in database)
- [ ] Tag queries (combine AND/OR/NOT in one query)
- [ ] Generic data on files (arbitrary data linking to files, retrieving)

## Tests
Project aims to have as much of test coverage as possible

For testing coverage:
1. Install nextest: `cargo install cargo-nextest --locked`
2. Install llvm-cov: `cargo +stable install cargo-llvm-cov --locked`
3. Run nextest:
    * For code `Coverage Gutters`: `cargo llvm-cov nextest --lcov --output-path ./target/lcov.info`
    * CLI: `Coverage Gutters`: `cargo llvm-cov nextest`