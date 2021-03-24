# cmp_env
research: compare the accessing to environment variables

## Slower libc::getenv()

|         `name`          |   `bench`   |   `musl`    |
|:------------------------|------------:|------------:|
| cmp-env_cache           |   18.306 us |   55.115 us |
| cmp-libc_getenv         |  132.650 us |  623.880 us |
| cmp-std_env_var         |  142.800 us |  589.970 us |
| cmp-envmnt_get          |  171.770 us |  663.860 us |

- `us` is micro seconds, lower is better
- rustc 1.50.0 (cb75ad5db 2021-02-10)
- bench on intel Q6600 @ 2.40GHz

- [envmnt](https://crates.io/crates/envmnt) - is the many access-type to environment variables.
- env_cache - is std::env::var() + HashMap<>, my simple implements.

## This benchmark measures getenv(key).

It is based on `std::env::var()`, which has the same functionality
as the `getenv()` functions in C language.

## What do you think? :octocat:
