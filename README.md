![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/olagem?link=https%3A%2F%2Fperso.eleves.ens-rennes.fr%2Fpeople%2Fbenjamin.voisin%2F)
![Licence](https://img.shields.io/aur/license/olagem)
![Crates.io Version](https://img.shields.io/crates/v/olagem)
![AUR Version](https://img.shields.io/aur/version/olagem)


# olagem

A simple and beautiful terminal based typing speed test.

## Features

- Beautiful TUI
- Proper Unicode support
- French and English out of the box
- Add any language you want
- See your progression with a nice chart !

## Installation

The package is available on the AUR and on [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html), so you can install it with
```shell
cargo install olagem
```

Or, if you are on Arch, (replace `yay` with your favorite AUR helper)
```shell
yay -S olagem
```

### Build from source

Install Cargo: [here](https://doc.rust-lang.org/cargo/getting-started/installation.html), and then just
```shell
git clone git@github.com:benjamin-voisin/olagem.git
cargo build -r
```

And the binary can be found in `target/release/olagem`.

## Configuration

The configurations files may be found in the `~/.config/olagem/` directory.

The default config file in `~/.config/olagem/config.toml`:
```toml
[defaults]
language = "english"
time = 60
```

Currently, olagem comes with only 2 languages: french and english. If you want to use another one, you need to add a word list into `~/.config/olagem/language`. For example, to add spanish, you would add create the file `~/.config/olagem/language/spanish`, and change your `~/.config/olagem/config.toml` to be
```toml
[defaults]
language = "spanish"
time = 60
```
