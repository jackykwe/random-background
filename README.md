# random-background

A utility program I use to randomly select a background from a collection of images. This README also outlines how to setup `systemd` so that this program runs daily.

## Operation Overview

This program takes in the path to the directory of background images, then creates a working directory named "Working" inside it.

- The current background will be generated in memory, saved to this folder, then set as the desktop background.
- A `config.toml` file is also generated in this folder. The signature is as follows:

  ```toml
  [general]
  ttf_font_path = '/path/to/font.ttf'

  # [countdown]
  # term_start = <YYYY-MM-DD>
  # term_last_lecture = <YYYY-MM-DD>
  # first_paper = <YYYY-MM-DD>
  # last_paper_end_time = <YYYY-MM-DD>T<HH:MM:SS>

  # [overlay]
  # text = ''
  ```

  - `ttf_font_path` specifies the path to the font file used to draw text onto the generated background.
  - The `[countdown]` section either doesn't exist entirely, or exists as a collective whole with all parts present. When present, a days countdown will be drawn in the bottom left corner of the generated background.
  - The `[overlay]` section either doesn't exist entirely, or exists as a collective with all part present. When present, a large stencil overlay containing the `text` will be drawn over the generated background.

  Error checking is built into the program; if things don't work you'll be directed on how to fix them via error messages.

## How to Use (NixOS)
Add these lines to your system Nix config's `flake.nix` under the inputs section:
```nix
inputs = {
  # ...
  random-background = {
    url = "github:jackykwe/random-background";
    inputs.nixpkgs.follows = "nixpkgs";  # optional
  };
};
```
Then add this `home.nix`:
```nix
home.packages = [
  #...
  inputs.random-background.defaultPackage.x86_64-linux
];
```

Full usage method derived from [this discussion](https://www.reddit.com/r/NixOS/comments/1bxa6dc/noob_question_how_to_install_software_from_github/).

## How to Use (all other OSes)

While in this directory, `cargo run --release -- --dir <DIR>`, where `<DIR>` is the directory of background images.

The compiled program is an executable that has the following help message. This is obtained via `cargo run --release -- --help`.

```
Usage: random-background --dir <DIR>

Options:
  -d, --dir <DIR>  Path to directory containing the images
  -h, --help       Print help
  -V, --version    Print version
```

> [!IMPORTANT]
> The first run of the program will fail, as designed. You need to specify `ttf_font_path` in `<DIR>/Working/config.toml`, which is generated after the first run. Then re-run the program.

## `systemd` setup

0. Run the program manually as above (["How to Use"](#how-to-use)) first. Proceed with the following steps only after witnessing the program finish without error.

   > [!IMPORTANT]
   > The first run of the program will fail, as designed. You need to specify `ttf_font_path` in `<DIR>/Working/config.toml`, which is generated after the first run. Then re-run the program.

1. Copy `random-background.fish.example` to `random-background.fish` (at any location of your choice ($*$)). Specify the path to this repository, and also the directory of background images.

   N.B. This directory should contain only images files (e.g. png, jpg, webp, gif, etc.) and directories.

   ```bash
   # Example bash/fish commands
   cp random-background.fish.example random-background.fish
   # Then open editor of your choice to edit random-background.fish
   ```

   N.B. The `.fish` file may be renamed to the `.sh` format if you wish. This script file contains no fish-specific syntax, so this operation is okay.

2. Copy `random-background.service.example` into `~/.config/systemd/user/`. Specify the path to the `random-background.fish` file you created in the previous step ($*$).

   ```bash
   # Example bash/fish commands
   cp random-background.service.example ~/.config/systemd/user/random-background.service
   # Then open editor of your choice to edit random-background.service
   ```

3. Copy `random-background.timer` into `~/.config/systemd/user/`.

   ```bash
   # Example bash/fish commands
   cp random-background.timer ~/.config/systemd/user/random-background.timer
   ```

4. Run `systemctl --user enable --now random-background.timer`. You're done.

## `systemd` tear-down

To remove the `systemd` timer (responsible for making the program run once daily):

1. Run `systemctl --user disable --now random-background.timer`.
2. Run `rm ~/.config/systemd/user/random-background.service`.
3. Run `rm ~/.config/systemd/user/random-background.timer`. You're done.

## Arch Linux and `systemd` References

- [Location to place `.timer` and `.service` files](https://wiki.archlinux.org/title/Systemd/User#How_it_works)\
  `.timer` and `.service` files must be copied into the `~/.config/systemd/user/` before running `systemctl` commands upon them. Otherwise, `systemctl` fails.
- [Meaning of `systemctl` commands](https://wiki.archlinux.org/title/Systemd#Using_units)\
  In particular, these are useful for us:
  - `systemctl enable --now <UNIT>.timer` to enable (run on startup) and start (immediately) the **timer**.
  - `systemctl disable --now <UNIT>.timer` for the inverse operation.
- [View all active timers](https://wiki.archlinux.org/title/Systemd/Timers#Management)\
  Use `systemctl --user list-timers [--all]` to troubleshoot whether the timer is running or not.
- [Linking a `.timer` file to a `.service` file](https://wiki.archlinux.org/title/Systemd/Timers#Manually)\
  Use the `Unit=%i.service` setting under the `[timer]` section in the `.timer` file.
