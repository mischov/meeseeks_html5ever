# Release checklist

In order to release a new version to Hex.pm we first need to:

1. Write the changes in the `CHANGELOG.md` file
1. Create a release branch named `release/<version>`
1. Update the `README.md`, `CHANGELOG.md`, `mix.exs`, and `Cargo.toml` with the new version
1. Commit with message `Release <version>`
1. Merge PR to `main`
1. Tag main with `git tag <version>`
1. Push tag with `git push origin <version>`
1. Wait for the CI to build all release files
1. Run `mix rustler.download MeeseeksHtml5ever.Native --all --print`
1. Copy the output of the mix task and add to the release notes
1. Run `mix hex.publish` and **make sure the checksum file is present**
in the list of files to be published.

It's important to ensure that we publish the checksum file with the
package because otherwise the users won't be able to use the lib
with precompiled files. They will need to always enforce compilation.
