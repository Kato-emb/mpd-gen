use crate::define_regex;

pub const CHARSET: &str = r"[A-Za-z0-9.\-]+";
pub const SQUOTE: &str = r"'";
pub const LANGUAGE: &str = r"[a-zA-Z]{2,3}";
pub const ID_LIST: &str = r"([A-Za-z0-9.\-]+(?:;[^,]+)*)";
pub const ID_SIMPLE: &str = r"[A-Za-z0-9.\-]+";

pub const ID: &str = r"[A-Za-z_][A-Za-z0-9_\-\.]*";

pub const NO_WHITE_SPACE: &str = r"[^\r\n\t \p{Z}]*";
pub const FRAMERATE: &str = r"[0-9]+(/[1-9][0-9]*)?";
pub const RFC7233_RANGE: &str = r"([0-9]*)(-([0-9]*))?";

pub const URN: &str = r"urn:[a-zA-Z0-9\-]+(:[a-zA-Z0-9\-]+)*";
pub const URL: &str = r"https?://[a-zA-Z0-9\-._~:/?#\[@\]!$&'()*+,;=]+";

define_regex!(
    PATTERN_FANCY,
    "^{}{}{}{}{}$",
    CHARSET,
    SQUOTE,
    LANGUAGE,
    SQUOTE,
    ID_LIST
);
define_regex!(PATTERN_SIMPLE, "^{0}(?:,{0})*$", ID_SIMPLE);
define_regex!(PATTERN_URN, "^{}$", URN);
define_regex!(PATTERN_URL, "^{}$", URL);
define_regex!(PATTERN_NO_WHITESPACE, "^{}$", NO_WHITE_SPACE);
define_regex!(PATTERN_FRAMERATE, "^{}$", FRAMERATE);
define_regex!(PATTERN_RFC7233_RANGE, "^{}$", RFC7233_RANGE);

define_regex!(PATTERN_ID, "^{}$", ID);

define_regex!(PATTERN_LANG, "^{}$", LANGUAGE);
