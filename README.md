# cmp_env
research: compare std::env::var()

## Slower libc::getenv()

|         `name`          |   `bench`   |   `musl`    |
|:------------------------|------------:|------------:|
| cmp-env_cache           |   17.868 uc |   55.549 uc |
| cmp-libc_getenv         |  144.090 uc |  618.360 uc |
| cmp-std_env_var         |  151.540 uc |  594.370 uc |

- rustc 1.50.0 (cb75ad5db 2021-02-10)

## This benchmark measures getenv(key).

It is based on `std::env::var()`, which has the same functionality
as the `getenv()` functions in C language.
