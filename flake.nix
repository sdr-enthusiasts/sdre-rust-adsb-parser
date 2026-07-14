# in flake.nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    precommit.url = "github:FredSystems/pre-commit-checks";
  };

  outputs = { self, nixpkgs, flake-utils, precommit }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
          };
          chk = precommit.lib.mkCheck {
            inherit system;
            src = ./.;

            # ── Feature toggles ─────────────────────────────
            check_rust = true;
            check_docker = false;
            check_python = false;

            # Rust-specific knobs (safe to leave here)
            enableXtask = false;

            # Python-specific knobs (safe to leave here)
            python = {
              enableBlack = true;
              enableFlake8 = true;
            };
          };
          extraDev = chk.passthru.devPackages or [ ];
          corePkgs = chk.enabledPackages or [ ];
        in
        with pkgs;
        {
          checks.pre-commit = chk;

          devShells.default = mkShell {
            # 👇 and now we can just inherit them
            buildInputs = extraDev ++ corePkgs;

            shellHook = ''
              ${chk.shellHook}

              alias pre-commit="pre-commit run --all-files"
            '';
          };
        }
      );
}

# https://www.reddit.com/r/rust/comments/mmbfnj/nixifying_a_rust_project/
