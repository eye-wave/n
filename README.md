# ru~~n~~ - Use the Right Script Runner

**ru** is a CLI tool inspired by [antfu-collective/ni](https://github.com/antfu-collective/ni), that automatically selects the correct script runner based on your project setup. 


## Usage

Instead of
```bash
$ cargo format
error: no such command: `format`

	Did you mean `fmt`?
```
you can just run
```bash
$ ru format
# or aliased
$ ru f
```

## Examples
for every `task / target / script / command` you use, ru will try to run it with whatever your project uses.

`ru build` will execute the following commands, depending on your project setup:
```bash
$ ru build

# npm run build
# yarn run build
# pnpm run build
# bun run build
# deno task build

# cargo run --package xtask -- build
# cargo build

# make build
# just build
```

to speedup your workflow even more, you can chain as many `tasks / targets / scripts / commands` as you want.

`ru f l` will execute the following commands, one after the other:
```bash
$ ru f l

# npm run format
# npm run lint

# yarn run format
# yarn run lint

# pnpm run format
# pnpm run lint

# bun run format
# bun run lint

# deno task format
# deno task lint


# cargo run --package xtask -- format
# cargo run --package xtask -- lint

# cargo fmt
# cargo clippy


# make format
# make lint

# just format
# just lint
```

## Installation

Clone this repository and run install.sh

```bash
$ git clone repo:url
$ ./install.sh

# use with "--stable" or "-s"
# if you prefer building with stable release of rust
```

## License

GPL 3.0
