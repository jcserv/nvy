# nvy 😡

![visitors](https://img.shields.io/endpoint?url=https://vu-mi.com/api/v1/views?id=jcserv/nv)

nvy (pronounced "ehn-vee", like the word "envy") is a simple command line tool for managing multiple env files (profiles) in a project.

child processes cannot modify the environment of the parent process, so the `use` command outputs the command text which can be eval'd to set the environment variables.

## usage ⚙️ 

1. `cargo install nvy`
2. `nvy init` - to create a new nvy.yaml file
3. `eval "$(nvy use <profile>)"` - to switch between profiles
- You can add an alias to your shell config to make this easier: `alias nv='eval "$(nv use $1)"'` 

## why 🤔

have you ever had multiple configurations with different environment variables that you had to switch between?
rather than tweaking the file by hand, or commenting out code, you can now: 
1. Define separate profiles (a .env* file for each)
2. Use `nvy` to easily switch between

also, this was an opportunity for me to learn Rust by doing.

## references 📚
- [Command Line Applications in Rust](https://rust-cli.github.io/book/index.html)