This is a minimal repro for
[mitsuhiko/insta#70](https://github.com/mitsuhiko/insta/issues/70). It includes
one snapshot test which will always fail, and uses a snapshot path that is
outside the `src/` directory. The issue is that `cargo-insta` cannot work with
snapshots in this location.

To see it for yourself, first [install
`cargo-insta`](https://crates.io/crates/cargo-insta), then:

```sh
git clone https://github.com/mkantor/demo-cargo-insta-issue-70.git
cd demo-cargo-insta-issue-70

# This will fail because the snapshot doesn't match, which is what we want:
cargo test

# However, this will succeed with "done: no snapshots to review", when it
# should start interactive review for the new snapshot:
cargo insta test --review
```
