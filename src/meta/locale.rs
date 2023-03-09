use chrono::Locale;
use once_cell::sync::OnceCell;
use sys_locale::get_locale;

/// Finds current locale
pub fn current_locale() -> Locale {
    const DEFAULT: Locale = Locale::en_US;
    static CACHE: OnceCell<Locale> = OnceCell::new();

    fn locale_str() -> String {
        get_locale().unwrap_or("".to_string()).replace('-', "_")
    }

    *CACHE
        .get_or_init(|| Locale::try_from(locale_str().as_str()).unwrap_or(DEFAULT))
}
