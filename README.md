# cmp_env

![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]

research: compare the accessing to environment variables

## Slower libc::getenv()

- rustc 1.56.1 (59eed8a2a 2021-11-01)

|         `name`          |   `bench`   |   `musl`    |
|:------------------------|------------:|------------:|
| cmp-env_cache           |   18.507 us |   52.180 us |
| cmp-libc_getenv         |  145.890 us |  596.490 us |
| cmp-std_env_var         |  154.550 us |  591.970 us |
| cmp-envmnt_get          |  165.920 us |  652.770 us |

- rustc 1.53.0 (53cb7b09b 2021-06-17)

|         `name`          |   `bench`   |   `musl`    |
|:------------------------|------------:|------------:|
| cmp-env_cache           |   19.044 us |   51.436 us |
| cmp-libc_getenv         |  146.800 us |  591.900 us |
| cmp-std_env_var         |  157.970 us |  589.630 us |
| cmp-envmnt_get          |  167.400 us |  652.310 us |

- `us` is micro seconds, lower is better
- bench on intel Q6600 @ 2.40GHz

- [envmnt](https://crates.io/crates/envmnt) - is the many access-type to environment variables.
- env_cache - is std::env::var() + HashMap<>, my simple implements.

## This benchmark measures getenv(key).

It is based on `std::env::var()`, which has the same functionality
as the `getenv()` functions in C language.

## What do you think? :octocat:

[//]: # (badges)

[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
