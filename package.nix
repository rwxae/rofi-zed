{
  cairo,
  glib,
  pango,
  pkg-config,
  rustPlatform,
  sqlite,
}:

rustPlatform.buildRustPackage {
  pname = "rofi-zed";
  version = "0.1.0";

  src = ./.;

  cargoHash = "sha256-MBZA6asr8VgxJCn9GnKG9Pgc8yIzJXa54Vfel/3Tdcc=";

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    glib
    cairo
    pango
    sqlite
  ];

  postInstall = ''
    mkdir -p $out/lib/rofi
    mv $out/lib/lib*.so $out/lib/rofi/
  '';

  meta = {
    description = "A Rofi plugin for Zed";
    platforms = [
      "x86_64-linux"
      "aarch64-linux"
    ];
  };
}
