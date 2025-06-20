{
  description = "Rofi plugin development environment";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    {
      self,
      nixpkgs,
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          pkg-config
          glib
          pango
          (pkgs.writeShellScriptBin "rofi-zed" ''
            rofi -show combi -combi-modi zed,drun -display-zed "Project"
          '')
        ];
        shellHook = ''
          export ROFI_PLUGIN_PATH="$(pwd)/target/debug"
        '';
      };
    };
}
