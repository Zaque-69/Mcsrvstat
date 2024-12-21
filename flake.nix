{
  description = "Mcsrvstat Rust project flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import nixpkgs { inherit system; };
    rustPlatform = pkgs.rustPlatform;

  in {
    packages.default = rustPlatform.buildRustPackage {
        pname = "mcsrvstat";
        version = "unstable-2024-11-27";

        src = pkgs.fetchFromGitHub {
            owner = "Zaque-69";
            repo = "Mcsrvstat";
            rev = "56ba8402b8ed1ec8ea27af9bff85850d73720793";
            hash = "sha256-4a6m+FbPwn2QpD+tgD8BI9Y8m8R8WHppFzVMYsUVo9o=";
        };

        cargoHash = "sha256-MkwhOFnA0U0pRT7Elv1KMt9KrxUfhpnuojeV+EhNtXU=";

        nativeBuildInputs = with pkgs; [
            pkg-config
        ];

        buildInputs = with pkgs; [
            openssl
        ] ++ lib.optionals stdenv.isDarwin [
            darwin.apple_sdk.frameworks.Security
            darwin.apple_sdk.frameworks.SystemConfiguration
        ];

        meta = {
            description = "";
            homepage = "https://github.com/Zaque-69/Mcsrvstat";
            license = pkgs.lib.licenses.mit;
            maintainers = with pkgs.lib.maintainers; [ ];
            mainProgram = "mcsrvstat";
        };
    };

    devShell = pkgs.mkShell {
      buildInputs = [ rustPlatform.rustc rustPlatform.cargo ];
    };
  });
}
