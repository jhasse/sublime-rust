# Custom Build Variants

You can define your own build system that takes advantage of the Cargo settings.
This is useful if you want to quickly switch between different configurations, or to add support for Cargo commands that are not already included.

The build variants are stored in your `.sublime-project` file.
To assist you in configuring a build variant, there is a Sublime command called `"Rust: Create New Cargo Build Variant"` which you can access from the Command Palette.
It will ask a series of questions, and when it is done it will automatically add the new build variant to your `.sublime-project` file.
Then use the "Build With..." command (Ctrl-Shift-B / ⌘-Shift-B) to select and execute your new variant.
The command will also copy over the stock build variants so you do not need to switch between build systems.

You can manually edit your `.sublime-project` file to change the settings.
The settings described above are under the `"settings"` key.
Additionally, there is a `"command_info"` key which describes the features the command supports.
The available values are:

Setting Name | Default | Description
------------ | ------- | -----------
`allows_target` |  False | If True, the command accepts cargo filters for determining which target to build (`--lib`, `--bin foo`, `--example bar`, etc.). Can also be a sequence of strings like `["bin", "example"]` to specify a subset of targets it supports.
`allows_target_triple` |  False | If True, the command accepts triples like `--target x86_64-apple-darwin`.
`allows_release` |  False | If True, allows `--release` flag.
`allows_features` |  False | If True, allows feature flags.
`allows_json` |  False | If True, allows `--message-format=json` flag.
`json_stop_pattern` | None | A regular expression matched against Cargo's output to detect when it should stop looking for JSON messages (used by `cargo run` to stop looking for JSON messages once compilation is finished).
`requires_manifest` | True | If True, the command must be run in a directory with a `Cargo.toml` manifest.
`requires_view_path` |  False | If True, then the active view must be a Rust source file, and the path to that file will be passed into Cargo (previously used by `cargo script`, now unused).
`wants_run_args` |  False | If True, it will ask for extra args to pass to the executable (after the `--` flag separator).
