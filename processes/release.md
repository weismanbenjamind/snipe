# Process for generating release

- [ ] Create branch off `master` with name `pre-release-prep-<version-number>`.
- [ ] Bump version in `cargo.toml` to `<version-number>`.
- [ ] Run `cargo update --package snipe` to ensure the `Cargo.lock` file gets update with the new version number.
- [ ] Update the `## Unreleased` section in the `CHANGELOG.md` to `Version <version-number>`.
- [ ] Run `./scripts/lint.sh` to apply linting changes if needed.
- [ ] Commit changes.
- [ ] Merge `pre-release-prep-<version-number>` branch into `master`.
- [ ] Delete the `pre-release-prep-<version-number>` branch.
- [ ] Create new branch at `releases/v<release-version-number>`.
- [ ] Create tag this commit at `releases/v<release-version-number>` titled `v<release-version-number>`.
- [ ] Create a new GitHub release titled `Version <release-version-number>` containing the contents of the `CHANGELOG.md` file for the target release.
- [ ] The above command should cause the `build` workflow on the release tag and attach the build artifacts to the release. Ensure this behavior occurs properly.
- [ ] Merge this `releases/v<release-version-number>` branch into `releases/latest`.
- [ ] Checkout the `master` branch.
- [ ]  Bump version in `cargo.toml` to `<version-number-bumped-by-minor>-dev.0`.
- [ ] Run `cargo update --package snipe` to ensure the `Cargo.lock` file gets update with the new version number.
- [ ] Add a new `## Unreleased` section in the `CHANGELOG.md` file.
- [ ] Commit these changes into `master`.
