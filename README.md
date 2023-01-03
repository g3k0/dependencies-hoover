# Dependencies Hoover

A superfast Node.js projects dependencies cleaner.

## Overview

Dependencies Hoover scans a Node.js project (both js and ts), searches recursively for package.json files and  if a dependency (included devDependencies) is not imported in a source file or is not present inside the node_modules/.bin folder, removes the dependency from the
package.json.<br/> 
A final report file is produced after the scan.<br/>
The application can run in cleaning mode or analysis only mode, in this case only the report is produced and the dependencies are not removed.

Dependencies Hoover is entirely written in [Rust](https://www.rust-lang.org/) and it doesn't have external dependencies, it means that the executable file builded on a local machine runs independently of the operating system.<br />
The executable reads at runtime from a configuration file, so it is possible to change the configuration with no need to compile the source code again.

## Configuration

The configuration is contained in the **config.toml** file, it has 4 parameters:

* project_to_scan_path: is the path (full of relative) of the Node.js project to scan;
* analysis_only: is a boolean flag that permits to perform only the code analysis without the dependencies being removed;
* ignore_dirs: is the list of directories that Dependencies Hoover will ignore;
* dependencies_whitelist: is a list of dependencies that are ignored by Depencies Hoover, wrote as regex strings;

## Build

In order to build the application, rust and cargo must be installed on the local machine.
Once installed, go to the root directory of this project and run the command:

```bash
cargo build
```

if you want to build a release version run:

```bash
cargo build --release
```

The release version of the executble is optimized, but it will take longer to compile.<br />
The executable will be produced under the **target** folder.

## How to run

Before to run the application edit the **config.toml** file depending on your needs.<br />

If you want to run the application in development mode, it is possible to install the dependencies, build and run the application at the same time with the following command:

```bash
cargo run
```

If you want to run the executable, be sure that the config.toml file is present and the executable has the needed rights (read ,write and execution), then just run from a shell:

```bash
./dependencies_hoover
```

If the execution is correct, a report is written in a reports folder.

## Test

In order to run tests type the following command:

```bash
cargo test
```

## Code Coverage Report

It is possible to generate a code coverage report if the following prerequisites are satisfied:

* [rust nightly version must be installed](https://www.geeksforgeeks.org/how-to-install-rust-nightly-on-macos/);
* [grcov must be installed](https://github.com/mozilla/grcov);

Then, follow these steps:

1. build the application with the **cargo build** command;
2. execute the tests with the **cargo test** command;
3. generate the report with the following command:

```bash
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
```

The HTML report is generated under the following path:

```bash
target/debug/coverage
```

At the time of writing, the application has about the 75% of code coverage.

## Notes

This application is developed and tested with the stable release of Rust:

```bash
rustc 1.65.0 (897e37553 2022-11-02)
```
