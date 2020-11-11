[![main](https://github.com/oltoko/pixelflut.rs/workflows/main/badge.svg)](https://github.com/oltoko/pixelflut.rs/actions?query=workflow%3Amain)

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
