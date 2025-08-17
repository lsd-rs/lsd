# UPDATE Tracking
This document tracks the updates I've been doing to lsd 1.1.5. Updates are organized by date, and go from newest to oldest.

## _August 2025_

This is how 1.1.5 builds with `cargo build` and the beginning messages that it produces (see below).

![image](/assets/CargoBuildMessages-2025-08-17.png)

_Built with Rust 1.89 under Ubuntu 25.04 on a Raspberry Pi 5_

Based on the messages output above, I've performed the following actions:
- Stepped lsd's version to 1.2.0
- Stepped up to a more current Rust release:
  * Updated Rust edition to 2024
  * Updated minimum Rust to version 1.85
- Then, from the `cargo build` output:
  * Updated clap to version 4.5.*
  * Updated clap_complete to version 4.5.*
  * Updated crossterm to version 0.29.0
  * Updated dirs to version 6
  * Updated git2 to version 0.20
  * Updated lscolors to version 0.20.0
  * Updated terminal_size to version 0.4.*
  * Updated thiserror to version 2.0
  * Updated unicode-width to version 0.2
- Finally, from `Cargo.toml`:
  * Updated chrono to version 0.4
  * Updated once_cell to version 1.21
  * updated wild to version 2.2

Everything else didn't change.

### Build errors with term_grid version 0.2

When attempting to step up to `term_grid` version 0.2, the following errors were produced:
```text
...
error[E0063]: missing field `alignment` in initializer of `term_grid::Cell`
   --> src/display.rs:137:24
    |
137 |             cells.push(Cell {
    |                        ^^^^ missing `alignment`

error[E0063]: missing field `alignment` in initializer of `term_grid::Cell`
   --> src/display.rs:224:18
    |
224 |         grid.add(Cell {
    |                  ^^^^ missing `alignment`

error[E0063]: missing field `alignment` in initializer of `term_grid::Cell`
   --> src/display.rs:269:24
    |
269 |             cells.push(Cell {
    |                        ^^^^ missing `alignment`

For more information about this error, try `rustc --explain E0063`.
error: could not compile `lsd` (bin "lsd") due to 3 previous errors
```
As a consequence `term_grid` is left at version 0.1
### Build errors with xdg version 3.0
When attempting to steup up to `xdg` version 3.0, the following errors were produced:
```text
...
   Compiling git2 v0.18.3
error[E0599]: no method named `ok` found for struct `BaseDirectories` in the current scope
   --> src/config_file.rs:184:18
    |
183 | /             BaseDirectories::with_prefix("")
184 | |                 .ok()
    | |                 -^^ method not found in `BaseDirectories`
    | |_________________|
    |

For more information about this error, try `rustc --explain E0599`.
error: could not compile `lsd` (bin "lsd") due to 1 previous error
```
As a consequence `xdg` is left at version 2.5
### Conclusion and Way Forward
- This has been an exercise in initially bringing `lsd` closer to current Rust and crate releases. So far the version of `lsd` with these updates seems to work as before, but the reader should realize my testing is limited at best.
- A way needs to be found to eliminate dependence upon the `term-grid` crate and to incorporate the needed functionality within `lsd` in order to stay up with Rust's evolving standards.
- The API has changed within `xdg` from version 2.5 to version 3.0. If the code depending upon the version 2.5 API is within `lsd` then it should be updated to use the version 3.0 API.
