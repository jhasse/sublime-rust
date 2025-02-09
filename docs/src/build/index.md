# Cargo Build System

Rust Enhanced has a custom build system tailored for running Cargo.
It will display errors and warnings inline using Sublime's phantoms (see [Diagnostic Messages](messages.md) for settings to control how messages are displayed).
It also supports a variety of configuration options to control how Cargo is run.

![testingrust](https://cloud.githubusercontent.com/assets/43198/22944409/7780ab9a-f2a5-11e6-87ea-0e253d6c40f6.png)

## Usage

When Sublime is set to use "Automatic" build system detection, it will choose the build system based on the syntax of the currently active view.
When you have a file open with the `.rs` extension, Sublime will automatically select the Rust Enhanced syntax highlighting, and the Automatic build system will pick Rust Enhanced to perform the build.
If you want to ensure the Rust Enhanced build system is used regardless of which file is open, choose it via `Tools > Build System > RustEnhanced`.

The basic Sublime commands available are:

Command | Keyboard | Menu | Description
------- | -------- | ---- | -----------
Build | Ctrl-B / ⌘-B | Tools > Build | Runs the currently active build variant.
Build With... | Ctrl-Shift-B / ⌘-Shift-B | Tools > Build With... | Choose the build variant.
Cancel Build | Ctrl-Break / Ctrl-C | Tools > Cancel Build | Abort the currently running build.
Show Build Results | | Tools > Build Results > Show Build Results | Opens the output panel with build results.
Next Result | F4 | Tools > Build Results > Next Result | Go to the next warning/error message.
Previous Result | Shift-F4 | Tools > Build Results > Previous Result | Go to the previous warning/error message.

See the [Build Settings](settings.md) chapter for information on customizing how the build commands are run.

## Build Variants

When you select the RustEnhanced build system in Sublime, there are a few variants that you can select with Tools > Build With... (Ctrl-Shift-B / ⌘-Shift-B).
They are:

Variant | Command | Description
------- | ------- | -----------
(Default) | <code>cargo&nbsp;build</code> | Builds the project.
Automatic | | Automatically detect the command to run based on the currently active view (tests do `test`, binaries and examples do `run`, libraries do `build`, benches do `bench`).
Run | <code>cargo&nbsp;run</code> | Runs the binary.
Run (with args)... | <code>cargo&nbsp;run&nbsp;-&#8288;-&#8288;&nbsp;*args*</code> | Runs the binary with optional arguments you specify.
Test | <code>cargo&nbsp;test</code> | Runs unit and integration tests.
Test (with args)... | <code>cargo&nbsp;test&nbsp;-&#8288;-&#8288;&nbsp;*args*</code> | Runs the test with optional arguments you specify.
Bench | <code>cargo&nbsp;bench</code> | Runs benchmarks.
Clean | <code>cargo&nbsp;clean</code> | Removes all built files.
Document | <code>cargo&nbsp;doc</code> | Builds package documentation.
Clippy | <code>cargo&nbsp;clippy</code> | Runs [Clippy](https://github.com/Manishearth/rust-clippy). Clippy must be installed, and currently requires the nightly toolchain.

You can add custom build variants, see [Custom Build Variants](custom.md) for more.

## Multiple Cargo Projects (Advanced)

You can have multiple Cargo projects in a single Sublime project (such as when using Cargo workspaces, or if you simply have multiple projects in different folders).

If you have multiple Cargo projects in your Sublime window, the build system will use the currently active view to attempt to determine which project to build.
Otherwise it will show an input panel to select a package.

You can set the `default_path` setting to always use a specific path.
It is specified at the same level as `paths` (see [Build Settings](settings.md)).
This can be set using the  `Rust: Configure Cargo Build` command.
