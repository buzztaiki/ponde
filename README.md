# Ponde

Pointing device configurationd daemon for Wayland and X11 using libinput and uinput.

It can do the same configiration as [https://gitlab.freedesktop.org/xorg/driver/xf86-input-libinput](xf86-input-libinput).

## Motivation

In Wayland, xf86-input-libinput is not available, and the pointing device settings are different for each compositor.
Gnome Wayland allows the same settings as xf86-input-libinput, but does not allow mapping of buttons, nor does it allow per-device settings.

I wrote this program to solve these problem.


## Install

```console
$ cargo install ponde
```

## Usage

```console
$ sudo ponde /path/to/config.yaml
```


## Configuration

Example configuration is here:

```config.yaml
devices:
  - match_rule:
      name: Kensington Expert Wireless TB Mouse
    accel_profile: flat
    scroll_button: BTN_SIDE
    scroll_button_lock: true
  - match_rule:
      name: "- GameBall"
    accel_profile: flat
    scroll_button: BTN_MIDDLE
    scroll_button_lock: false
  - match_rule:
      name: Logitech M570
    accel_profile: flat
    scroll_button: BTN_EXTRA
    scroll_button_lock: false
    button_mapping:
      BTN_SIDE: BTN_MIDDLE
```


## Configuration properties

- `match_rule`
  - `name`: Specifies device name to match.
- `accel_profile`: Sets the pointer acceleration profile to the given profile. Permitted values are `adaptive`, `flat`.  Not all devices support this option or all profiles. If a profile is unsupported, the default profile for this device is used. For a description on the profiles and their behavior, see the libinput documentation.
- `accel_speed`: Sets the pointer acceleration speed within the range [-1, 1]
- `button_mapping`: Sets the logical button mapping for this device.
- `left_handed`: Enables left-handed button orientation, i.e. swapping left and right buttons.
- `middle_emulation`: Enables middle button emulation. When enabled, pressing the left and right buttons simultaneously produces a middle mouse button click.
- `natural_scrolling`: Enables or disables natural scrolling behavior.
- `rotation_angle`: Sets the rotation angle of the device to the given angle, in degrees clockwise. The angle must be between 0.0 (inclusive) and 360.0 (exclusive).
- `scroll_button`: Designates a button as scroll button. If the button is logically down, x/y axis movement is converted into scroll events.
- `scroll_button_lock`: Enables or disables the scroll button lock. If enabled, the `scroll_button` is considered logically down after the first click and remains down until the second click of that button. If disabled (the default), the `scroll_button` is considered logically down while held down and up once physically released.


## Avoid conflicts with Wayland compositor and X11 settings

The events output by ponde to uinput become input events for the Wayland compositor and X11. In other words, they are affected by these settings.
Especially affected is `accel_profile`, which libinput sets to adaptive by default, so if nothing is set, double acceleration will occur.

You can avoid this problem by setting the following:

For gnome:
```console
$ gsettings set org.gnome.desktop.peripherals.mouse accel-profile flat
```

For X11:

```conf
# /etc/X11/xorg.conf.d/90-ponde.conf
Section "InputClass"
  Identifier "Ponde"
  MatchProduct "ponde"
  Driver "libinput"
  Option "AccelProfile" "flat"
EndSection
```

## Limitation

Trackpad cannot be configured. This is intentionally disallowed because libinput converts multiple finger swipes into gesture events and the original input cannot be restored.

## License

MIT
