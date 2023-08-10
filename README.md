# kde-random-background

# Arch Linux Resources

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
