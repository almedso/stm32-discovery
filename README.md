# Embedded playground with STM32L053 Discovery board

The board is characterized as:

- user button
- user led
- SPI connected e-paper.
- STM32L0 (Cortex-M0 MCU)

## Project structure

* one project - one specific hardware
* applications - are in the example folder (one file per example)
* board specific source is organized in lib.rs and below

## Environment Preparation

see .cargo/config

``` toml
[build]
# Compilation target
target = "thumbv6m-none-eabi"    # Cortex-M0 and Cortex-M0+
```

### Linker and memory

Memory region information is in the `memory.x` file.

``` console
$ cat memory.x
...
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 40K
}
...
```

### Target hardware

Add the cortex m hardware targets via rustup.

``` console
$ rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```

## Examples and application

* panic: just panic but nothing else via semi-hosting


## Building


``` console
$ cargo build --example panic
$ cargo run --example led
```

## Flashing

check open ocd first after setting u-dev rules and connecting the board properly.

``` console
$ ./openocd_flash.sh target/thumbv6m-none-eabi/release/examples/hello
```

next let cargo take over ...

``` console
$ cargo run --example panic
```

This will use *openocd.cfg* and *openocd.gdb* configuration files.


## VS Code Intergration

This template includes launch configurations for debugging CortexM programs with Visual Studio Code located in the `.vscode/` directory.
See [.vscode/README.md](./.vscode/README.md) for more information.
If you're not using VS Code, you can safely delete the directory from the generated project.

# License

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

# Credits

This project has been generated by cortex-m-quickstart template maintained by
[team]: https://github.com/rust-embedded/wg#the-cortex-m-team
via

``` console
$ cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart
```
