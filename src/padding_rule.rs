use crate::flags::Flags;
use crate::icon::Icons;
use crate::meta::Meta;

use unicode_width::UnicodeWidthStr;

/// Holds the maximum width along metas for aligning
pub struct PaddingRule {
    pub size: Option<usize>,
    pub icon: Option<usize>,
}

impl PaddingRule {
    pub fn new() -> Self {
        Self {
            size: None,
            icon: None,
        }
    }
    /// Detects the maximum length of the size values in the given metas.
    fn detect_size_lengths(metas: &[Meta], flags: &Flags, recursive: bool) -> usize {
        let mut max_value_length: usize = 0;

        for meta in metas {
            let value_len = match &meta.size {
                Some(size) => size.value_string(flags).len(),
                None => 0,
            };

            if value_len > max_value_length {
                max_value_length = value_len;
            }

            // search for every element recursively since 'size' is rendered globally aligned
            if recursive {
                if let Some(subs) = &meta.content {
                    let sub_length = Self::detect_size_lengths(subs, flags, true);
                    if sub_length > max_value_length {
                        max_value_length = sub_length;
                    }
                }
            }
        }

        max_value_length
    }
    pub fn set_size_lengths(
        &mut self,
        metas: &[Meta],
        flags: &Flags,
        recursive: bool,
    ) -> Option<usize> {
        let old = self.size;
        self.size = Some(Self::detect_size_lengths(metas, flags, recursive));
        old
    }
    /// Detects the maximum length of the icon in the given metas.
    fn detect_icon_lengths(metas: &[Meta], icons: &Icons, recursive: bool) -> usize {
        // max length of icon + separator
        let mut max_icon_visible_width: usize = 0;

        for meta in metas {
            let icon_visible_width = UnicodeWidthStr::width(icons.get(&meta.name).as_str());
            max_icon_visible_width = max_icon_visible_width.max(icon_visible_width);

            if recursive {
                if let Some(subs) = &meta.content {
                    let sub_length = Self::detect_icon_lengths(subs, icons, true);
                    max_icon_visible_width = max_icon_visible_width.max(sub_length);
                }
            }
        }

        max_icon_visible_width
    }
    pub fn set_icon_lengths(
        &mut self,
        metas: &[Meta],
        icons: &Icons,
        recursive: bool,
    ) -> Option<usize> {
        let old = self.icon;
        self.icon = Some(Self::detect_icon_lengths(metas, icons, recursive));
        old
    }
}
