# random-background

A utility program I use to randomly select a background from a collection of images. This README also outlines how to setup `systemd` so that this program runs daily.

## How to Use

While in this directory, `cargo run --release -- --dir <DIR>`, where `<DIR>` is the directory of background images.

The compiled program is an executable that has the following help message. This is obtained via `cargo run --release -- --help`.

```
Usage: random-background --dir <DIR>

Options:
  -d, --dir <DIR>  Path to directory containing the images
  -h, --help       Print help
  -V, --version    Print version
```

## `systemd` setup

1. Copy `random-background.fish.example` to `random-background.fish` (at any location of your choice ($*$)). Specify the directory of background images.

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
4. Run `systemctl --user enable --now random-background.timer`. You're done.

## `systemd` teardown

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
