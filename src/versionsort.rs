use core::cmp::PartialOrd;
use std::cmp::Ordering;

use regex::Regex;

pub fn compare_version_sort(a: &str, b: &str) -> Ordering {
    let mut remainder_a = a;
    let mut remainder_b = b;
    loop {
        let (a_non_digit_part, out_a) = non_digit_seq(remainder_a);
        let (b_non_digit_part, out_b) = non_digit_seq(remainder_b);
        let cmp = compare_non_digit_seq(a_non_digit_part, b_non_digit_part);
        if cmp != Ordering::Equal {
            return cmp;
        }
        remainder_a = out_a;
        remainder_b = out_b;
        let (a_digit_part, out_a) = digit_seq(remainder_a);
        let (b_digit_part, out_b) = digit_seq(remainder_b);

        let a_digits = a_digit_part.parse::<u64>().unwrap_or_default();
        let b_digits = b_digit_part.parse::<u64>().unwrap_or_default();
        let cmp = a_digits.cmp(&b_digits);
        if cmp != Ordering::Equal {
            return cmp;
        }

        remainder_a = out_a;
        remainder_b = out_b;

        if remainder_a.is_empty() && remainder_b.is_empty() {
            return Ordering::Equal;
        }
        if remainder_a.is_empty() {
            return Ordering::Less;
        }
        if remainder_b.is_empty() {
            return Ordering::Greater;
        }
    }
}

pub fn compare(a: &str, b: &str) -> Ordering {
    let (a_str, _) = split_extension(a);
    let (b_str, _) = split_extension(b);
    // compare without the file extensions
    let cmp = compare_version_sort(a_str, b_str);
    if cmp != Ordering::Equal {
        return cmp;
    }
    // compare with the file extensions
    let cmp = compare_version_sort(a, b);
    if cmp != Ordering::Equal {
        return cmp;
    }
    // at this point the file extensions are the same, so we compare the full strings.
    // this helps with cases like a0001 and a1
    a.cmp(b)
}

fn split_extension(s: &str) -> (&str, &str) {
    let re = Regex::new(r"(\.[A-Za-z~][A-Za-z0-9~]*)*$").unwrap();

    match re.find(s) {
        Some(m) => {
            let (a, b) = s.split_at(m.start());
            (a, b)
        }
        None => (s, ""),
    }
}

#[derive(Eq)]
struct VersionSortChar(Option<char>);

impl From<Option<char>> for VersionSortChar {
    fn from(c: Option<char>) -> Self {
        Self(c)
    }
}

impl PartialOrd for VersionSortChar {
    // Based on https://github.com/coreutils/coreutils/blob/master/doc/sort-version.texi
    // For non-digit characters, we apply the following rules:
    //   ~(tilde) comes before all other strings, even the empty string.
    //   ASCII letters sort before other bytes.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.0, other.0) {
            (None, None) => Some(Ordering::Equal),
            (Some(a), None) => {
                if a == '~' {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
            (None, Some(b)) => {
                if b == '~' {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Less)
                }
            }
            (Some(a), Some(b)) => {
                if a == b {
                    return Some(Ordering::Equal);
                }
                if a == '~' {
                    return Some(Ordering::Less);
                }
                if b == '~' {
                    return Some(Ordering::Greater);
                }
                match (a.is_ascii_alphabetic(), b.is_ascii_alphabetic()) {
                    // ASCII letters sort before other bytes. If they are both ASCII
                    // or both are not ASCII sort normally.
                    (true, true) => Some(a.cmp(&b)),
                    (false, false) => Some(a.cmp(&b)),
                    (true, false) => Some(Ordering::Less),
                    (false, true) => Some(Ordering::Greater),
                }
            }
        }
    }
}

impl PartialEq for VersionSortChar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

fn compare_non_digit_seq(a: &str, b: &str) -> Ordering {
    let mut a_chars = a.chars();
    let mut b_chars = b.chars();
    loop {
        let a_char = a_chars.next();
        let b_char = b_chars.next();
        if a_char.is_none() && b_char.is_none() {
            return Ordering::Equal;
        }
        let a_char = VersionSortChar::from(a_char);
        let b_char = VersionSortChar::from(b_char);
        let cmp = a_char.partial_cmp(&b_char).unwrap();
        if cmp == Ordering::Equal {
            continue;
        }
        return cmp;
    }
}

fn non_digit_seq(a: &str) -> (&str, &str) {
    let ind = a.chars().take_while(|c| !c.is_ascii_digit()).count();
    (&a[..ind], &a[ind..])
}

fn digit_seq(a: &str) -> (&str, &str) {
    let ind = a.chars().take_while(|c| c.is_ascii_digit()).count();
    (&a[..ind], &a[ind..])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_extension() {
        // Examples from https://github.com/coreutils/coreutils/blob/master/doc/sort-version.texi
        assert_eq!(split_extension("hello-8.txt"), ("hello-8", ".txt"));
        assert_eq!(split_extension("hello-8.2.txt"), ("hello-8.2", ".txt"));
        assert_eq!(
            split_extension("hello-8.0.12.tar.gz"),
            ("hello-8.0.12", ".tar.gz")
        );
        assert_eq!(split_extension("hello-8.2"), ("hello-8.2", ""));
        assert_eq!(split_extension("hello.foobar65"), ("hello", ".foobar65"));
        assert_eq!(
            split_extension("gcc-c++-10.8.12-0.7rc2.fc9.tar.bz2"),
            ("gcc-c++-10.8.12-0.7rc2", ".fc9.tar.bz2")
        );
        assert_eq!(split_extension(".autom4te.cfg"), ("", ".autom4te.cfg"));
    }

    #[test]
    fn test_non_digit_sorting() {
        let mut list = vec!["aaa", "aa", "aab", "aa&", "aa_", "aa~", "a"];
        list.sort_by(|a, b| compare_non_digit_seq(a, b));

        assert_eq!(
            list,
            vec![
                // Absolute shortest comes first
                "a", // Tilde comes before empty string
                "aa~", "aa", // ASCII letters come before other bytes
                "aaa", "aab", "aa&", "aa_",
            ]
        );
    }

    #[test]
    fn test_non_digit_seq() {
        let a = "file_1.txt";
        let (seq, remainder) = non_digit_seq(a);
        assert_eq!(seq, "file_");
        assert_eq!(remainder, "1.txt");

        let (seq, remainder) = non_digit_seq(&a[5..]);
        assert_eq!(seq, "");
        assert_eq!(remainder, "1.txt");

        let (seq, remainder) = non_digit_seq(&a[6..]);
        assert_eq!(seq, ".txt");
        assert_eq!(remainder, "");
    }

    #[test]
    fn test_unusual_test_case() {
        let mut list = vec![
            "a.txt", "b 1.txt", "b 10.txt", "b 11.txt", "b 5.txt", "Ssm.txt",
        ];
        list.sort_by(|a, b| compare(a, b));

        assert_eq!(
            list,
            vec!["Ssm.txt", "a.txt", "b 1.txt", "b 5.txt", "b 10.txt", "b 11.txt",]
        );
    }

    #[test]
    fn test_small_list() {
        let mut list = vec![
            "file_1.txt",
            "file_10.txt",
            "file_2.txt",
            "file_20.txt",
            "file_11.txt",
            "file_1a.txt",
            "file_1B.txt",
            "file_a1.txt",
            "file_A1.txt",
            "file_001.txt",
        ];

        list.sort_by(|a, b| compare(a, b));

        assert_eq!(
            list,
            vec![
                "file_001.txt",
                "file_1.txt",
                "file_1B.txt",
                "file_1a.txt",
                "file_2.txt",
                "file_10.txt",
                "file_11.txt",
                "file_20.txt",
                "file_A1.txt",
                "file_a1.txt",
            ]
        );
    }

    #[test]
    fn test_large_list() {
        let mut original_list = vec![
            "file1.txt",
            "file2.txt",
            "file3.txt",
            "file10.txt",
            "file10a.txt",
            "file10b.txt",
            "file10c.txt",
            "file11.txt",
            "file12.txt",
            "file1a.txt",
            "file1b.txt",
            "file1c.txt",
            "file20.txt",
            "file200.txt",
            "file2000.txt",
            "file2001.txt",
            "file201.txt",
            "file21.txt",
            "file22.txt",
            "file100.txt",
            "file1000.txt",
            "file101.txt",
            "file1002.txt",
            "file102.txt",
            "file2002.txt",
            "file202.txt",
            "file1001.txt",
            "fileA.txt",
            "fileB.txt",
            "fileC.txt",
            "filea1.txt",
            "filea2.txt",
            "filea3.txt",
            "filea10.txt",
            "filea10b.txt",
            "filea10c.txt",
            "filea12.txt",
            "filea20.txt",
            "filea100.txt",
            "filea200.txt",
            "filea1000.txt",
            "filea1001.txt",
            "filea101.txt",
            "filea1002.txt",
            "filea102.txt",
            "filea10a.txt",
            "filea11.txt",
            "filea1a.txt",
            "filea1b.txt",
            "filea1c.txt",
            "filea2000.txt",
            "filea2001.txt",
            "filea201.txt",
            "filea21.txt",
            "filea2002.txt",
            "filea202.txt",
            "filea22.txt",
            "fileaA.txt",
            "fileaB.txt",
            "fileaC.txt",
            "fileb1.txt",
            "fileb2.txt",
            "fileb3.txt",
            "fileb10.txt",
            "fileb100.txt",
            "fileb101.txt",
            "fileb102.txt",
            "fileb10a.txt",
            "fileb10b.txt",
            "fileb10c.txt",
            "fileb11.txt",
            "fileb12.txt",
            "fileb20.txt",
            "fileb200.txt",
            "fileb1001.txt",
            "fileb2000.txt",
            "fileb2001.txt",
            "fileb201.txt",
            "fileb21.txt",
            "fileb22.txt",
            "fileb1000.txt",
            "fileb2002.txt",
            "fileb202.txt",
            "fileb1002.txt",
        ];
        original_list.sort_by(|a, b| compare(a, b));
        let expected = vec![
            "file1.txt",
            "file1a.txt",
            "file1b.txt",
            "file1c.txt",
            "file2.txt",
            "file3.txt",
            "file10.txt",
            "file10a.txt",
            "file10b.txt",
            "file10c.txt",
            "file11.txt",
            "file12.txt",
            "file20.txt",
            "file21.txt",
            "file22.txt",
            "file100.txt",
            "file101.txt",
            "file102.txt",
            "file200.txt",
            "file201.txt",
            "file202.txt",
            "file1000.txt",
            "file1001.txt",
            "file1002.txt",
            "file2000.txt",
            "file2001.txt",
            "file2002.txt",
            "fileA.txt",
            "fileB.txt",
            "fileC.txt",
            "filea1.txt",
            "filea1a.txt",
            "filea1b.txt",
            "filea1c.txt",
            "filea2.txt",
            "filea3.txt",
            "filea10.txt",
            "filea10a.txt",
            "filea10b.txt",
            "filea10c.txt",
            "filea11.txt",
            "filea12.txt",
            "filea20.txt",
            "filea21.txt",
            "filea22.txt",
            "filea100.txt",
            "filea101.txt",
            "filea102.txt",
            "filea200.txt",
            "filea201.txt",
            "filea202.txt",
            "filea1000.txt",
            "filea1001.txt",
            "filea1002.txt",
            "filea2000.txt",
            "filea2001.txt",
            "filea2002.txt",
            "fileaA.txt",
            "fileaB.txt",
            "fileaC.txt",
            "fileb1.txt",
            "fileb2.txt",
            "fileb3.txt",
            "fileb10.txt",
            "fileb10a.txt",
            "fileb10b.txt",
            "fileb10c.txt",
            "fileb11.txt",
            "fileb12.txt",
            "fileb20.txt",
            "fileb21.txt",
            "fileb22.txt",
            "fileb100.txt",
            "fileb101.txt",
            "fileb102.txt",
            "fileb200.txt",
            "fileb201.txt",
            "fileb202.txt",
            "fileb1000.txt",
            "fileb1001.txt",
            "fileb1002.txt",
            "fileb2000.txt",
            "fileb2001.txt",
            "fileb2002.txt",
        ];
    }
}
