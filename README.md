# Filtile

This is a layout manager for the [https://github.com/riverwm/river](River) window
manager. It's basically `rivertile`, but with a few things not implimented (because
I don't use them), and configuration per tag.

## Usage

All numbers will set the value, but also support a prefix of either `+` or `-`
for adjustment.

Folling are the commands that can be sent to `riverctl send-layout-cmd filtile ...`:

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
    <dt>pad</dt>
    <dd>Toggle single view padding. When only one view is in the layout, it
        will be centered and given as much width as it would have if there
        were more windows.</dd>
</dl>

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
