# Contributing
Thanks for your interest in contributing to Voting App!

Before contributing to this repository, please discuss the change you wish to make via issue on this repository, email to one of the codeowners, or on the ScottyLabs [discord](go.scottylabs.org/discord).

## How Can I Contribute?
For now, please just refer to the communication channels listed above. As this project matures, we will establish a more well-formed contributing structure.

## Documentation
When making a change, it would be wonderful if you could update the corresponding documentation. If you cannot or are unsure how to, please leave an issue or let [Yiyoung Liu](github.com/maybe-yiyi) know so that the documentation does not lag behind. If the documentation does not exist, don't worry about it! (or write the documentation yourself, that would be greatly appreciated.)

## Pull Requests
Direct pushes to main are blocked. You should create a branch (if you are a contributor in ScottyLabs) or fork the repository, make your changes, then create a PR to main.

## Style Guide
- All Rust code should be formatted using `cargo fmt` and linted with `cargo clippy`. The github CI/CD will check that all PR'ed code passes `cargo fmt` and `cargo clippy`.
- All Svelte code should checked with `bun run check`. The github CI/CD will automatically check this too.
