# Changelog

All notable changes to this project will be documented in this file.

## [v1.0.0](https://github.com/Antiz96/oniri/releases/tag/v1.0.0) - 2026-03-19

### Features

- *(arg)* Add the -h / --help argument ([#14](https://github.com/Antiz96/oniri/pull/14)) - ([d3b3300](https://github.com/Antiz96/oniri/commit/d3b33007df7fcabd46dc18f927272bc62dfb6775)) by @Antiz96
- *(ipc)* Add IPC listener and act on events ([#2](https://github.com/Antiz96/oniri/pull/2)) - ([df8e765](https://github.com/Antiz96/oniri/commit/df8e765b6218a361147b0956685e5d27feb77418)) by @Antiz96
- Add shell completions ([#17](https://github.com/Antiz96/oniri/pull/17)) - ([c3614ef](https://github.com/Antiz96/oniri/commit/c3614eff4d3d4cdf58a9b2b4e7a3fc13094527bb)) by @Antiz96
- Add the -F / --first-only CLI argument ([#9](https://github.com/Antiz96/oniri/pull/9)) - ([7b45471](https://github.com/Antiz96/oniri/commit/7b454711725911cd1f14d232de64cf778a2e7ab4)) by @Antiz96
- Make height and width tolerance configurable ([#7](https://github.com/Antiz96/oniri/pull/7)) - ([fe84143](https://github.com/Antiz96/oniri/commit/fe841438c88fd272ef2b186e2165ff5fb7fd2d26)) by @Antiz96

### Fixes

- Initialize workspace/window(s) map after creation ([#12](https://github.com/Antiz96/oniri/pull/12)) - ([32580a1](https://github.com/Antiz96/oniri/commit/32580a1eb96b1e496529552f6eeec05aa2c2386f)) by @Antiz96

### Documentation

- *(README)* Add notice about current niri IPC limitations and related (eventual) buggy behaviors in specific setups / edgy cases ([#20](https://github.com/Antiz96/oniri/pull/20)) - ([e019268](https://github.com/Antiz96/oniri/commit/e019268929d703db0f719c8455f754865a828213)) by @Antiz96
- *(README)* Add video demo ([#19](https://github.com/Antiz96/oniri/pull/19)) - ([caf00f2](https://github.com/Antiz96/oniri/commit/caf00f2fc31ae9c65fa460075ea7d9ef2ae59a4a)) by @Antiz96
- *(README)* Expand installation instructions ([#6](https://github.com/Antiz96/oniri/pull/6)) - ([b0fe699](https://github.com/Antiz96/oniri/commit/b0fe69930fc80161c73c8da3dc902559b85daf7b)) by @Antiz96
- *(RELEASE)* Add gcc and rustup as dependencies ([#10](https://github.com/Antiz96/oniri/pull/10)) - ([d0dd472](https://github.com/Antiz96/oniri/commit/d0dd472cb2fd1e6b8f2b1a57d7dbd929390ad121)) by @Antiz96
- *(man)* Add extra niri link + typo fix ([#18](https://github.com/Antiz96/oniri/pull/18)) - ([5dfe463](https://github.com/Antiz96/oniri/commit/5dfe4635542578d8ddd95e90cb215f1ac4ce44a3)) by @Antiz96
- *(man)* Add man page ([#16](https://github.com/Antiz96/oniri/pull/16)) - ([fd3c6a9](https://github.com/Antiz96/oniri/commit/fd3c6a9db48141dd7fd5507b13be8453b0bdf0c0)) by @Antiz96

### Styling

- Use a clearer and more conventional naming for modules and functions ([#13](https://github.com/Antiz96/oniri/pull/13)) - ([0249614](https://github.com/Antiz96/oniri/commit/0249614c73f9245cac34b981c362c849f969e1e7)) by @Antiz96

### Miscellaneous

- Update release script to build binary before committing ([#23](https://github.com/Antiz96/oniri/pull/23)) - ([ae674dd](https://github.com/Antiz96/oniri/commit/ae674dda35f17a6e3a04fe5f7fbf1b19527534d6)) by @Antiz96
- Update version in man page from release script ([#22](https://github.com/Antiz96/oniri/pull/22)) - ([26bdf30](https://github.com/Antiz96/oniri/commit/26bdf304fd1c4d61658baa15bc88beb30552ce87)) by @Antiz96
- *(CHANGELOG)* Remove duplicate top level header ([#4](https://github.com/Antiz96/oniri/pull/4)) - ([6a12d2](https://github.com/Antiz96/oniri/commit/6a12d22aa18c020e0176201b1efaff357aad6259)) by @Antiz96
- *(codebase)* Split functions into separate modules ([#5](https://github.com/Antiz96/oniri/pull/5)) - ([17b5cb1](https://github.com/Antiz96/oniri/commit/17b5cb102df2b12be9fdc325fa423067351659a2)) by @Antiz96
- *(release)* Add pre-compiled binary to releases assets ([#8](https://github.com/Antiz96/oniri/pull/8)) - ([bb23535](https://github.com/Antiz96/oniri/commit/bb235358a9d171a50cd17eef55b8277c29729a90)) by @Antiz96
- *(issue_template)* Mention how to gather some logs for bug report ([#11](https://github.com/Antiz96/oniri/pull/11)) - ([1e5b1d6](https://github.com/Antiz96/oniri/commit/1e5b1d6e4748923a0baa1d34d13ae5dc23385012)) by @Antiz96
- Centralize argument parsing ([#15](https://github.com/Antiz96/oniri/pull/15)) - ([ef8207b](https://github.com/Antiz96/oniri/commit/ef8207b4547e900b14ebc74f73986cf5f4461b53)) by @Antiz96
- Make release script executable ([#21](https://github.com/Antiz96/oniri/pull/21)) - ([4e870cd](https://github.com/Antiz96/oniri/commit/4e870cdf53bc166b670a9332a1c113f73a1129e0)) by @Antiz96

## [v0.0.1](https://github.com/Antiz96/oniri/releases/tag/v0.0.1) - 2026-03-18

### Features

- *(init)* Initial commit ([#1](https://github.com/Antiz96/oniri/pull/1)) - ([ac62c6c](https://github.com/Antiz96/oniri/commit/ac62c6c63e1b7c3a923b0cbc460db26d70494be6)) by @Antiz96
