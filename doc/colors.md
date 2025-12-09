# Color reference
> [!TIP]
> Checkout [trapd00r/LS_COLORS](https://github.com/trapd00r/LS_COLORS) and [sharkdp/vivid](https://github.com/sharkdp/vivid) for help in theming using `LS_COLORS`.

You can customize filetype colors using `LS_COLORS` and other colors using the theme.

## Default colors
### Users and groups
| Color                                                                | Hex code  | RGB array         | Used for |
| :------------------------------------------------------------------- | :-------- | :---------------- | :------- |
| <img valign='middle' alt='very pale yellow' src='./img/ffffd7.svg'/> | `#ffffd7` | `[255, 255, 215]` | User     |
| <img valign='middle' alt='grayish yellow' src='./img/d7d7af.svg'/>   | `#d7d7af` | `[215, 215, 175]` | Group    |

### Permissions
| Color                                                                             | Hex code  | RGB array         | Used for                          |
| :-------------------------------------------------------------------------------- | :-------- | :---------------- | :-------------------------------- |
| <img valign='middle' alt='strong lime green' src='./img/00d700.svg'/>             | `#00d700` | `[0, 215, 0]`     | Read permission                   |
| <img valign='middle' alt='very light green' src='./img/d7ff87.svg'/>              | `#d7ff87` | `[215, 255, 135]` | Write permission                  |
| <img valign='middle' alt='dark red' src='./img/af0000.svg'/>                      | `#af0000` | `[175, 0, 1]`     | Execute permission                |
| <img valign='middle' alt='pure (or mostly pure) magenta' src='./img/ff00ff.svg'/> | `#ff00ff` | `[255, 0, 255]`   | Execute permission with stickybit |
| <img valign='middle' alt='moderate pink' src='./img/d75f87.svg'/>                 | `#d75f87` | `[215, 95, 135]`  | No Access                         |

### File Types
> [!Note]
> These change based on your configured terminal color scheme

| Color                                                                           | Hex code  | RGB array         | Used for                                |
| :------------------------------------------------------------------------------ | :-------- | :---------------- | :-------------------------------------- |
| <img valign='middle' alt='pure (or mostly pure) blue.' src='./img/0087ff.svg'/> | `#0087ff` | `[0, 135, 255]`   | Directory                               |
| <img valign='middle' alt='strong lime green' src='./img/00d700.svg'/>           | `#00d700` | `[0, 215, 0]`     | Executable file                         |
| <img valign='middle' alt='white' src='./img/ffffff.svg'/>                       | `#ffffff` | `[255, 255, 255]` | Non-executable file                     |
| <img valign='middle' alt='dark red' src='./img/af0000.svg'/>                    | `#af0000` | `[175, 0, 1]`     | Broken symlink                          |
| <img valign='middle' alt='strong cyan' src='./img/00d7d7.svg'/>                 | `#00d7d7` | `[0, 215, 215]`   | Pipe/Symlink/Blockdevice/Socket/Special |
| <img valign='middle' alt='strong orange' src='./img/d78700.svg'/>               | `#d78700` | `[215, 135, 0]`   | CharDevice                              |


### Dates
| Color                                                                        | Hex code  | RGB array       | Used for             |
| :--------------------------------------------------------------------------- | :-------- | :-------------- | :------------------- |
| <img valign='middle' alt='strong lime green' src='./img/00d700.svg'/>        | `#00d700` | `[0, 215, 0]`.  | Within the last hour |
| <img valign='middle' alt='strong cyan - lime green' src='./img/00d787.svg'/> | `#00d787` | `[215, 135, 1]` | Within the last day  |
| <img valign='middle' alt='dark cyan' src='./img/00af87.svg'/>                | `#00af87` | `[175, 135, 1]` | Older                |

### File Sizes
| Color                                                                 | Hex code  | RGB array         | Used for    |
| :-------------------------------------------------------------------- | :-------- | :---------------- | :---------- |
| <img valign='middle' alt='pale yellow' src='./img/ffffaf.svg'/>       | `#ffffaf` | `[255, 255, 175]` | Small file  |
| <img valign='middle' alt='very light orange' src='./img/ffaf87.svg'/> | `#ffaf87` | `[215, 135, 1]`   | Medium file |
| <img valign='middle' alt='strong orange' src='./img/d78700.svg'/>     | `#d78700` | `[215, 135, 0]`   | Large file  |
| <img valign='middle' alt='white' src='./img/ffffff.svg'/>             | `#ffffff` | `[255, 255, 255]` | Non file    |
