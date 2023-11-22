{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    llm-git = {
      url = "github:rustformers/llm";
      inputs.nixpkgs.follows = "nixpkgs";      
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, llm-git }:  
    flake-utils.lib.eachDefaultSystem 
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
            config.allowUnfree = true;
          };
          stdenv = pkgs.clangStdenv;
          rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
       in
        with pkgs;
        {
          llm = llm-git.packages."${system}".llm;

          environment.systemPackages = with pkgs; [
              llm
            ];

          modules = [
            nix-ld.nixosModules.nix-ld
            { programs.nix-ld.dev.enable = true; }
          ];

          hardware.pulseaudio.enable = true;
          hardware.pulseaudio.package = pulseaudioFull;
          hardware.opengl.extraPackages = [ mesa.drivers ];

          services.xserver.enable = true;

          fonts.packages = with pkgs; [
            noto-fonts
            noto-fonts-cjk
            noto-fonts-emoji
            liberation_ttf
            fira-code
            fira-code-symbols
            mplus-outline-fonts.githubRelease
            dina-font
            proggyfonts
          ];

          devShells.default = mkShell {
            buildInputs = [
              rustToolchain
              cargo-edit
              cargo-expand
              cargo-udeps
              cargo-whatfeatures
              cargo-leptos
              cargo-generate
              cargo-make
              cacert
              trunk
              direnv
              lld
              clang
              gcc
              zsh
              git
              just
              starship
              openssl
              openssl.dev
              pkg-config
              zlib.dev
              alsa-lib
              xorg.libX11
              xorg.libXi
              xorg.libXcursor
              libpulseaudio
              libGL
              libglvnd
              libiconv
              tailwindcss
              sass
            ];
            
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
            # see: https://discourse.nixos.org/t/running-a-rust-application-that-needs-egl-with-shell-nix/33245/3
            LD_LIBRARY_PATH="${pkgs.libglvnd}/lib:/home/steele/git/cimlabs/drover/crates/ggml/sys/llama.cpp";

          shellHook = ''
            if [ -f .env ]; then
              export $(grep -v '^#' .env | xargs)
            fi
            export GIT_CONFIG_NOSYSTEM=1
            ZSH_CUSTOM=$HOME/.config/zsh
            export PATH="$HOME/.cargo/bin:$PATH"
            export LD_LIBRARY_PATH="${pkgs.libglvnd}/lib:/home/steele/git/cimlabs/drover/crates/ggml/sys/llama.cpp";
            '';
        };
        }
      );
}
