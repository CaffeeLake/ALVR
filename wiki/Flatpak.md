# Installing ALVR and using SteamVR on Linux through Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak

## Disclaimer

1. This is not a fully-featured version of ALVR! It lacks Nvidia support and has bugs related to Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak sandboxing

2. Nvidia GPUs are currently not supported (but might be supported with [this PR](https://github.com/alvr-org/ALVR/pull/2207))

3. Native Linux SteamVR utility applications such as OpenVRAS are not supported nor tested, use at your own risk

4. Firewall configuration does not work

5. Any scripts that affect the host will run within the sandbox

6. Sometimes, a new instance of Steam will launch when launching the dashboard. To fix this, close both ALVR and Steam then launch Steam. As soon as Steam opens to the storefront, launch the ALVR dashboard.

7. The ALVR Dashboard is not available in the Applications menu. To run the dashboard, run the following command to run `alvr_dashboard` in the Steam Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak environment:

```sh
Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak run --command=alvr_dashboard com.valvesoftware.Steam
```

8. This only works with the Steam Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak. For non-Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak Steam, use the launcher or tar.gz

## Dependencies

First, Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak must be installed from your distro's repositories. Refer to [this page](https://Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak.org/setup/) to find the instructions for your distro.

Once Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak is installed, the Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak dependencies must also be installed. They are:

* Rust
* LLVM
* Freedesktop SDK
* Steam

These can be installed like so:

```sh
Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak install flathub org.freedesktop.Sdk//23.08 \
    org.freedesktop.Sdk.Extension.llvm16//23.08 \
    org.freedesktop.Sdk.Extension.rust-stable//23.08 \
    com.valvesoftware.Steam
```

AMD users may need to install the appropriate Mesa codec extensions as well:

```sh
Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak install flathub org.freedesktop.Platform.GL.default//23.08-extra \
   org.freedesktop.Platform.GL32.default//23.08-extra
```

## Setup

Install SteamVR via the Steam Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak. After installing SteamVR, run the following command:

```sh
sudo setcap CAP_SYS_NICE+eip ~/.var/app/com.valvesoftware.Steam/data/Steam/steamapps/common/SteamVR/bin/linux64/vrcompositor-launcher
```

This command is normally run by SteamVR, but due to the lack of sudo access within the Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak sandbox, it must be run outside of the Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak sandbox. After running the command, run SteamVR once then close it.

## Install

Download `com.valvesoftware.Steam.Utility.alvr.Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak` file from one of the latest [nightly](https://github.com/alvr-org/ALVR-nightly/releases) that contains Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak bundle and install like so:

```sh
Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak --user install --bundle com.valvesoftware.Steam.Utility.alvr.Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak
```

Use apk for headset from the same nightly.

## Build and Install

Alternatively, if the file is not available or a newer version is needed, the Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak can be built from source and installed.

First, the dependencies from above must be fulfilled. Then, install `Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak-builder` like so:

```sh
Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak install flathub org.Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak.Builder
```

Once the dependencies are fulfilled, clone and enter the repository.

```sh
git clone https://github.com/alvr-org/ALVR.git
cd ALVR
```

Once inside the repository, simply run the following command to build and install the Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak.

```sh
Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak run org.Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak.Builder --user --install --force-clean .Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak-build-dir alvr/xtask/Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak/com.valvesoftware.Steam.Utility.alvr.json
```

If ALVR is not cloned under the home directory, permission to access the directory may need to be given to the build command. An example of this is given below.

```sh
Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak run --filesystem="$(pwd)" org.Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak.Builder --user --install --force-clean .Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak-build-dir alvr/xtask/Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak/com.valvesoftware.Steam.Utility.alvr.json
```

## Notes

### Running the dashboard

To run the ALVR Dashboard, run the following command:

```sh
Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak run --command=alvr_dashboard com.valvesoftware.Steam
```

A desktop file named `com.valvesoftware.Steam.Utility.alvr.desktop` is supplied within the `alvr/xtask/Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak` directory. Move this to where other desktop files are located on your system in order to run the dashboard without the terminal.

### Other Applications

The support for other applications that are not launched via Steam is non-existent due to the Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak sandbox.

Various SteamVR utilities such as [WlxOverlay](https://github.com/galister/WlxOverlay) and [OpenVR-AdvancedSettings](https://github.com/OpenVR-Advanced-Settings/OpenVR-AdvancedSettings) cannot run within the Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak sandbox due to their usage of AppImage. However, unpacking the supplied AppImage or building the utilities from source and running their binaries from within the sandbox similiarly to `alvr_dashboard` could work, but there is no guarantee that they will work properly.

Some applications such as [Godot](https://godotengine.org) support OpenXR. However, unless they are launched within the Steam Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak sandbox, they will not work with the Steam Installing-ALVR-and-using-SteamVR-on-Linux-through-Flatpak. See [here](https://github.com/flathub/com.valvesoftware.Steam/issues/1010) for more details.
