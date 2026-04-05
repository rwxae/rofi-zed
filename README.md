# rofi-zed

https://github.com/user-attachments/assets/a2220191-22c7-48c9-b7e3-5e62b13d0ecd

Quickly open recent Zed Editor projects with Rofi.

## Install

Add `rofi-zed` input to your flake:

```nix
{
  inputs = {
    rofi-zed.url = "github:rwxae/rofi-zed";
    rofi-zed.inputs.nixpkgs.follows = "nixpkgs";
  };
}
```

Install using [Home Manager](https://github.com/nix-community/home-manager):

```nix
{ inputs, ... }:

{
  programs.rofi = {
    plugins = [
      inputs.rofi-zed.packages.${pkgs.stdenv.system}.default
    ];
    modes = "zed-recent";
  };
}
```

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
