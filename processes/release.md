# Process for generating release

- [ ] Create branch off `master` with name `pre-release-prep-<version-number>`
- [ ] Bump version in `cargo.toml` to `<version-number>`
- [ ] Update the `## Unreleased` section in the `CHANGELOG.md` to `Version <version-number>`
- [ ] Run `./scripts/format.sh` to apply formatting changes if needed
- [ ] Run `./scripts/clippy_debug.sh` and resolve all linting errors
- [ ] Run `./scripts/clippy_release.sh` and resolve all linting errors
- [ ] Commit changes
- [ ] Merge `pre-release-prep-<version-number>` branch into `master`
- [ ] Delete the `pre-release-prep-<version-number>` branch
- [ ] Create new branch at `releases/v<release-version-number>`
- [ ] Create tag this commit at `releases/v<release-version-number>` titled `v<release-version-number>`
- [ ] Run the `build_macos_arm_64` workflow on the `v<release-version-number>` branch
- [ ] Create a new GitHub release titled `Version <release-version-number>` containing the contents of the `CHANGELOG.md` file for the target release and with the artifacts from the `build_macos_arm_64` workflow attached
- [ ] Merge this `releases/v<release-version-number>` branch into `releases/latest`
- [ ] Checkout the `master` branch
- [ ]  Bump version in `cargo.toml` to `<version-number-bumped-by-minor>-dev.0`
- [ ] Add a new `## Unreleased` section in the `CHANGELOG.md` file
- [ ] Commit these changes into `master`
