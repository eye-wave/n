# ~~ru~~n - Use the Right Script Runner

[![GitHub License](https://img.shields.io/github/license/ryanccn/nrr?style=flat-square&color=blue)](https://github.com/eye-wave/ru/blob/main/LICENSE)


**n** is a CLI tool inspired by [antfu-collective/ni](https://github.com/antfu-collective/ni), that automatically selects the correct script runner based on your project setup. 

## Usage
For every `task / target / script / command` you use, n will try to run it with whatever your project uses.

`n build` will execute the following commands, depending on your project setup:
```bash
$ n build

# npm run build
# yarn run build
# pnpm run build
# bun run build
# deno task build

# make build

# cargo run --package xtask -- build
# cargo build
```

> [!TIP]
>
> To speedup your workflow even more, you can chain as many `tasks / targets / scripts / commands` as you want.

`n f l` will execute the following commands, one after the other:
```bash
$ n f l

# npm run format
# npm run lint

# cargo fmt
# cargo clippy

# ...
```
It aslo comes with aliases for cargo since format and lint does not exist as subcommands in it.

You can also add arguments and flags directly to commands.
For example: 

`n a "vite postcss tailwindcss" -d test`
```bash
$ n add "vite postcss tailwindcss" --save-dev test

# npm add vite postcss tailwindcss --save-dev
# npm test

# yarn add vite postcss tailwindcss --save-dev
# yarn test

# ...
```

Every quoted string *(with at least one whitespace)* and flags until the next command, will be treated as subargs that will be executed with your command.
> [!WARNING]
>
> If you want to pass a single subargument, you need to quote it and also put a space at the start or the end.
> Example: `n add "express "` to install express.js with whatever package manager you use at the moment

## Installation

Using curl:
```bash
curl -fsSL https://raw.githubusercontent.com/eye-wave/n/refs/heads/main/install.sh | bash
```

or clone this repo and
```bash
git clone https://github.com/eye-wave/n.git
cd n/
./install.sh

# use with "--stable" or "-s"
# if you prefer building with stable release of rust
```

## Special thanks
[ryanccn/nrr](https://github.com/ryanccn/nrr)
[antfu-collective/ni](https://github.com/antfu-collective/ni)

## License

GPL 3.0
