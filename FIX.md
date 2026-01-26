# Fix #1014 and improve header underline styling

So I've been working on a similar ls utility myself and I've been taking a lot of inspiration from this project and eza. While poking around the code, I noticed a couple of things that bugged me visually and decided to fix them.

## Header Underline Fix

The main thing was the header underlines in long format (`lsd -l --header`). Previously, the underlines would stretch across the entire column width with padding, which looked kind of off when you had varying column widths. I thought it made sense for the underline to stay within the bounds of the actual header text, that way, it looks visually even.

**Before:**
**After:**

While I was at it, I updated term_grid to 0.2.0, and ran into errors relating to a missing alignment specification, this led me to change the header alignment so text columns (like Name, User, Group) are left-aligned and numeric columns (like Size, Date, INode, Links) are right-aligned. This made it easier to fix the header underline 'problem'.

The fix was in `add_header()` in `src/display.rs`, so instead of manually padding headers and then underlining the whole thing, I now just underline the header text and let term_grid handle the alignment via its `Alignment` enum.

## Issue #1014 Fix

After I made that change, I thought it would be nice to fix an issue from lsd, so I noticed issue #1014.

The root cause was that term_grid 0.2 doesn't automatically strip ANSI escape codes when calculating cell widths. The fix is actually already in the code, when creating cells, we use `get_visible_width()` which properly calculates the visible width excluding ANSI codes:

```rust
cells.push(Cell {
    width: get_visible_width(&block, flags.hyperlink == HyperlinkOption::Always),
    contents: block,
    alignment: Alignment::Left
});
```

This way term_grid gets the correct width (e.g., 6 for "f1.txt") instead of the full string length including escape codes (e.g., 22 for "\x1b[38;5;184mf1.txt\x1b[39m").

Added a test `test_grid_layout_with_colors_issue_1014` to make sure this doesn't regress.

## Files Changed

- `src/display.rs` - Header alignment fix and issue #1014 test
- `src/flags/blocks.rs` - Added `is_numeric()` method to determine column alignment
- `Cargo.toml` - Updated term_grid to 0.2.0

## Testing

`cargo test` output:

```shell
running 392 tests
Warning: the 'themes' directory is deprecated, use 'colors.yaml' instead.

test color::elem::test_default_theme_color ... ok
test color::tests::test_color_new_no_color_theme ... ok
test config_file::tests::test_read_bad_display ... ok
test config_file::tests::test_read_config_ok ... ok
test config_file::tests::test_read_config_bad_bool ... ok
test config_file::tests::test_read_config_file_not_found ... ok
test config_file::tests::test_read_default ... ok
test display::tests::test_display_get_visible_width_hypelink_simple ... ok
test display::tests::test_display_get_visible_width_without_colors ... ok
test display::tests::test_display_get_visible_width_without_icons ... ok
test display::tests::test_folder_path ... ok
test display::tests::test_display_get_visible_width_with_colors ... ok
test display::tests::test_folder_path_with_links ... ok
test display::tests::test_display_tree_with_all ... ok
test display::tests::test_grid_layout_with_colors_issue_1014 ... ok
test color::tests::test_color_new_custom_theme ... ok
test color::tests::test_color_new_custom_no_file_theme ... ok
test display::tests::test_grid_no_header_with_empty_meta ... ok
test color::tests::test_color_new_bad_legacy_custom_theme ... ok
test display::tests::test_grid_all_block_headers ... ok
test display::tests::test_tree_align_subfolder ... ok
test display::tests::test_display_get_visible_width_with_icons ... ok
test flags::blocks::test_block::test_err ... ok
test flags::blocks::test_block::test_block_headers ... ok
test flags::blocks::test_block::test_context ... ok
test flags::blocks::test_block::test_date ... ok
test flags::blocks::test_block::test_git_status ... ok
test flags::blocks::test_block::test_inode ... ok
test flags::blocks::test_block::test_links ... ok
test flags::blocks::test_block::test_name ... ok
test flags::blocks::test_block::test_permission ... ok
test flags::blocks::test_block::test_group ... ok
test flags::blocks::test_block::test_size_value ... ok
test display::tests::test_tree_edge_before_name ... ok
test display::tests::test_tree_size_first_without_name ... ok
test flags::blocks::test_block::test_user ... ok
test flags::blocks::test_block::test_size ... ok
test flags::blocks::test_blocks::test_configure_from_prepend_inode_with_long ... ok
test flags::blocks::test_blocks::test_configure_from_ignore_prepend_inode_with_long ... ok
test flags::blocks::test_blocks::test_configure_from_with_blocks_and_long ... ok
test flags::blocks::test_blocks::test_configure_from_without_long ... ok
test flags::blocks::test_blocks::test_configure_from_with_long ... ok
test flags::blocks::test_blocks::test_configure_from_with_inode ... ok
test flags::blocks::test_blocks::test_configure_from_prepend_inode_without_long ... ok
test flags::blocks::test_blocks::test_configure_from_with_blocks_and_without_long ... ok
test flags::blocks::test_blocks::test_from_cli_every_second_one ... ok
test flags::blocks::test_blocks::test_configure_from_ignore_prepend_inode_without_long ... ok
test flags::blocks::test_blocks::test_context_present_if_context_on ... ok
test flags::blocks::test_blocks::test_context_not_present_on_cli ... ok
lsd: Not a valid block name: foo.

test flags::blocks::test_blocks::test_from_cli_multi_occurences ... ok
test flags::blocks::test_blocks::test_from_config_invalid_is_ignored ... ok
test flags::blocks::test_blocks::test_from_cli_one ... ok
test flags::blocks::test_blocks::test_from_cli_implicit_add_git_block ... ok
test flags::blocks::test_blocks::test_from_config_none ... ok
test flags::blocks::test_blocks::test_from_cli_multi_values ... ok
test flags::blocks::test_blocks::test_from_cli_none ... ok
test flags::blocks::test_blocks::test_from_config_one ... ok
test flags::blocks::test_blocks::test_from_config_reversed_default ... ok
test flags::blocks::test_blocks::test_from_config_every_second_one ... ok
test flags::blocks::test_blocks::test_from_cli_reversed_default ... ok
test flags::color::test_color_option::test_from_cli_never ... ok
test flags::color::test_color_option::test_from_config_always ... ok
test flags::color::test_color_option::test_from_config_classic_mode ... ok
test flags::color::test_color_option::test_from_config_auto ... ok
test flags::color::test_color_option::test_from_cli_color_multiple ... ok
test flags::blocks::test_blocks::test_only_one_context_no_other_blocks_affected ... ok
test flags::color::test_color_option::test_from_env_no_color ... ok
test flags::color::test_color_option::test_from_cli_classic_mode ... ok
test flags::color::test_color_option::test_from_cli_always ... ok
test flags::color::test_theme_option::test_from_config_bad_file_flag ... ok
test flags::color::test_color_option::test_from_config_none ... ok
test flags::color::test_theme_option::test_from_config_classic_mode ... ok
test flags::color::test_color_option::test_from_cli_none ... ok
test flags::color::test_theme_option::test_from_config_default ... ok
test flags::color::test_color_option::test_from_config_never ... ok
test flags::color::test_theme_option::test_from_config_no_color ... ok
test flags::color::test_theme_option::test_from_config_no_lscolor ... ok
test flags::color::test_theme_option::test_from_config_none_default ... ok
test flags::color::test_color_option::test_from_cli_auto ... ok
test flags::date::test::test_from_cli_classic_mode ... ok
test flags::date::test::test_from_cli_date_multi ... ok
test flags::date::test::test_from_config_classic_mode ... ok
test flags::date::test::test_from_cli_date ... ok
test flags::date::test::test_from_cli_locale ... ok
lsd: Not a valid date format: +%J.

test flags::date::test::test_from_cli_format_invalid - should panic ... ok
test flags::date::test::test_from_cli_none ... ok
test flags::date::test::test_from_config_format ... ok
test flags::date::test::test_from_config_date ... ok
test flags::date::test::test_from_cli_format ... ok
test flags::date::test::test_from_config_format_invalid ... ok
test flags::date::test::test_from_config_none ... ok
test flags::date::test::test_from_config_relative ... ok
test flags::date::test::test_from_environment_full_iso ... ok
test flags::date::test::test_from_cli_relative ... ok
lsd: Not a valid date value: .

test flags::date::test::test_parsing_order_config ... ok
test flags::date::test::test_from_environment_format ... ok
test flags::dereference::test::test_from_config_false ... ok
test flags::dereference::test::test_from_cli_none ... ok
test flags::dereference::test::test_from_config_none ... ok
test flags::dereference::test::test_from_config_true ... ok
test flags::dereference::test::test_from_cli_true ... ok
test flags::date::test::test_parsing_order_arg ... ok
test flags::display::test::test_from_cli_all ... ok
test flags::display::test::test_from_cli_none ... ok
test flags::display::test::test_from_cli_directory_only ... ok
test flags::display::test::test_from_cli_system_protected ... ok
test flags::display::test::test_from_cli_almost_all ... ok
test flags::display::test::test_from_config_all ... ok
test flags::display::test::test_from_config_almost_all ... ok
test flags::display::test::test_from_config_directory_only ... ok
test flags::display::test::test_from_config_none ... ok
test flags::display::test::test_from_config_visible_only ... ok
test flags::header::test::test_from_config_false ... ok
test flags::header::test::test_from_config_true ... ok
test flags::header::test::test_from_config_none ... ok
test flags::header::test::test_from_cli_none ... ok
test flags::header::test::test_from_cli_true ... ok
test flags::hyperlink::test_hyperlink_option::test_from_cli_always ... ok
test flags::hyperlink::test_hyperlink_option::test_from_cli_classic_mode ... ok
test flags::hyperlink::test_hyperlink_option::test_from_cli_hyperlink_when_multi ... ok
test flags::hyperlink::test_hyperlink_option::test_from_cli_auto ... ok
test flags::hyperlink::test_hyperlink_option::test_from_config_always ... ok
test flags::hyperlink::test_hyperlink_option::test_from_config_auto ... ok
test flags::hyperlink::test_hyperlink_option::test_from_config_classic_mode ... ok
test flags::hyperlink::test_hyperlink_option::test_from_config_never ... ok
test flags::hyperlink::test_hyperlink_option::test_from_config_none ... ok
test flags::hyperlink::test_hyperlink_option::test_from_cli_never ... ok
test flags::hyperlink::test_hyperlink_option::test_from_cli_none ... ok
test flags::icons::test_icon_option::test_from_cli_always ... ok
test flags::icons::test_icon_option::test_from_cli_classic_mode ... ok
test flags::icons::test_icon_option::test_from_cli_auto ... ok
test flags::icons::test_icon_option::test_from_config_always ... ok
test flags::icons::test_icon_option::test_from_config_auto ... ok
test flags::icons::test_icon_option::test_from_cli_icon_when_multi ... ok
test flags::icons::test_icon_option::test_from_cli_none ... ok
test flags::icons::test_icon_option::test_from_config_never ... ok
test flags::icons::test_icon_option::test_from_cli_never ... ok
test flags::icons::test_icon_option::test_from_config_none ... ok
test flags::icons::test_icon_separator::test_from_config_custom ... ok
test flags::icons::test_icon_separator::test_from_config_default ... ok
test flags::icons::test_icon_option::test_from_config_classic_mode ... ok
test flags::icons::test_icon_theme::test_from_cli_none ... ok
test flags::icons::test_icon_theme::test_from_cli_icon_multi ... ok
test flags::icons::test_icon_theme::test_from_cli_unicode ... ok
test flags::icons::test_icon_theme::test_from_config_fancy ... ok
test flags::icons::test_icon_theme::test_from_config_none ... ok
test flags::icons::test_icon_theme::test_from_config_unicode ... ok
test flags::ignore_globs::test::test_configuration_from_none ... ok
test flags::ignore_globs::test::test_configuration_from_config ... ok
test flags::ignore_globs::test::test_from_cli_none ... ok
test flags::icons::test_icon_theme::test_from_cli_fancy ... ok
test flags::ignore_globs::test::test_from_config_none ... ok
test flags::ignore_globs::test::test_configuration_from_args ... ok
test flags::indicators::test::test_from_config_false ... ok
test flags::indicators::test::test_from_config_true ... ok
test flags::indicators::test::test_from_cli_true ... ok
test flags::indicators::test::test_from_cli_none ... ok
test flags::indicators::test::test_from_config_none ... ok
test flags::layout::test::test_from_cli_none ... ok
test flags::layout::test::test_from_cli_oneline ... ok
test flags::layout::test::test_from_cli_oneline_through_blocks ... ok
test flags::layout::test::test_from_cli_oneline_through_long ... ok
test flags::layout::test::test_from_cli_tree ... ok
test flags::layout::test::test_from_config_grid ... ok
test flags::layout::test::test_from_config_none ... ok
test flags::layout::test::test_from_config_oneline ... ok
test flags::layout::test::test_from_config_tree ... ok
test flags::literal::test::test_from_config_false ... ok
test flags::literal::test::test_from_config_none ... ok
test flags::permission::test::test_default ... ok
test flags::literal::test::test_from_config_true ... ok
test flags::literal::test::test_from_cli_none ... ok
test flags::literal::test::test_from_cli_literal ... ok
test flags::permission::test::test_from_cli_none ... ok
test flags::permission::test::test_from_cli_default ... ok
test flags::permission::test::test_from_cli_attributes ... ok
test flags::permission::test::test_from_cli_permissions_classic ... ok
test flags::permission::test::test_from_cli_short ... ok
test flags::permission::test::test_from_cli_permissions_multi ... ok
test flags::permission::test::test_from_config_classic_mode ... ok
test flags::permission::test::test_from_config_octal ... ok
test flags::permission::test::test_from_config_none ... ok
test flags::permission::test::test_from_config_rwx ... ok
test flags::permission::test::test_from_cli_permissions_disable ... ok
test flags::permission::test::test_from_cli_unknown - should panic ... ok
test flags::recursion::test::test_depth_from_cli_neg_int ... ok
test flags::recursion::test::test_depth_from_cli_integer ... ok
test flags::recursion::test::test_depth_from_cli_empty ... ok
test flags::recursion::test::test_depth_from_cli_depth_multi ... ok
test flags::recursion::test::test_depth_from_cli_non_int ... ok
test flags::recursion::test::test_depth_from_config_none_max ... ok
test flags::recursion::test::test_depth_from_config_pos_integer ... ok
test flags::recursion::test::test_enabled_from_cli_empty ... ok
test flags::recursion::test::test_enabled_from_matches_empty_and_config_false ... ok
test flags::recursion::test::test_enabled_from_empty_matches_and_config ... ok
test flags::recursion::test::test_enabled_from_matches_empty_and_config_true ... ok
test flags::recursion::test::test_enabled_from_cli_true ... ok
test flags::size::test::test_default ... ok
test flags::size::test::test_from_cli_none ... ok
test flags::size::test::test_from_cli_default ... ok
test flags::size::test::test_from_cli_bytes ... ok
test flags::size::test::test_from_cli_short ... ok
test flags::size::test::test_from_cli_size_multi ... ok
test flags::size::test::test_from_cli_size_classic ... ok
test flags::size::test::test_from_config_bytes ... ok
test flags::size::test::test_from_config_classic_mode ... ok
test flags::size::test::test_from_cli_unknown - should panic ... ok
test flags::size::test::test_from_config_default ... ok
test flags::size::test::test_from_config_none ... ok
test flags::size::test::test_from_config_short ... ok
test flags::sorting::test_dir_grouping::test_from_cli_explicit_none ... ok
test flags::sorting::test_dir_grouping::test_from_cli_classic_mode ... ok
test flags::sorting::test_dir_grouping::test_from_cli_group_directories_first ... ok
test flags::sorting::test_dir_grouping::test_from_cli_first ... ok
test flags::sorting::test_dir_grouping::test_from_config_classic_mode ... ok
test flags::sorting::test_dir_grouping::test_from_config_empty ... ok
test flags::sorting::test_dir_grouping::test_from_config_explicit_empty ... ok
test flags::sorting::test_dir_grouping::test_from_cli_group_dirs_multi ... ok
test flags::sorting::test_dir_grouping::test_from_cli_none ... ok
test flags::sorting::test_dir_grouping::test_from_cli_last ... ok
test flags::sorting::test_dir_grouping::test_from_config_first ... ok
test flags::sorting::test_dir_grouping::test_from_config_last ... ok
test flags::sorting::test_dir_grouping::test_from_str_bad_value - should panic ... ok
test flags::sorting::test_sort_column::test_from_cli_no_sort ... ok
test flags::sorting::test_sort_column::test_from_cli_git ... ok
test flags::sorting::test_sort_column::test_from_cli_extension ... ok
test flags::sorting::test_sort_column::test_from_arg_cli_sort_git ... ok
test flags::sorting::test_sort_column::test_from_cli_none ... ok
test flags::sorting::test_sort_column::test_from_cli_size ... ok
test flags::sorting::test_sort_column::test_from_config_empty ... ok
test flags::sorting::test_sort_column::test_from_config_empty_column ... ok
test flags::sorting::test_sort_column::test_from_config_extension ... ok
test flags::sorting::test_sort_column::test_from_config_git_status ... ok
test flags::sorting::test_sort_column::test_from_config_name ... ok
test flags::sorting::test_sort_column::test_from_cli_time ... ok
test flags::sorting::test_sort_column::test_from_config_size ... ok
test flags::sorting::test_sort_column::test_from_config_time ... ok
test flags::sorting::test_sort_column::test_from_config_version ... ok
test flags::sorting::test_sort_column::test_from_cli_version ... ok
test flags::sorting::test_sort_column::test_multi_sort ... ok
test flags::sorting::test_sort_column::test_multi_sort_use_last ... ok
test flags::sorting::test_sort_order::test_from_cli_none ... ok
test flags::sorting::test_sort_order::test_from_config_empty ... ok
test flags::sorting::test_sort_order::test_from_config_empty_reverse ... ok
test flags::sorting::test_sort_order::test_from_config_default_config ... ok
test flags::sorting::test_sort_order::test_from_config_reverse_false ... ok
test flags::sorting::test_sort_order::test_from_cli_reverse ... ok
test flags::sorting::test_sort_order::test_from_config_reverse_true ... ok
test flags::symlink_arrow::test::test_symlink_arrow_default ... ok
test flags::symlink_arrow::test::test_symlink_arrow_from_config_utf8 ... ok
test flags::symlink_arrow::test::test_symlink_display ... ok
test flags::symlink_arrow::test::test_symlink_arrow_from_args_none ... ok
test flags::symlinks::test::test_from_config_false ... ok
test flags::symlinks::test::test_from_cli_true ... ok
test flags::symlinks::test::test_from_cli_none ... ok
test flags::symlinks::test::test_from_config_none ... ok
test flags::symlinks::test::test_from_config_true ... ok
test flags::sorting::test_sort_column::test_from_cli_sort ... ok
test flags::total_size::test::test_from_config_false ... ok
test flags::total_size::test::test_from_config_true ... ok
test flags::total_size::test::test_from_config_none ... ok
test flags::total_size::test::test_from_cli_true ... ok
test flags::total_size::test::test_from_cli_none ... ok
test flags::truncate_owner::test::test_from_config_all_fields_none ... ok
test flags::truncate_owner::test::test_from_config_all_fields_some ... ok
test flags::truncate_owner::test::test_from_cli_marker_some ... ok
test git::tests::compare_git_status ... ok
test flags::truncate_owner::test::test_from_cli_after_some ... ok
test flags::truncate_owner::test::test_from_config_none ... ok
test flags::truncate_owner::test::test_from_cli_none ... ok
test icon::test::get_icon_always_not_tty_default_file ... ok
test icon::test::get_icon_always_tty_default_file ... ok
test icon::test::get_icon_auto_tty ... ok
test icon::test::get_icon_default_directory ... ok
test icon::test::get_icon_default_directory_unicode ... ok
test icon::test::get_icon_default_file_icon_unicode ... ok
test icon::test::get_no_icon_auto ... ok
test icon::test::get_no_icon_never_not_tty ... ok
test icon::test::get_no_icon_never_tty ... ok
test meta::access_control::test::test_acl_and_selinux_indicator ... ok
test meta::access_control::test::test_acl_only_indicator ... ok
test git::tests::test_git_workflow ... ok
test meta::access_control::test::test_no_context ... ok
test meta::access_control::test::test_selinux_and_smack_context ... ok
test meta::access_control::test::test_selinux_context ... ok
test meta::access_control::test::test_smack_only_indicator ... ok
test meta::date::test::test_a_day_old_file_color ... ok
test meta::date::test::test_a_several_days_old_file_color ... ok
test meta::date::test::test_an_hour_old_file_color ... ok
test meta::date::test::test_bad_date ... ok
test meta::date::test::test_iso_format_now ... ok
test meta::date::test::test_iso_format_year_old ... ok
test meta::date::test::test_locale_format_now ... ok
test meta::date::test::test_with_relative_date ... ok
test meta::filetype::test::test_dir_type ... ok
test meta::filetype::test::test_file_type ... ok
test meta::filetype::test::test_pipe_type ... ok
test meta::filetype::test::test_socket_type ... ok
test meta::date::test::test_with_relative_date_now ... ok
test meta::filetype::test::test_symlink_type_dir ... ok
test meta::filetype::test::test_symlink_type_file ... ok
test meta::indicator::test::test_directory_indicator ... ok
test meta::indicator::test::test_executable_file_indicator ... ok
test meta::indicator::test::test_not_represented_indicator ... ok
test meta::indicator::test::test_socket_indicator ... ok
test meta::indicator::test::test_symlink_indicator ... ok
test meta::inode::tests::test_inode_no_zero ... ok
test meta::links::tests::test_hardlinks_no_zero ... ok
test meta::name::test::test_current_relative_path ... ok
test meta::name::test::test_extensions_with_file_without_extension ... ok
test meta::name::test::test_extensions_with_valid_file ... ok
test meta::name::test::test_grand_parent_relative_path ... ok
test meta::name::test::test_order_impl_is_case_insensitive ... ok
test meta::name::test::test_parent_relative_path ... ok
test meta::name::test::test_partial_eq_impl ... ok
test meta::name::test::test_partial_eq_impl_is_case_insensitive ... ok
test meta::name::test::test_partial_order_impl ... ok
test meta::name::test::test_partial_order_impl_is_case_insensitive ... ok
test meta::name::test::test_print_file_name ... ok
test meta::name::test::test_print_dir_name ... ok
test meta::name::test::test_print_hyperlink ... ok
test meta::name::test::test_print_symlink_name_dir ... ok
test meta::name::test::test_print_other_type_name ... ok
test meta::name::test::test_print_without_icon_or_color ... ok
test meta::name::test::test_print_symlink_name_file ... ok
test meta::owner::test_truncate::test_none ... ok
test meta::owner::test_truncate::test_truncated_with_marker ... ok
test meta::owner::test_truncate::test_truncated_without_marker ... ok
test meta::owner::test_truncate::test_unchanged_with_marker ... ok
test meta::owner::test_truncate::test_unchanged_without_marker ... ok
test meta::permissions::test::permission_disable ... ok
test meta::permissions::test::permission_octal ... ok
test meta::permissions::test::permission_octal2 ... ok
test meta::permissions::test::permission_octal_sticky ... ok
test meta::permissions::test::permission_rwx ... ok
test meta::permissions::test::permission_rwx2 ... ok
test meta::permissions::test::permission_rwx_sticky ... ok
test meta::size::test::render_100_plus_gigabyte ... ok
test meta::size::test::render_100_plus_kilobyte ... ok
test meta::name::test::test_special_chars_in_filename ... ok
test meta::size::test::render_100_plus_megabyte ... ok
test meta::size::test::render_100_plus_terabyte ... ok
test meta::size::test::render_10_minus_gigabyte ... ok
test meta::size::test::render_10_minus_kilobyte ... ok
test meta::size::test::render_10_minus_megabyte ... ok
test meta::size::test::render_10_minus_terabyte ... ok
test meta::size::test::render_byte ... ok
test meta::size::test::render_gigabyte ... ok
test meta::size::test::render_kilobyte ... ok
test meta::size::test::render_megabyte ... ok
test meta::size::test::render_short_nospaces ... ok
test meta::size::test::render_terabyte ... ok
test meta::size::test::render_with_a_fraction ... ok
test meta::size::test::render_with_a_truncated_fraction ... ok
test meta::symlink::tests::test_symlink_render_default_invalid_target_nocolor ... ok
test meta::symlink::tests::test_symlink_render_default_invalid_target_withcolor ... ok
test meta::tests::test_calculate_total_file_size_empty ... ok
test meta::tests::test_calculate_total_file_size_file_100b ... ok
test meta::symlink::tests::test_symlink_render_default_valid_target_nocolor ... ok
test meta::tests::test_from_path ... ok
test meta::tests::test_from_path_disable_permission ... ok
test meta::tests::test_from_path_path ... ok
test sort::tests::test_sort_assemble_sorters_by_name_unordered ... ok
test sort::tests::test_sort_assemble_sorters_by_extension ... ok
test sort::tests::test_sort_assemble_sorters_by_name_unordered_2 ... ok
test sort::tests::test_sort_assemble_sorters_by_name_with_dirs_first ... ok
test sort::tests::test_sort_assemble_sorters_by_name_with_files_first ... ok
test sort::tests::test_sort_assemble_sorters_by_version ... ok
test sort::tests::test_sort_assemble_sorters_by_time ... ok
test sort::tests::test_sort_assemble_sorters_no_sort ... ok
test sort::tests::test_sort_by_size ... ok
test theme::color::tests::test_default_theme ... ok
test theme::color::tests::test_default_theme_file ... ok
test theme::color::tests::test_empty_theme_return_default ... ok
test theme::color::tests::test_first_level_theme_return_default_but_changed ... ok
test theme::color::tests::test_hexadecimal_colors ... ok
test theme::color::tests::test_second_level_theme_return_default_but_changed ... ok
test theme::icon::tests::test_custom_icon_by_name ... ok
test theme::icon::tests::test_custom_icon_by_extension ... ok
test theme::icon::tests::test_default_icon_by_name_with_custom_entry ... ok
test theme::icon::tests::test_default_icon_by_extension_with_custom_entry ... ok
test theme::icon::tests::test_empty_theme_return_default ... ok
test theme::icon::tests::test_partial_theme_return_default ... ok
test theme::icon::tests::test_default_theme ... ok
test theme::icon::tests::test_serde_dir_from_yaml ... ok
test theme::icon::tests::test_tmp_partial_default_theme_file ... ok
test icon::test::get_icon_by_name ... ok
test icon::test::get_icon_by_extension ... ok
test flags::date::test::test_from_environment_long_iso ... ok
lsd: Not a valid date value: .

test flags::date::test::test_parsing_order_env ... ok
test flags::date::test::test_from_environment_iso ... ok
test flags::date::test::test_from_environment_none ... ok

test result: ok. 392 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.05s

     Running tests/integration.rs (target/debug/deps/integration-33a1832ca295febc)

running 45 tests
test test_custom_config_file_parsing ... ok
test test_almost_sort_with_folder ... ok
test test_date_custom_format_supports_nanos_with_length ... ok
test test_all_directory ... ok
test test_bad_utf_8_name ... ok
test test_bad_utf_8_extension ... ok
lsd: /tmp/.tmpbQYj4q/d/subdir/onemore: Permission denied (os error 13).

 d
└──  subdir
test test_cannot_access_subdir_exit_status ... ok
test test_date_custom_format_supports_padding ... ok
test test_dereference_link_broken_link ... ok
test test_dereference_link_broken_link_output ... ok
test test_list_all_populated_directory ... ok
lsd: /tmp/.tmpnOPrsU/does_not_exist: No such file or directory (os error 2).

test test_list_all_empty_directory ... ok
test test_cannot_access_file_exit_status ... ok
test test_list_almost_all_populated_directory ... ok
test test_dereference_link_right_type_and_no_link ... ok
test test_list_block_inode_populated_directory_with_long ... ok
test test_list_empty_directory ... ok
test test_list_block_inode_populated_directory_without_long ... ok
test test_list_populated_directory ... ok
test test_list_inode_with_long_ok ... ok
test test_lower_case_name_icon_match ... ok
test test_lower_case_ext_icon_match ... ok
test test_list_inode_populated_directory ... ok
test test_multiple_files ... ok
test test_no_show_folder_content_of_symlink_for_long ... ok
test test_runs_okay ... ok
test test_show_folder_content_of_symlink ... ok
test test_list_almost_all_empty_directory ... ok
test test_show_folder_content_of_symlink_for_long_tail_slash ... ok
test test_list_broken_link_ok ... ok
test test_show_folder_of_symlink_for_long_multi ... ok
test test_tree ... ok
test test_symlink_on_long ... ok
test test_tree_all_not_show_self ... ok
test test_tree_d ... ok
test test_nosymlink_on_non_long ... ok
test test_tree_show_edge_before_name ... ok
test test_tree_dereference ... ok
test test_tree_no_dereference ... ok
test test_upper_case_ext_icon_match ... ok
test test_version_sort_overwrite_by_sizesort ... ok
test test_truncate_owner ... ok
test test_upper_case_name_icon_match ... ok
test test_version_sort_overwrite_by_timesort ... ok
test test_version_sort ... ok

test result: ok. 45 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s
```
All 437 tests pass (392 unit + 45 integration).
