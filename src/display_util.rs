use std::borrow::Cow;
use std::fmt;
use std::path::Path;

#[inline]
fn is_dangerous(c: char) -> bool {
    (c as u32) < 0x20
        || c == '\u{7f}'
        || matches!(c, '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}')
}

pub fn sanitize_for_terminal(s: &str) -> Cow<'_, str> {
    if !s.chars().any(is_dangerous) {
        return Cow::Borrowed(s);
    }
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if is_dangerous(c) {
            out.extend(c.escape_default());
        } else {
            out.push(c);
        }
    }
    Cow::Owned(out)
}

pub struct SafePath<'a>(pub &'a Path);

impl fmt::Display for SafePath<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&sanitize_for_terminal(&self.0.to_string_lossy()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn sanitize_passes_through_clean_strings() {
        assert!(matches!(sanitize_for_terminal("hello.txt"), Cow::Borrowed(_)));
        assert_eq!(sanitize_for_terminal("hello.txt"), "hello.txt");
    }

    #[test]
    fn sanitize_preserves_non_ascii() {
        let s = "café_文件.txt";
        assert!(matches!(sanitize_for_terminal(s), Cow::Borrowed(_)));
        assert_eq!(sanitize_for_terminal(s), s);
    }

    #[test]
    fn sanitize_escapes_esc() {
        let s = "a\x1b[31mred\x1b[0m";
        let out = sanitize_for_terminal(s);
        assert!(matches!(out, Cow::Owned(_)));
        assert!(!out.contains('\x1b'));
        assert!(out.contains("\\u{1b}"));
    }

    #[test]
    fn sanitize_escapes_newline_and_tab() {
        let out = sanitize_for_terminal("a\nb\tc");
        assert_eq!(out, "a\\nb\\tc");
    }

    #[test]
    fn sanitize_escapes_bell() {
        let out = sanitize_for_terminal("a\x07b");
        assert!(!out.contains('\x07'));
    }

    #[test]
    fn sanitize_escapes_del() {
        let out = sanitize_for_terminal("a\x7fb");
        assert!(!out.contains('\x7f'));
    }

    #[test]
    fn sanitize_escapes_bidi_override() {
        let out = sanitize_for_terminal("innocent\u{202e}gpj.exe");
        assert!(matches!(out, Cow::Owned(_)));
        assert!(!out.contains('\u{202e}'));
        assert!(out.contains("\\u{202e}"));
    }

    #[test]
    fn sanitize_escapes_bidi_isolate() {
        let out = sanitize_for_terminal("a\u{2066}b");
        assert!(!out.contains('\u{2066}'));
    }

    #[test]
    fn safe_path_display_sanitizes() {
        let p = PathBuf::from("dir/evil\x1b[2Jcleared/file");
        let rendered = format!("{}", SafePath(&p));
        assert!(!rendered.contains('\x1b'));
        assert!(rendered.contains("\\u{1b}"));
    }

    #[test]
    fn safe_path_clean_is_unchanged() {
        let p = PathBuf::from("/home/user/doc.txt");
        assert_eq!(format!("{}", SafePath(&p)), "/home/user/doc.txt");
    }
}
