<a id="readme-top"></a>

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]

<br />
<div align="center">

<h3 align="center">chipATE</h3>

  <p align="center">
    A CHIP-8 emulator that runs in your terminal
    <br />
    <br />
    <a href="https://github.com/CK-7vn/chipATE/issues/new?labels=bug">Report Bug</a>
    &middot;
    <a href="https://github.com/CK-7vn/chipATE/issues/new?labels=enhancement">Request Feature</a>
  </p>
</div>

<details>
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#about-the-project">About The Project</a></li>
    <li><a href="#getting-started">Getting Started</a></li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#controls">Controls</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

## About The Project

![chipATE Demo](chipAte.type.gif)

chipATE is a CHIP-8 emulator built in Rust that renders directly in your terminal using a TUI (Text User Interface). Play classic CHIP-8 games like Pong, Tetris, Space Invaders, and Tic-Tac-Toe without leaving the command line.

### Built With

[![Rust][Rust-badge]][Rust-url]
[![Ratatui][Ratatui-badge]][Ratatui-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Getting Started

### Prerequisites

- Rust toolchain (1.70+)
- A terminal with unicode support

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/CK-7vn/chipATE.git
   cd chipATE
   ```
2. Build the project
   ```sh
   cargo build --release
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Usage

Run a ROM:
```sh
cargo run --release roms/tictac.ch8
```

Optionally specify cycles per frame (default: 12):
```sh
cargo run --release roms/pong.ch8 20
```

### Included ROMs

| ROM | Description |
|-----|-------------|
| `tictac.ch8` | Tic-Tac-Toe |
| `pong.ch8` | Classic Pong |
| `tetris.ch8` | Tetris |
| `invaders.ch8` | Space Invaders |
| `br8kout.ch8` | Breakout |
| `maze.ch8` | Random maze generator |

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Controls

CHIP-8 uses a 16-key hexadecimal keypad. The mapping is:

```
CHIP-8 Keypad        Keyboard
┌───┬───┬───┬───┐    ┌───┬───┬───┬───┐
│ 1 │ 2 │ 3 │ C │    │ 1 │ 2 │ 3 │ 4 │
├───┼───┼───┼───┤    ├───┼───┼───┼───┤
│ 4 │ 5 │ 6 │ D │    │ Q │ W │ E │ R │
├───┼───┼───┼───┤    ├───┼───┼───┼───┤
│ 7 │ 8 │ 9 │ E │    │ A │ S │ D │ F │
├───┼───┼───┼───┤    ├───┼───┼───┼───┤
│ A │ 0 │ B │ F │    │ Z │ X │ C │ V │
└───┴───┴───┴───┘    └───┴───┴───┴───┘
```

Press `Esc` to quit.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Roadmap

- [x] Core CHIP-8 instruction set
- [x] TUI display with ratatui
- [x] Keyboard input
- [x] Sound timer (terminal beep)
- [ ] SUPER-CHIP support
- [ ] Configurable color themes
- [ ] Save states

See the [open issues](https://github.com/CK-7vn/chipATE/issues) for known issues and feature requests.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Contributing

Contributions are welcome! Fork the repo and submit a pull request.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Acknowledgments

* [Cowgod's CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
* [Ratatui](https://github.com/ratatui-org/ratatui)
* [CHIP-8 Test ROMs](https://github.com/Timendus/chip8-test-suite)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

[contributors-shield]: https://img.shields.io/github/contributors/CK-7vn/chipATE.svg?style=for-the-badge
[contributors-url]: https://github.com/CK-7vn/chipATE/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/CK-7vn/chipATE.svg?style=for-the-badge
[forks-url]: https://github.com/CK-7vn/chipATE/network/members
[stars-shield]: https://img.shields.io/github/stars/CK-7vn/chipATE.svg?style=for-the-badge
[stars-url]: https://github.com/CK-7vn/chipATE/stargazers
[issues-shield]: https://img.shields.io/github/issues/CK-7vn/chipATE.svg?style=for-the-badge
[issues-url]: https://github.com/CK-7vn/chipATE/issues
[Rust-badge]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/
[Ratatui-badge]: https://img.shields.io/badge/Ratatui-000000?style=for-the-badge
[Ratatui-url]: https://github.com/ratatui-org/ratatui
