# Changelog

## [0.1.4](https://github.com/open-feature/rust-sdk/compare/open-feature-v0.1.3...open-feature-v0.1.4) (2023-12-19)


### ✨ New Features

* add (optional) serde_json conversion support ([#45](https://github.com/open-feature/rust-sdk/issues/45)) ([6dde097](https://github.com/open-feature/rust-sdk/commit/6dde097b160ce821ac2f662e5eb1c3b8855559c0))

## [0.1.3](https://github.com/open-feature/rust-sdk/compare/open-feature-v0.1.2...open-feature-v0.1.3) (2023-12-15)


### 🐛 Bug Fixes

* **deps:** update rust crate async-trait to 0.1.74 ([#37](https://github.com/open-feature/rust-sdk/issues/37)) ([d5e9d91](https://github.com/open-feature/rust-sdk/commit/d5e9d91a100009dfc8bd9e1459b404c8ee0f4f30))
* **deps:** update rust crate time to 0.3.30 ([#36](https://github.com/open-feature/rust-sdk/issues/36)) ([864ba1f](https://github.com/open-feature/rust-sdk/commit/864ba1fb333b748b806e72c18412e79e9dbd613d))
* **deps:** update rust crate tokio to 1.33.0 ([#35](https://github.com/open-feature/rust-sdk/issues/35)) ([c3624f8](https://github.com/open-feature/rust-sdk/commit/c3624f8d534cf5b8bdbb6dcecd10d1d9ec8f0210))
* **deps:** update rust crate tokio to 1.34.0 ([#40](https://github.com/open-feature/rust-sdk/issues/40)) ([959d3ed](https://github.com/open-feature/rust-sdk/commit/959d3ed9f934af17ab02b5c35d9d26d38673a903))
* **deps:** update rust crate tokio to 1.35.0 ([#42](https://github.com/open-feature/rust-sdk/issues/42)) ([a0f9f80](https://github.com/open-feature/rust-sdk/commit/a0f9f8058e35dc84bd09a16c042c72e56f15d91d))
* **deps:** update rust crate typed-builder to 0.18.0 ([#38](https://github.com/open-feature/rust-sdk/issues/38)) ([4f6c2cb](https://github.com/open-feature/rust-sdk/commit/4f6c2cb2d45029322819f34a742a93f81a1d6031))


### ✨ New Features

* use TryFrom instead of custom trait for StructValue conversion ([#43](https://github.com/open-feature/rust-sdk/issues/43)) ([8981b45](https://github.com/open-feature/rust-sdk/commit/8981b45abef478a720a582f43a30aecd2d68a4d5))


### 🧹 Chore

* add badges to readme ([bf4b901](https://github.com/open-feature/rust-sdk/commit/bf4b901ff7a6574fd2f44287842aea39e23a204b))
* add release badge to readme ([#33](https://github.com/open-feature/rust-sdk/issues/33)) ([31b0e48](https://github.com/open-feature/rust-sdk/commit/31b0e487a2ad6376d4b994b35410556ac8cd80ae))
* **deps:** update actions/checkout action to v4 ([#28](https://github.com/open-feature/rust-sdk/issues/28)) ([1cb61c9](https://github.com/open-feature/rust-sdk/commit/1cb61c93534ae037b7dd2c143e03809b877c2728))
* **deps:** update google-github-actions/release-please-action action to v4 ([#41](https://github.com/open-feature/rust-sdk/issues/41)) ([f6c8cba](https://github.com/open-feature/rust-sdk/commit/f6c8cbacbe5106355d565858f5f43637b0e4203f))
* update readme ([397ff70](https://github.com/open-feature/rust-sdk/commit/397ff70ae1526f7741dcbe57df1fc6b8036ad90e))
* update readme for automated inclusion ([#39](https://github.com/open-feature/rust-sdk/issues/39)) ([7b52561](https://github.com/open-feature/rust-sdk/commit/7b52561c14befc304a91508fbe9d526664b2be47))

## [0.1.2](https://github.com/open-feature/rust-sdk/compare/open-feature-v0.1.1...open-feature-v0.1.2) (2023-09-27)


### 🐛 Bug Fixes

* update license to a valid license identifier ([88e38e4](https://github.com/open-feature/rust-sdk/commit/88e38e454d4ee06ff7b83b4abb025a857d48f30a))

## [0.1.1](https://github.com/open-feature/rust-sdk/compare/open-feature-v0.1.0...open-feature-v0.1.1) (2023-09-27)


### 🐛 Bug Fixes

* **deps:** update rust crate time to 0.3.29 ([#26](https://github.com/open-feature/rust-sdk/issues/26)) ([affff7e](https://github.com/open-feature/rust-sdk/commit/affff7eb912a475cd4314a609f388bb62d4cd84c))
* **deps:** update rust crate typed-builder to 0.16.1 ([#22](https://github.com/open-feature/rust-sdk/issues/22)) ([953f68b](https://github.com/open-feature/rust-sdk/commit/953f68b5b462f8662837822f160d10a998e3f607))
* **deps:** update rust crate typed-builder to 0.16.2 ([#25](https://github.com/open-feature/rust-sdk/issues/25)) ([4808212](https://github.com/open-feature/rust-sdk/commit/4808212f59471c51be1558dfd43e5c44d6bda811))


### ✨ New Features

* add shutdown() function to provider trait ([9f779b3](https://github.com/open-feature/rust-sdk/commit/9f779b32aac79970052b8a65d0d9bbf7beb1605a))
* enhance singleton and tests ([3a655bf](https://github.com/open-feature/rust-sdk/commit/3a655bfd46facaa7d975268ac36a37396f02b298))
* implement evaluation details and re-design evaluation API ([#24](https://github.com/open-feature/rust-sdk/issues/24)) ([d6aace1](https://github.com/open-feature/rust-sdk/commit/d6aace1a47ed41974a2916fd7576c59fbeeba9d2))
* implement support for array type ([#19](https://github.com/open-feature/rust-sdk/issues/19)) ([525223d](https://github.com/open-feature/rust-sdk/commit/525223d6fb88e10bdb0a05e8f6acedbdb8fa4f7e))


### 🧹 Chore

* Add tooling to detect missing specification tests ([#23](https://github.com/open-feature/rust-sdk/issues/23)) ([daf2ae2](https://github.com/open-feature/rust-sdk/commit/daf2ae2d4d742814418c39976bffb71e5a865a1a))
* adding WIP label to readme. ([734e813](https://github.com/open-feature/rust-sdk/commit/734e8131457ee3a04a358c7b16ee6dbee6074c8a))
* **deps:** update actions/checkout action to v4 ([#14](https://github.com/open-feature/rust-sdk/issues/14)) ([98d12c0](https://github.com/open-feature/rust-sdk/commit/98d12c043d47e1210d707b5fa1cafd6b50cd8aec))
* prepare for automatic release ([#27](https://github.com/open-feature/rust-sdk/issues/27)) ([4aa0dd5](https://github.com/open-feature/rust-sdk/commit/4aa0dd55d6e33b7881a560595377d659767d891e))


### 🔄 Refactoring

* change EvaluationContext to take a reference ([9cc5af2](https://github.com/open-feature/rust-sdk/commit/9cc5af29a20b96c7ce0f16779039d2e6fa677c65))
