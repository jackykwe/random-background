# Adapted from the template from https://github.com/NixOS/templates/blob/master/rust/flake.nix
{
  description = "Random Background flake";
  inputs = {
    # ! Note that Naersk by default ignores the rust-toolchain file, using whatever Rust compiler version is present in nixpkgs.
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";
    naersk = {
      url = "github:nix-community/naersk/master"; # Nix library for building Rust projects
      inputs.nixpkgs.follows = "nixpkgs"; # avoid downloading two different versions of nixpkgs
    };
    utils.url = "github:numtide/flake-utils"; # to simplify Flake writing
    # nix-vscode-extensions = {
    #   url = "github:nix-community/nix-vscode-extensions";
    #   inputs.nixpkgs.follows = "nixpkgs";
    # };
    # pkgs = import (fetchTarball("https://github.com/NixOS/nixpkgs/archive/a58a0b5098f0c2a389ee70eb69422a052982d990.tar.gz")) {};
  };
  outputs = {
    self, # directory of this flake in the Nix store (see https://nixos.wiki/wiki/Flakes#Output_schema)
    nixpkgs,
    naersk,
    utils,
    # nix-vscode-extensions
  }:
    utils.lib.eachSystem [utils.lib.system.x86_64-linux] (
      system:
      # devShells.${system}.default = pkgs.mkShell {
      let
        pkgs = import nixpkgs {
          inherit system;
          # config.allowUnfree = true;  # Courtesy of https://discourse.nixos.org/t/allow-unfree-in-flakes/29904/2 (see also https://nixos.wiki/wiki/Unfree_Software)
          # overlays = [
          #   nix-vscode-extensions.overlays.default  # provides pkgs.vscode-marketplace
          # ];
        };
        naersk-lib = pkgs.callPackage naersk {};
        # ? pkgs.callPackage is like an import, but additionally passes all the arguments of naersk
        # ? to it automatically if it already exists in scope (scope refers to here in the let-binding
        # ? (i.e. pkgs and naersk-lib), and finally overrides are specified as {}.
        # ?
        # ? Courtesy of https://nixos.org/guides/nix-pills/13-callpackage-design-pattern#using-callpackage-to-simplify-the-repository
        # ? NB. Understanding pill 13 requires pre-requisite knowledge from pills 1--12.
        # * TL;DR: callPackage is an import on steroids.
      in {
        # For `nix build` and `nix run`
        defaultPackage = naersk-lib.buildPackage ./.; # ./. is the "src" attribute

        # For `nix develop` (optional, can be skipped)
        devShell = with pkgs;
          mkShell {
            # These are dependencies that should only exist in the build environment (tools you need to build)
            nativeBuildInputs = [
              cargo
              rustc
              bacon
              # (vscode-with-extensions.override {
              #   # Syntax from documentation in https://github.com/NixOS/nixpkgs/blob/master/pkgs/applications/editors/vscode/with-extensions.nix
              #   vscodeExtensions = with pkgs.vscode-marketplace; [
              #     rust-lang.rust-analyzer
              #   ];
              # })
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
      }
    );
}
