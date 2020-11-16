[![Crates.io](https://img.shields.io/crates/v/pixelflut-rs?label=%F0%9F%93%A6%20crates-io&style=flat-square)](https://crates.io/crates/pixelflut-rs/)
[![License](https://img.shields.io/github/license/oltoko/pixelflut.rs?color=informational&label=%F0%9F%93%84%20license&style=flat-square)](https://github.com/oltoko/pixelflut.rs/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/oltoko/pixelflut.rs/main?style=flat-square&label=%F0%9F%94%A7%20build)](https://github.com/oltoko/pixelflut.rs/actions?query=workflow%3Amain)

# pixelflut.rs
A library providing a Pixelflut server to easily connect your display or whatever to it. 

## What is Pixelflut?

Pixelflut uses a very simple (and inefficient) ASCII based network protocol. You can write a basic client in a single line of shell code if you want, but you only get to change a single pixel at a time. If you want to get rectangles, lines, text or images on the screen you have to implement that functionality yourself. That is part of the game.

## Pixelflut Protocol

* `HELP`: Returns the available commands.
* `SIZE`: Returns the size of the visible canvas in pixel as `SIZE <w> <h>`.
* `PX <x> <y>`: Return the current color of a pixel as `PX <x> <y> <rrggbb(aa)>`.
* `PX <x> <y> <rrggbb(aa)>`: Draw a single pixel at position (x, y) with the specified hex color code. If the color code contains an alpha channel value, it is blended with the current color of the pixel.

You can send multiple commands over the same connection by terminating each command with a single newline character (`\n`).

## Example

To get a better understanding on how this library should be used, please take a look at the [really simple example](https://github.com/oltoko/pixelflut.rs/blob/main/examples/vec_grid.rs) (**Warning** 😱 no fancy bling bling 😢).
