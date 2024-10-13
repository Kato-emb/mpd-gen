use crate::define_regex;

/// XML Schema pattern: `[\-+]?[0-9]+`
const INTEGER: &str = r"[\-\+]?[0-9]+";

/// XML Schema pattern: `[\i-[:]][\c-[:]]*`
const NC_NAME: &str = r"[A-Za-z_][A-Za-z0-9_\-\.]*";

/// XML Schema pattern: `[a-zA-Z]{1,8}(-[a-zA-Z0-9]{1,8})*`
const LANGUAGE: &str = r"[a-zA-Z]{1,8}(-[a-zA-Z0-9]{1,8})*";

/// charset (RFC2045-2046)
///
/// [IANA Charset Registry](https://www.iana.org/assignments/character-sets/character-sets.xhtml)
const CHARSET: &str = r"[A-Za-z0-9\-]+";
const ID_ENCODED: &str = r"[A-Za-z0-9.\-]+";
const ID_SIMPLE: &str = r"[A-Za-z0-9\-]+";

const NO_WHITESPACE: &str = r"[^\r\n\t \p{Z}]*";
// const FRAMERATE: &str = r"[0-9]+(/[1-9][0-9]*)?";
// const RFC7233_RANGE: &str = r"([0-9]*)(-([0-9]*))?";

const URN: &str = r"urn:[a-zA-Z0-9\-]+(:[a-zA-Z0-9\-]+)*";
const URL: &str = r"https?://[a-zA-Z0-9\-._~:/?#\[@\]!$&'()*+,;=]+";

define_regex!(PATTERN_INTEGER, "^{}$", INTEGER);
define_regex!(PATTERN_NC_NAME, "^{}$", NC_NAME);
define_regex!(PATTERN_LANG, "^{}$", LANGUAGE);

define_regex!(PATTERN_PROFILE, "^({0}|{1})((,({0}|{1}))*)$", URN, URL);
define_regex!(PATTERN_NO_WHITESPACE, "^{}$", NO_WHITESPACE);

define_regex!(
    PATTERN_FANCY,
    r"^(?:(?P<charset>{})'(?P<language>{})')?(?P<codecs>{2}(?:,\s*{2})*)$",
    CHARSET,
    LANGUAGE,
    ID_ENCODED
);
define_regex!(PATTERN_SIMPLE, r"^{0}(?:,\s*{0})*$", ID_SIMPLE);
