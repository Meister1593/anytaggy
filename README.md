# Anytaggy

## Project is in heavy development, expect major breaking changes!

Project heavily inspired by [TMSU](https://github.com/oniony/TMSU), but written in rust and with a bit smaller scope.

Initial idea was to have a portable database of file tags for media, but this cat work on any kind of files (as codebase for media or for random files will be literally the same) and i see no reason to restrict app to just media.

Roadmap:
* CRUD (?)
 * Tags (?)
 * Files (x)
* Repair (x)

Tests:
1. Install nextest: `cargo install cargo-nextest --locked`
2. Install llvm-cov: `cargo +stable install cargo-llvm-cov --locked`
3. Run nextest:
    * For code `Coverage Gutters`: `cargo llvm-cov nextest --lcov --output-path ./target/lcov.info`
    * CLI: `Coverage Gutters`: `cargo llvm-cov`