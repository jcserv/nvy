# nv

nv (pronounced "ehn-vee", like the word "envy") is a simple command line tool for managing multiple env files (profiles) in a project.

## usage

`nv init` - to create a new nv.yaml file

`nv use <profile>` - to switch between profiles

nv.yaml
```yaml
version: 0.1.0
profiles:
  default:
    - path: .env # or ./.env 
  local:
    - path: .env.local
  prod:
    - path: .env.prod
  devspace:
    - path: .env.devspace
```

References:
- [Command Line Applications in Rust](https://rust-cli.github.io/book/index.html)