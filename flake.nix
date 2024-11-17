{
  inputs = {
    utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, utils }: utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
      lsd = pkgs.rustPlatform.buildRustPackage {
        pname = "lsd";
        version = "1.1.5";

        src = ./.;

        cargoHash = "sha256-chryC4YDvd8c7fIiHMWi+g5JYZJqkLPknSCgzYVKucE=";
        doCheck = false;

        nativeBuildInputs = [ pkgs.git ];

        #meta = with pkgs.lib; {
        #  description = "A fast line-oriented regex search tool, similar to ag and ack";
        #  homepage = "https://github.com/BurntSushi/ripgrep";
        #  license = licenses.unlicense;
        #  maintainers = [];
        #};
      };
    in {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          cargo
          rustc
          rust-analyzer
          rustfmt
        ];
      };
      packages.default = lsd;
    }
  );
}
