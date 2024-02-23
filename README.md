# Filtile

This is a layout manager for the [River](https://github.com/riverwm/river) window
manager. It's (nearly) a drop-in replacement for `rivertile`, but with a few things
added and configuration per tag/output.

## Usage

All numbers will set the value, but also support a prefix of either `+` or `-`
for adjustment.

Following are the commands that can be sent to `riverctl send-layout-cmd filtile ...`:

<dl>
    <dt>view-padding [pixels]</dt>
    <dd>Set the padding around views in pixels.</dd>
    <dt>outer-padding [pixels]</dt>
    <dd>Set the padding around the edge of the layout area in pixels.</dd>
    <dt>main-location [left | top | right | bottom]<dt>
    <dd>Set the location of the main area in the layout.</dd>
    <dt>main-count [count]<dt>
    <dd>Set the number of views in the main area of the layout.</dd>
    <dt>main-ratio [percent]</dt>
    <dd>Set the ratio of the main area to total layout area, in percent. The
        ratio must be between 10 and 90, inclusive.</dd>
    <dt>flip<dt>
    <dd>Flip the main area to the other side of the layout.</dd>
    <dt>pad</dt>
    <dd>Toggle single stack padding. When only one stack is visible, it
        will be centered and given as much width/height as it would have if
        there were more windows. Also supports sending "on" or "off" to not
        toggle.</dd>
    <dt>monocle</dt>
    <dd>Toggle the "monocle" layout. Also supports sending "on" or "off" to not
        toggle.</dd>
    <dt>smart-padding [pixels]</dt>
    <dd>The padding to apply when there is only one window (and no monocle).</dd>
    <dt>smart-padding off</dt>
    <dd>Turn off smart padding.</dd>
    <dt>smart-padding-h [pixels]</dt>
    <dd>The horizontal (left and right) padding to apply when there is only one
        window (and no monocle).</dd>
    <dt>smart-padding-v [pixels]</dt>
    <dd>The vertical (top and bottom) padding to apply when there is only one
        window (and no monocle).</dd>
    <dt>move-split-[up|down|left|right] [percent]</dt>
    <dd>A different way to think about the main ratio. "move-split-right", for
        example, will make the main-ratio larger when the main-location is
        left, smaller when it's right, and is a no op for top and bottom.</dd>
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
riverctl map normal Super LEFT send-layout-cmd filtile "move-split-left 5"
riverctl map normal Super RIGHT send-layout-cmd filtile "move-split-right 5"

riverctl map normal Super Z send-layout-cmd filtile "flip"
riverctl map normal Super C send-layout-cmd filtile "pad"

# Set the default layout generator to be filtile and start it.
riverctl default-layout filtile

# - Smart gaps on the sides of every tag (on the larger monitor), to keep
#   single windows from being gigantic.
#
# - A scratch pad on tag 7 with pad (to keep the first window from resizing
#   a bunch) and giant gaps for bling. 
#
# - Tag 1 usually has a browser, which is usually easier to read when it's on
#   the right.
filtile \
    --output HDMI-A-1 smart-padding-h 384, \
    --tags $((1 << 6)) pad on, \
    --tags $((1 << 6)) view-padding 64, \
    --tags $((1 << 6)) outer-padding 64, \
    --output HDMI-A-1 --tags 1 main-location right &
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
