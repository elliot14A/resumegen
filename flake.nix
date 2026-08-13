{
  description = "resumegen: Agent-First ATS Resume & Cover Letter Toolchain";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "resumegen";
          version = "0.1.0";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          nativeBuildInputs = with pkgs; [ pkg-config fontconfig openssl ];
          buildInputs = with pkgs; [ openssl fontconfig ];
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            clippy
            rustfmt
            tectonic
            poppler-utils
            python3
            python3Packages.pyyaml
            pkg-config
            fontconfig
            openssl
          ];

          shellHook = ''
            export PATH="$PWD/.agents/skills/resume-cover-letter-generator/scripts:$PATH"
            # Ensure release binary is deployed to the Agent Skill scripts folder
            if [ -f Cargo.toml ] && [ ! -f .agents/skills/resume-cover-letter-generator/scripts/resumegen ]; then
              cargo build --release --quiet && cp target/release/resumegen .agents/skills/resume-cover-letter-generator/scripts/
              chmod +x .agents/skills/resume-cover-letter-generator/scripts/resumegen
            fi
            echo "resumegen agent dev environment ready"
            echo "   CLI: resumegen $(resumegen --version 2>/dev/null || true)"
            echo "   Tectonic: $(tectonic --version 2>&1 | head -n1)"
          '';
        };
      }
    );
}
