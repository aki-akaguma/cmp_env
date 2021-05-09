use criterion::{criterion_group, criterion_main, Criterion};

fn process_std_env_var(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    cmp_env::do_std_env_var(texts, pattern)
}

fn process_libc_getenv(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    cmp_env::do_libc_getenv(texts, pattern)
}

fn process_envmnt_get(texts: &[&str], pattern: &str) -> anyhow::Result<usize> {
    cmp_env::do_envmnt_get(texts, pattern)
}

fn process_env_cache(
    texts: &[&str],
    pattern: &str,
    env_cache: &mut cmp_env::EnvCache,
) -> anyhow::Result<usize> {
    cmp_env::do_env_cache(texts, pattern, env_cache)
}

mod create_data;

fn criterion_benchmark(c: &mut Criterion) {
    let (v, match_cnt, pat_string_s) = create_data::setup_env();
    let vv: Vec<&str> = v.iter().map(|item| item.as_str()).collect();
    //
    let mut ec = cmp_env::EnvCache::new();
    //
    match process_std_env_var(
        criterion::black_box(&vv),
        criterion::black_box(pat_string_s),
    ) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_libc_getenv(
        criterion::black_box(&vv),
        criterion::black_box(pat_string_s),
    ) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_envmnt_get(
        criterion::black_box(&vv),
        criterion::black_box(pat_string_s),
    ) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    match process_env_cache(
        criterion::black_box(&vv),
        criterion::black_box(pat_string_s),
        &mut ec,
    ) {
        Ok(n) => {
            assert_eq!(n, match_cnt);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    ec.clear();
    //
    c.bench_function("cmp-std_env_var", |b| {
        b.iter(|| {
            let _r = process_std_env_var(
                criterion::black_box(&vv),
                criterion::black_box(pat_string_s),
            );
        })
    });
    c.bench_function("cmp-libc_getenv", |b| {
        b.iter(|| {
            let _r = process_libc_getenv(
                criterion::black_box(&vv),
                criterion::black_box(pat_string_s),
            );
        })
    });
    c.bench_function("cmp-envmnt_get", |b| {
        b.iter(|| {
            let _r = process_envmnt_get(
                criterion::black_box(&vv),
                criterion::black_box(pat_string_s),
            );
        })
    });
    c.bench_function("cmp-env_cache", |b| {
        b.iter(|| {
            let _r = process_env_cache(
                criterion::black_box(&vv),
                criterion::black_box(pat_string_s),
                &mut ec,
            );
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(std::time::Duration::from_millis(300))
        .measurement_time(std::time::Duration::from_millis(1500));
    targets = criterion_benchmark
}
//criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
