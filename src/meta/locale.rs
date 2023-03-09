use chrono::Locale;
use once_cell::sync::OnceCell;
use sys_locale::get_locale;

fn locale_str() -> String {
    get_locale().unwrap_or_default().replace('-', "_")
}

/// Finds current locale
pub fn current_locale() -> Locale {
    const DEFAULT: Locale = Locale::en_US;
    static CACHE: OnceCell<Locale> = OnceCell::new();

    *CACHE.get_or_init(|| Locale::try_from(locale_str().as_str()).unwrap_or(DEFAULT))
}
