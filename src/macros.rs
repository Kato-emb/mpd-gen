/// s
///
#[macro_export]
macro_rules! define_regex {
    ($name:ident, $fmt:expr, $($arg:expr),*) => {
        pub static $name: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            let pattern = format!($fmt, $($arg),*);
            regex::Regex::new(&pattern).expect(concat!("Invalid regex pattern for ", stringify!($name)))
        });
    };
}
