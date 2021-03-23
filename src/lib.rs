pub fn do_std_env_var(keys: &[&str], pattern: &str) -> anyhow::Result<usize> {
    let mut found: usize = 0;
    for key in keys {
        match std::env::var(key) {
            Ok(s) => {
                if s == pattern {
                    found += 1;
                }
            }
            Err(e) => {
                eprintln!("can not found {}: {}", key, e);
                std::process::exit(1);
            }
        }
    }
    Ok(found)
}

pub fn do_libc_getenv(keys: &[&str], pattern: &str) -> anyhow::Result<usize> {
    use std::ffi::CStr;
    use std::ffi::CString;
    let mut found: usize = 0;
    for key in keys {
        let key_c = CString::new(*key).expect("CString::new failed");
        let val = {
            let c_buf = unsafe { libc::getenv(key_c.as_ptr()) };
            if c_buf.is_null() {
                eprintln!("can not found {}", key);
                std::process::exit(1);
            }
            let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
            let str_slice: &str = c_str.to_str().unwrap();
            let str_buf: String = str_slice.to_owned();
            str_buf
        };
        if val == pattern {
            found += 1;
        }
    }
    Ok(found)
}

pub fn do_env_cache(
    keys: &[&str],
    pattern: &str,
    env_cache: &mut EnvCache,
) -> anyhow::Result<usize> {
    let mut found: usize = 0;
    for key in keys {
        match env_cache.get(key) {
            Ok(s) => {
                if s == pattern {
                    found += 1;
                }
            }
            Err(e) => {
                eprintln!("can not found {}: {}", key, e);
                std::process::exit(1);
            }
        }
    }
    Ok(found)
}

#[derive(Debug)]
pub struct EnvCache {
    cache: std::collections::HashMap<String, String>,
}

impl EnvCache {
    pub fn new() -> EnvCache {
        EnvCache {
            cache: std::collections::HashMap::new(),
        }
    }
    pub fn get(&mut self, key: &str) -> Result<String, std::env::VarError> {
        match self.cache.get(key) {
            Some(value) => return Ok(value.to_string()),
            None => (),
        }
        match std::env::var(key) {
            Ok(value) => {
                let _ = self.cache.insert(key.to_string(), value.to_string());
                Ok(value.to_string())
            }
            Err(e) => Err(e),
        }
    }
    pub fn clear(&mut self) {
        self.cache.clear()
    }
}
impl std::default::Default for EnvCache {
    fn default() -> EnvCache {
        EnvCache::new()
    }
}
