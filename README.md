# cmp_env
research: compare the accessing to environment variables

## Slower libc::getenv()

|         `name`          |   `bench`   |   `musl`    |
|:------------------------|------------:|------------:|
| cmp-env_cache           |   17.552 us |   53.633 us |
| cmp-libc_getenv         |  130.600 us |  624.720 us |
| cmp-envmnt_get          |  146.220 us |  662.910 us |
| cmp-std_env_var         |  150.750 us |  590.460 us |

- `us` is micro seconds, lower is better
- rustc 1.51.0 (2fd73fabe 2021-03-23)
- bench on intel Q6600 @ 2.40GHz

- [envmnt](https://crates.io/crates/envmnt) - is the many access-type to environment variables.
- env_cache - is std::env::var() + HashMap<>, my simple implements.

## This benchmark measures getenv(key).

It is based on `std::env::var()`, which has the same functionality
as the `getenv()` functions in C language.

## What do you think? :octocat:
