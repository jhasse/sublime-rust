# Build Settings

A variety of settings are available to customize how Cargo is run.
These settings can be set globally, per-Sublime project, per-Cargo package, for specific build variants ("Build", "Run", "Test", etc.), or specific Cargo targets (`--lib`, `--example foo`, etc.).

## Configure Command

To help you configure the Cargo build settings, run the `Rust: Configure Cargo Build` command from Sublime's Command Palette (Ctrl-Shift-P / ⌘-Shift-P).
This will ask you a series of questions for the setting to configure.
It will update your `.sublime-project` or `Users/RustEnhanced.sublime-settings` file depending on which options you pick.
The first question is the setting you want to update:

Setting | Description
------- | -----------
Target | Specify an explicit target (`--bin`, `--lib`, `--example`, etc.). The "Automatic Detection" option will attempt to determine which target to use based on the current active view in Sublime (a test file will use `--test` or a binary will use `--bin`, etc.).
Profile | Determine whether or not the `--release` flag is used.
Target Triple | The `--target` option to specify a target triple (such as `x86_64-apple-darwin`).
Toolchain | Set the Rust toolchain to use (`nightly`, `beta`, etc.).
Features | Set the Cargo build features to use.
Environment Variables | Specify additional environment variables to set.
Extra Cargo Arguments | Extra arguments to include in the command line.
Default Package/Path | The default package to build, useful if you have a workspace with multiple Cargo packages. See `Multiple Cargo Projects` below.

If you have multiple Cargo packages in your workspace, it will ask for the package to configure.

Caution: If you have not created a `sublime-project` file, then any changes you make will be lost if you close the Sublime window.

## Settings

Cargo settings are stored in the `"cargo_build"` Sublime setting.
This can be either in your `sublime-project` file or in `Users/RustEnhanced.sublime-settings`.
`"cargo_build"` is an object with the following keys:

Key | Description
--- | -----------
`"paths"` | Settings for specific Cargo packages.
`"default_path"` | The default Cargo package to build (useful for workspaces, see below).
`"variants"` | Settings per build variant.
`"defaults"` | Default settings used if not set per target or variant.

Paths should be an absolute path to the directory of a Cargo package.

`"paths"` is an object of path keys mapping to an object with the keys:

Path Key | Description
-------- | -----------
`"defaults"` | Default settings used if not set per target or variant.
`"targets"` | Settings per target (such as `"--lib"` or `"--bin foo"`).
`"variants"` | Settings per build variant.

An example of a `sublime-project` file:

```json
{
    "folders": [
        { "path": "." }
    ],
    "settings": {
        "cargo_build": {
            "paths": {
                "/path/to/package": {
                    "defaults": {
                        "release": false
                    },
                    "targets": {
                        "--example ex1": {
                            "extra_run_args": "-f file"
                        }
                    },
                    "variants": {
                        "bench": {
                            "toolchain": "nightly"
                        },
                        "clippy": {
                            "toolchain": "nightly"
                        }
                    }
                }
            },
            "default_path": "/path/to/package",
            "variants": {
                "run": {
                    "env": {
                        "RUST_BACKTRACE": "1"
                    }
                }
            },
            "defaults": {
                "release": true
            }
        }
    }
}
```

The available settings are:

Setting Name | Description
------------ | -----------
`target` | The Cargo target (such as `"--bin myprog"`). Applies to `variants` only. Can be `"auto"` (see "Automatic Detection" above).
`toolchain` | The Rust toolchain to use (such as `nightly` or `beta`).
`target_triple` | If set, uses the `--target` flag with the given value.
`release` | If true, uses the `--release` flag.
`features` | A string with a space separated list of features to pass to the `--features` flag. Set to "ALL" to pass the `--all-features` flag.
`extra_cargo_args` | String of extra arguments passed to Cargo (before the `--` flags separator).
`extra_run_args` | String of extra arguments passed to Cargo (after the `--` flags separator).
`env` | Object of environment variables to add when running Cargo.
`working_dir` | The directory where to run Cargo. If not specified, uses the value from `default_path`, otherwise attempts to detect from the active view, or displays a panel to choose a Cargo package.
`script_path` | Path to a `.rs` script (previously used by `cargo script`, now unused).
`no_default_features` | If True, sets the `--no-default-features` flag.

The extra args settings support standard Sublime variable expansion (see [Build System Variables](https://www.sublimetext.com/docs/build_systems.html)).

## Setting Precedence

The Cargo commands will generally use the most specific setting available.
The order they are searched are (first found value wins):

1. `.sublime-project` > Cargo Package > Cargo Target
2. `.sublime-project` > Cargo Package > Build Variant
3. `.sublime-project` > Cargo Package > Defaults
4. `.sublime-project` > Build Variant
5. `RustEnhanced.sublime-settings` > Build Variant
6. `.sublime-project` > Defaults
7. `RustEnhanced.sublime-settings` > Defaults
