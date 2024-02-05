# Filtile

This is a layout manager for the [River](https://github.com/riverwm/river) window
manager. It's basically `rivertile`, but with a few things not implemented (because
I don't use them), and configuration per tag.

## Usage

All numbers will set the value, but also support a prefix of either `+` or `-`
for adjustment.

Following are the commands that can be sent to `riverctl send-layout-cmd filtile ...`:

<dl>
    <dt>view-padding [pixels]</dt>
    <dd>Set the padding around views in pixels.</dd>
    <dt>outer-padding [pixels]</dt>
    <dd>Set the padding around the edge of the layout area in pixels.</dd>
    <dt>main-ratio [percent]</dt>
    <dd>Set the ratio of the main area to total layout area, in percent. The
        ratio must be between 10 and 90, inclusive.</dd>
    <dt>swap<dt>
    <dd>Swap the main area to the other side of the layout.</dd>
    <dt>main-location [left | top | right | bottom]<dt>
    <dd>Set the location of the main area in the layout. </dd>
    <dt>pad</dt>
    <dd>Toggle single view padding. When only one view is visible, it
        will be centered and given as much width/height as it would have if
        there were more windows. Also supports sending "on" or "off" to not
        toggle.</dd>
    <dt>monocle</dt>
    <dd>Toggle the "monocle" layout. Also supports sending "on" or "off" to not
        toggle.</dd>
</dl>

All commands can be prefaced with one or both of the following options. Either
can be "all". Both set to "all" changes the default. 

<dl>
    <dt>--output</dt>
    <dd>The output (monitor) to apply this setting to.</dd>
    <dt>--tags</dt>
    <dd>The tags to apply this setting to.</dd>
</dl>

Commands can also be sent to the executable on startup, separated by commas,
as shown below.

## Examples

```bash
# Super+H and Super+L to decrease/increase the main ratio of filtile
riverctl map normal Super H send-layout-cmd filtile "main-ratio -5"
riverctl map normal Super L send-layout-cmd filtile "main-ratio +5"

riverctl map normal Super Z send-layout-cmd filtile "swap"
riverctl map normal Super C send-layout-cmd filtile "pad"

# Set the default layout generator to be filtile and start it.
# River will send the process group of the init executable SIGTERM on exit.
riverctl default-layout filtile

filtile pad on, \
    --tags $((1 << 4)) main-ratio 70, \
    --tags $((1 << 4)) main-location right &
```

## Installation

You can install from source by cloning the repo and running:

    cargo install

Or, if you run NixOS, you can do something like the following:

```nix
{
  inputs = {
    filtile.url = "github:pkulak/filtile";
  };
  outputs =
    inputs@{ self
    , nixpkgs-unstable
    , ...
    }:
    let
      overlays = {
        unstable = _: prev: {
          unstable = import nixpkgs-unstable
            {
              inherit (prev.stdenv) system;
            } // {
            filtile = inputs.filtile.packages.${prev.stdenv.system}.filtile;
          };
        };
      };
    in
    {
      <snip>;
      packages = with pkgs; [
        unstable.filtile
      ];
    }
}
```
