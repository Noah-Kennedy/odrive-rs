# odrive-rs
[![Build Status](https://travis-ci.com/Noah-Kennedy/odrive-rs.svg?branch=master)](https://travis-ci.com/Noah-Kennedy/odrive-rs)
[![codecov](https://codecov.io/gh/Noah-Kennedy/odrive-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/Noah-Kennedy/odrive-rs)
[![Documentation](https://docs.rs/odrive-rs/badge.svg)](https://docs.rs/odrive-rs)
![crates.io](https://img.shields.io/crates/v/odrive-rs.svg)

A community library for control of ODrive motor controllers.
This library was based heavily on the ODrive python and Arduino libraries.

## Roadmap
- [x] ASCII protocol commands
- [x] ODrive property editing
- [ ] Documentation
    - [x]   ASCII Protocol commands 
    - [ ]   Configuration parameter documentation
- [ ] Read ODrive errors

## Examples
The examples directory has several examples. To run one, run
```bash
cargo run --example {Example} -- /dev/ttyACM0
```

## Contributing
If you have any features you would like added, or any bugs you wish to
report, please submit and issue on the GitHub repo.

If you would like to work on implementing a new feature or fixing a bug,
feel free to make a pull request.