# rofi-zed

![Demo](./docs/demo.mp4)

Quickly open recent Zed Editor projects with Rofi.

## Usage

### As a Rofi mode

```sh
rofi -show zed-recent -modi zed-recent
```

### With [combi mode](https://davatorium.github.io/rofi/current/rofi.1/#combi)

```sh
rofi -show combi -combi-modi drun,zed-recent,ssh
```

## Credits

- [Zed Recent Projects Raycast extension](https://www.raycast.com/ewgenius/zed-recent-projects)
- [Zed recents extension (Vicinae)](https://github.com/vicinaehq/extensions/tree/main/extensions/zed-recents)
- [VSCode mode for Rofi](https://github.com/fuljo/rofi-vscode-mode)
