{
  inputs,

  # Dependencies
  main,
  mdbook,
  stdenv,
}:

stdenv.mkDerivation {
  inherit (main) pname version;

  src = inputs.nix-filter {
    root = inputs.self;
    include = [
      "book.toml"
      "matron-server-example.toml"
      "CODE_OF_CONDUCT.md"
      "CONTRIBUTING.md"
      "README.md"
      "development.md"
      "debian/matron-server.service"
      "debian/README.md"
      "arch/matron-server.service"
      "rpm/matron-server.service"
      "rpm/README.md"
      "docs"
      "theme"
    ];
  };

  nativeBuildInputs = [
    mdbook
  ];

  buildPhase = ''
    mdbook build -d $out
  '';
}
