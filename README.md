## dm-SpaceEcho

A space echo effect written in Rust.
The effect can be compiled to a [lv2](./lv2), [vst](./vst) or [vst3/CLAP](./vst3) plugin.
This plugin has been written primarily to run on [Mod devices](https://moddevices.com/). And because I mainly use this for guitar it's just mono to stereo for now.

## Table of contents:

- [Mod devices installation](#Mod-devices-installation)
- [LV2 installation](#LV2-installation)
- [VST installation](#VST-installation)
- [VST3 installation](#VST3-installation)
- [CLAP installation](#CLAP-installation)

## Mod devices installation

You can find the plugin for the Mod Dwarf [here](./lv2/dm-SpaceEcho.lv2/).

To build the plugin for your MOD device see these [instructions](https://github.com/moddevices/mod-plugin-builder).

## LV2 installation
Run `cd lv2 && cargo build --release`.

If you want to build for a specific architecture you can add a specific build target:
1. You can list all available targets with this command: `rustup target list`
2. You can add a target like this: `rustup target add <target>` (e.g. `rustup target add armv7-unknown-linux-gnueabihf`)
3. Then build for this target by running `cd lv2 && cargo build --release --target <target>`

DISCLAIMER:
I don't use this plugin format myself. So if someone wants to try this, please let me know how that goes. Let me know when you run into issues and if I can add some instruction to this README. 

## VST installation

Windows:

1. Run `./scripts/build-vst.sh`
2. Copy dll file in /target/release to your vst plugin folder

Intel Mac:

- Run `./scripts/build-vst-for-mac.sh`.

M1 Mac:

- Run `./scripts/build-vst-for-mac-m1.sh`.

## VST3 installation

Mac:

- Run: `./scripts/build-vst3-for-mac.sh`

## CLAP installation
This plugin can be compiled to a CLAP plugin aswell. 

Run `cd vst3 && cargo xtask bundle dm_space_echo --release`.
Then look for the compiled CLAP plugin at [./vst3/target/bundled/dm_space_echo.clap](./vst3/target/bundled/dm_space_echo.clap). Then copy this file to the required location.

DISCLAIMER:
I don't use this plugin format myself. So if someone wants to try this, please let me know how that goes. Let me know when you run into issues and if I can add some instruction to this README. 