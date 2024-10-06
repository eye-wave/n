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
For every `task / target / script / command` you use, ru will try to run it with whatever your project uses.

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

To speedup your workflow even more, you can chain as many `tasks / targets / scripts / commands` as you want.

`ru f l` will execute the following commands, one after the other:
```bash
$ ru f l

# npm run format
# npm run lint

# yarn run format
# yarn run lint

# ...
```

## Advanced use

You can also add arguments and flags directly to commands.

For example: `ru a "vite postcss tailwindcss" -d test`
```bash
$ ru add "vite postcss tailwindcss" --save-dev test

# npm add vite postcss tailwindcss --save-dev
# npm test

# yarn add vite postcss tailwindcss --save-dev
# yarn test

# ...
```

Every quoted string *(with at least one whitespace)* and flags until the next command, will be treated as subargs that will be executed with your command.

## Installation

Clone this repository and run install.sh

```bash
$ git clone repo:url
$ ./install.sh

# use with "--stable" or "-s"
# if you prefer building with stable release of rust
```

## Special thanks
[ryanccn/nrr](https://github.com/ryanccn/nrr)
[antfu-collective/ni](https://github.com/antfu-collective/ni)

## License

GPL 3.0
