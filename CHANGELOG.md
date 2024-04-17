# Changelog

## [0.2.3](https://github.com/open-feature/rust-sdk/compare/open-feature-v0.2.2...open-feature-v0.2.3) (2024-04-16)


### 🐛 Bug Fixes

* **deps:** update rust crate tokio to 1.37 ([#69](https://github.com/open-feature/rust-sdk/issues/69)) ([b6b418b](https://github.com/open-feature/rust-sdk/commit/b6b418b70ff655dcc31b30a5d7966e09f46038b2))
* specfinder was using an incorrect regex ([#72](https://github.com/open-feature/rust-sdk/issues/72)) ([cd42b8c](https://github.com/open-feature/rust-sdk/commit/cd42b8c228f196a035e9826fa354a308bc095a68))

## [0.2.2](https://github.com/open-feature/rust-sdk/compare/open-feature-v0.2.1...open-feature-v0.2.2) (2024-03-01)


### 🐛 Bug Fixes

* **deps:** update rust crate darling to 0.20.6 ([#64](https://github.com/open-feature/rust-sdk/issues/64)) ([0397117](https://github.com/open-feature/rust-sdk/commit/039711764b8fc0c495208729117bac20fe9db69c))
* **deps:** update rust crate darling to 0.20.8 ([#67](https://github.com/open-feature/rust-sdk/issues/67)) ([e1606c3](https://github.com/open-feature/rust-sdk/commit/e1606c3eb80cfe18843142139cdf9fd4505bc851))
* **deps:** update rust crate serde_json to 1.0.114 ([#66](https://github.com/open-feature/rust-sdk/issues/66)) ([812efe3](https://github.com/open-feature/rust-sdk/commit/812efe3538948b7e9265f463a5ba8722b2e61789))


### 🧹 Chore

* change Cargo.toml file to reference minor version ([#68](https://github.com/open-feature/rust-sdk/issues/68)) ([24fff2e](https://github.com/open-feature/rust-sdk/commit/24fff2e7a75dd74192b469f1dede272fe59c3118))

## [0.2.1](https://github.com/open-feature/rust-sdk/compare/open-feature-v0.2.0...open-feature-v0.2.1) (2024-02-13)


### 🐛 Bug Fixes

* **deps:** update rust crate darling to 0.20.5 ([#61](https://github.com/open-feature/rust-sdk/issues/61)) ([921f027](https://github.com/open-feature/rust-sdk/commit/921f027b8e60cb149af153a7047c2f22417b975c))
* **deps:** update rust crate serde_json to 1.0.113 ([#60](https://github.com/open-feature/rust-sdk/issues/60)) ([04125b2](https://github.com/open-feature/rust-sdk/commit/04125b2fd556013532dd9d44c21e424ca01760f6))
* **deps:** update rust crate time to 0.3.34 ([#62](https://github.com/open-feature/rust-sdk/issues/62)) ([a8e0ddb](https://github.com/open-feature/rust-sdk/commit/a8e0ddb2d17e66301bb8e09dc2396747eb1ebb15))
* **deps:** update rust crate tokio to 1.36.0 ([#63](https://github.com/open-feature/rust-sdk/issues/63)) ([f99de1c](https://github.com/open-feature/rust-sdk/commit/f99de1cfd49cde660425f5fb2ca8f54c50e1f738))
* **deps:** update rust crate typed-builder to 0.18.1 ([#58](https://github.com/open-feature/rust-sdk/issues/58)) ([396bf02](https://github.com/open-feature/rust-sdk/commit/396bf022402b8864cf136aa458052296fdb757b4))

## [0.2.0](https://github.com/open-feature/rust-sdk/compare/open-feature-v0.1.5...open-feature-v0.2.0) (2024-01-11)


### ⚠ BREAKING CHANGES

* re-design No-op Provider ([#56](https://github.com/open-feature/rust-sdk/issues/56))

### 🐛 Bug Fixes

* **deps:** update rust crate async-trait to 0.1.77 ([#52](https://github.com/open-feature/rust-sdk/issues/52)) ([50390e8](https://github.com/open-feature/rust-sdk/commit/50390e87b8cdd44d7a509aa9174ae3f0a53342f4))
* **deps:** update rust crate serde_json to 1.0.110 ([#53](https://github.com/open-feature/rust-sdk/issues/53)) ([1d90cd9](https://github.com/open-feature/rust-sdk/commit/1d90cd9b884999285be79604c6c7b90be24d936e))
* **deps:** update rust crate serde_json to 1.0.111 ([#54](https://github.com/open-feature/rust-sdk/issues/54)) ([78b0bf3](https://github.com/open-feature/rust-sdk/commit/78b0bf3aab39d41bd0938b4b903eacfe0de6654a))


### ✨ New Features

* re-design No-op Provider ([#56](https://github.com/open-feature/rust-sdk/issues/56)) ([554cf22](https://github.com/open-feature/rust-sdk/commit/554cf22302781ee5c2015e75c514be3d4be67ca5))

## [0.1.5](https://github.com/open-feature/rust-sdk/compare/open-feature-v0.1.4...open-feature-v0.1.5) (2023-12-20)


### 🐛 Bug Fixes

* **deps:** update rust crate time to 0.3.31 ([#47](https://github.com/open-feature/rust-sdk/issues/47)) ([cb734b0](https://github.com/open-feature/rust-sdk/commit/cb734b0979bab5d205b7d313c2aff913f696bc65))
* **deps:** update rust crate tokio to 1.35.1 ([#48](https://github.com/open-feature/rust-sdk/issues/48)) ([8d5dd69](https://github.com/open-feature/rust-sdk/commit/8d5dd6915a6c36e5ff86e66cd33cefe7450f2d5a))


### ✨ New Features

* add conversion from &serde_json::Value ([#46](https://github.com/open-feature/rust-sdk/issues/46)) ([d450bad](https://github.com/open-feature/rust-sdk/commit/d450bad6993b9fc5e8365da38df64d2321a74281))

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
