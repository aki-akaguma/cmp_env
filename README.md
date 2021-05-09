# cmp_env
research: compare the accessing to environment variables

## Slower libc::getenv()

|         `name`          |   `bench`   |   `musl`    |
|:------------------------|------------:|------------:|
| cmp-env_cache           |   17.899 us |   52.371 us |
| cmp-std_env_var         |  144.760 us |  594.500 us |
| cmp-envmnt_get          |  151.480 us |  666.190 us |
| cmp-libc_getenv         |  153.880 us |  624.450 us |

- `us` is micro seconds, lower is better
- rustc 1.52.0 (88f19c6da 2021-05-03)
- bench on intel Q6600 @ 2.40GHz

- [envmnt](https://crates.io/crates/envmnt) - is the many access-type to environment variables.
- env_cache - is std::env::var() + HashMap<>, my simple implements.

## This benchmark measures getenv(key).

It is based on `std::env::var()`, which has the same functionality
as the `getenv()` functions in C language.

## What do you think? :octocat:
