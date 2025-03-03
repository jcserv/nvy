# nvy 😡

![visitors](https://img.shields.io/endpoint?url=https://vu-mi.com/api/v1/views?id=jcserv/nv) ![downloads](https://img.shields.io/crates/d/nvy)

nvy (pronounced "ehn-vee", like the word "envy") is a simple command line tool for managing multiple env files (profiles) in a project.

- supports exporting to a target file
- supports exporting to a shell
  - note: child processes cannot modify the environment of the parent process, so the `use` command outputs the command text which can be eval'd to set the environment variables, when in shell mode.

## installation 📦

### homebrew
`brew tap jcserv/cask`

`brew install nvy`

### cargo

`cargo binstall nvy` ([cargo-binstall](https://github.com/cargo-bins/cargo-binstall?tab=readme-ov-file#installation))

or

`cargo install nvy`

## usage ⚙️ 

1. `nvy init` - to create a new nvy.yaml file in the current working directory
2. switching profiles
   - shell mode:
     -  `eval "$(nvy use <profile>)"` - to switch between profiles
     - You can add an alias to your shell config to make this easier: `alias nv='eval "$(nvy use $1)"'` 
   - file mode:
     - `nvy target <target-file>` to set the target file to write to
     - `nvy use <profile>` to switch between profiles

note: you can also use `nvy use <profile1> <profile2> ...` to use multiple profiles

## why 🤔

have you ever had multiple configurations with different environment variables that you had to switch between?

rather than tweaking the file by hand, or commenting out code, you can now: 
1. define separate profiles (a .env* file for each)
2. use `nvy` to easily switch between

also, this was an opportunity for me to learn Rust by doing.

## references 📚
- [Command Line Applications in Rust](https://rust-cli.github.io/book/index.html)
- [How to Deploy Rust Binaries with GitHub Actions](https://dzfrias.dev/blog/deploy-rust-cross-platform-github-actions/)
- [How to Publish your Rust project on Homebrew](https://federicoterzi.com/blog/how-to-publish-your-rust-project-on-homebrew/)
