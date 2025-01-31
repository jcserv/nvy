# nv

nv (pronounced "ehn-vee", like the word "envy") is a simple command line tool for managing multiple env files (profiles) in a project.

child processes cannot modify the environment of the parent process, so the `use` command outputs the command text which can be eval'd to set the environment variables.

## usage

1. `cargo install nv`
2. `nv init` - to create a new nv.yaml file
3. `chmod +x nv.sh`
4. `eval "$(./target/debug/nv use <profile>)"` - to switch between profiles

```yaml
profiles:
  default:
    - path: .env
  local:
    - path: .env.local
  prod:
    - path: .env.prod
  devspace:
    - path: .env.devspace
```

References:
- [Command Line Applications in Rust](https://rust-cli.github.io/book/index.html)