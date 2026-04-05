{
  description = "Rofi plugin development environment";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      eachSystem =
        fn:
        nixpkgs.lib.genAttrs [
          "x86_64-linux"
          "aarch64-linux"
        ] (system: fn (import nixpkgs { inherit system; }));
    in
    {
      devShells = eachSystem (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            rustc
            rustfmt
            rust-analyzer
            cargo
            pkg-config
            glib
            pango
            rofi
            sqlite
            (pkgs.writeShellScriptBin "rofi-zed" ''
              rofi -show combi -combi-modi window,zed,drun
            '')
          ];
          shellHook = ''
            export ROFI_PLUGIN_PATH="$(pwd)/target/debug"
          '';
        };
      });
    };
}
