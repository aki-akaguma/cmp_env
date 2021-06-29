# cmp_env
research: compare the accessing to environment variables

## Slower libc::getenv()

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
