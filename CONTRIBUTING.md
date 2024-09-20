Thank you for considering contributing to OpenFeature Rust SDK! We appreciate your help and look forward to collaborating with you.

Here are the key steps for contributing to the OpenFeature Rust SDK:

## How to Contribute

1. **Fork the Repository**: Create a personal fork of the [repository](https://github.com/open-feature/rust-sdk.git) on GitHub.
2. **Clone Your Fork**:
   ```sh
   mkdir open-feature
   cd open-feature
   git clone https://github.com/open-feature/rust-sdk.git
   cd rust-sdk
   ```
3. **Create a Branch**: Create a new branch with a name that follows the recommended convention:
    ```sh
    git checkout -b <prefix>/<gh-issue-number>
    ```
    - If there is a corresponding GitHub issue number, include it in the branch name.
    - Otherwise, name the branch meaningfully, using hyphens to replace spaces.
    - Use the following prefixes based on the type of change:
      - 🐛 Bug Fixes: `fix/<description>` or `fix/<issue-number>`
      - ✨ New Features: `feat/<description>` or `feat/<issue-number>`
      - 🧹 Chore: `chore/<description>` or `chore/<issue-number>`
      - 📚 Documentation: `docs/<description>` or `docs/<issue-number>`
      - 🚀 Performance: `perf/<description>` or `perf/<issue-number>`
      - 🛠️ Build: `build/<description>` or `build/<issue-number>`
      - 📦 Dependencies: `deps/<description>` or `deps/<issue-number>`
      - 🚦 CI: `ci/<description>` or `ci/<issue-number>`
      - 🔄 Refactoring: `refactor/<description>` or `refactor/<issue-number>`
      - 🔙 Reverts: `revert/<description>` or `revert/<issue-number>`
      - 🎨 Styling: `style/<description>` or `style/<issue-number>`
      - 🧪 Tests: `test/<description>` or `test/<issue-number>`

4. **Make Changes**: Implement your changes or additions to the codebase.
5. **Commit Changes**: Stage your changes and commit them with a descriptive message.
6. **Push to Your Fork**: Push your branch to your forked repository.
7. **Create a Pull Request**: Open a pull request from your forked repository to the main OpenFeature Rust SDK repository.
8. **Address Feedback**: If there are any comments or requested changes, address them in your branch and push the updates.
9. **Merge**: Once your pull request is approved, it will be merged into the main repository.

## Development Setup

To set up your development environment, ensure you have the following installed:

- [Rust & Cargo](https://www.rust-lang.org/tools/install)
- [VS Code](https://code.visualstudio.com)
- [Rust Analyzer VS Code Extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

After installing Rust, you can verify your installation with following command:

```sh
rustc --version
```

After installing Cargo, you can verify your installation with following command:

```sh
cargo --version
```

## Build, Test, Document and Run the Project

To build the project, execute the following command:
```sh
cargo build
```

To test the project, execute the following command:
```sh
cargo test
```

To build documentation for the project, execute the following command:
```text
cargo doc
```

To run the project, execute the following command:
```sh
cargo run
```

## Closing Note

Thank you 🙏 for your interest and support!  Your contributions help us improve and grow 🌱 the OpenFeature Rust SDK. We truly appreciate your time and effort in making our project better. If you have any questions or need assistance, please don’t hesitate to reach out on Slack in the [#openfeature-rust](https://cloud-native.slack.com/archives/C05RG7F93NV) channel!