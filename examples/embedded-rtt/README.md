# canbls-embedded

- STM32F407

## build

```bash
$ cargo build --target thumbv7em-none-eabihf
```

> release

```bash
$ cargo build --release --target thumbv7em-none-eabihf
```

## openocd

```bash
$ ./openocd -f openocd.cfg -c init -c halt -c "flash write_image erase ./embedded-rtt 0x08000000" -c reset
```

> display debug info

```bash
$ nc localhost 19021
```