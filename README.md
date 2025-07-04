<p align="center">
<a href="https://rencore.github.io"><img src="https://raw.githubusercontent.com/rencore/rencore.github.io/master/docs/images/logo_color.png" height="100px"/></a>
 </p>
<h1 align="center">rencore</h1>
<div align="center">
 <strong>
    A simple framework to build Rust CLI Applications
 </strong>
</div>
<br/>

[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bartekus/rencore/blob/master/LICENSE) [![Gitter](https://badges.gitter.im/rencore/community.svg)](https://gitter.im/rencore/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge)
[![Tests](https://github.com/rencore/rencore/actions/workflows/tests.yml/badge.svg)](https://github.com/rencore/rencore/actions/workflows/tests.yml)
[![Build](https://github.com/rencore/rencore/actions/workflows/build.yml/badge.svg)](https://github.com/rencore/rencore/actions/workflows/build.yml)
[![codecov](https://codecov.io/gh/rencore/rencore/branch/master/graph/badge.svg)](https://codecov.io/gh/rencore/rencore)

[Website](https://rencore.github.io)

`rencore` is a starter boilerplate to create a Rust CLI application. It comes with a battery of components like argument parsing and configuration. It also has different tooling to create your binary, or automate your build process.

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**

- [What's New?](#whats-new)
    - [Version 2.0.0](#version-200)
- [About](#about)
- [FAQ](#faq)
- [Features](#features)
- [Quick Bootstrapping](#quick-bootstrapping)
- [How to Contribute](#how-to-contribute)
  - [Versioning](#versioning)
- [License](#license)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## What's New?

#### Version 2.0.0

What's new in Version 2.0?

- [x] Upgrade to Clap 4.x
- [x] Shell Completion
- [x] Merge AppConfig with Clap/Cli arguments
- [x] use ConfigBuilder instead of Config
- [x] rustfmt update
- [x] Github release action

Planned for a 2.x release

- [x] Journald/Syslog as features
- [x] Fix CI/CD actions and Docker image

## About

`rencore` is an empty Rust CLI application with libraries, and few defaults. The goal is to help you bootstrap your next CLI project as quickly as possible while ensuring you make use of the best tools and best-practices that are available today.

There is no configuration required (though we recommend you check all the possible configurations possible). An empty clone will compile, and has a few sample commands. You can start coding right away!

## FAQ

For the Full FAQ, check the [website](https://rencore.github.io/#faq)

## Features

- [Clap](https://github.com/clap-rs/clap) for Command Line Argument parsing.
- Error Chaining with [Failure](https://github.com/rust-lang-nursery/failure).
- Configuration management with [config-rs](https://github.com/mehcode/config-rs).
- Multi-Drain, async Logging with [slog](https://github.com/slog-rs/slog).
- Static binaries with [rust-musl-builder](https://github.com/emk/rust-musl-builder).
- CI/CD through Github actions.
- Code Coverage, Justfile, etc..
- MIT License.

## Quick Bootstrapping

`rencore` should compile and run as is. You just need to clone the repository. A `cargo-generate` template is also [available](https://github.com/rencore/rencore-generate). For a more detailed introduction, check the [Getting Started](https://rencore.github.io/#getting-started) guide.

## How to Contribute

Details on how to contribute can be found in the [CONTRIBUTING.md](.github/CONTRIBUTING.md) file.

### Versioning

Rust Starter stricltly adheres to the [SemVer](https://semver.org/) Semantic Versioning.

## License

`rencore` is licensed under the MIT license. Please read the [LICENSE](LICENSE) file in this repository for more information.
