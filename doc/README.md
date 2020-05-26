# Various help


## nightly features

Install toolchain (with nightly)

``` console
$ rustup target add thumbv6m-none-eabi --toolchain nightly
```
this installs also the respective core module

List toolchains (including nightly's)

``` console
$ rustup target list --toolchain nightly
```


References:
* https://github.com/rust-lang/rust/issues/63519