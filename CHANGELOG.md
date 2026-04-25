# Changelog

All notable changes to this project will be documented in this file.

## [v1.1.1](https://github.com/Antiz96/oniri/releases/tag/v1.1.1) - 2026-04-25

### Miscellaneous

- *(deps)* Update Rust crate niri-ipc to 26.4.0 ([#36](https://github.com/Antiz96/oniri/pull/36)) - ([2e8d9a2](https://github.com/Antiz96/oniri/commit/2e8d9a272a4152788e7d9d524da21101f7080d30)) by @renovate[bot]

## [v1.1.0](https://github.com/Antiz96/oniri/releases/tag/v1.1.0) - 2026-04-21

### Features

- General refactor and new tiling layout mode option ([#33](https://github.com/Antiz96/oniri/pull/33)) - ([d9d08c7](https://github.com/Antiz96/oniri/commit/d9d08c7be4085bc11916af17c63695c33d62d83e)) by @1Naim

### Fixes

- *(event)* Differentiate WindowOpened and WindowChanged ([#35](https://github.com/Antiz96/oniri/pull/35)) - ([67132a3](https://github.com/Antiz96/oniri/commit/67132a3161d7745ca78350532eaceb7a9eb68ed8)) by @1Naim
- Skip parsing Nautilus when it is in loading state ([#34](https://github.com/Antiz96/oniri/pull/34)) - ([8526532](https://github.com/Antiz96/oniri/commit/8526532d98af47dd99204b2bba607370105e961f)) by @1Naim

## [v1.0.5](https://github.com/Antiz96/oniri/releases/tag/v1.0.5) - 2026-04-20

### Fixes

- *(maximize_window)* Only iterate on workspace of the current window ([#32](https://github.com/Antiz96/oniri/pull/32)) - ([def67ce](https://github.com/Antiz96/oniri/commit/def67ce7ea0f2f145b9e9391abb055e103d6f73f)) by @1Naim

## [v1.0.4](https://github.com/Antiz96/oniri/releases/tag/v1.0.4) - 2026-04-19

### Fixes

- Properly handle workspace map cleanup and window targeting ([#31](https://github.com/Antiz96/oniri/pull/31)) - ([9151589](https://github.com/Antiz96/oniri/commit/9151589b963182b154e0cb8f9cbce35f1771c6c1)) by @1Naim

### Miscellaneous

- *(deps)* Update Rust crate assert_cmd to 2.2.1 ([#30](https://github.com/Antiz96/oniri/pull/30)) - ([d0b6f5b](https://github.com/Antiz96/oniri/commit/d0b6f5b68e8fb66ac42f1766a18b979fe9447a07)) by @renovate[bot]

## [v1.0.3](https://github.com/Antiz96/oniri/releases/tag/v1.0.3) - 2026-04-16

### Fixes

- *(event)* Prevent duplicate windows in the workspace map ([#29](https://github.com/Antiz96/oniri/pull/29)) - ([fefb66c](https://github.com/Antiz96/oniri/commit/fefb66c802b09b70750a09cc5fffe1dfb0524099)) by @1Naim

## [v1.0.2](https://github.com/Antiz96/oniri/releases/tag/v1.0.2) - 2026-04-05

### Miscellaneous

- *(deps)* Update Rust crate env_logger to 0.11.10 ([#27](https://github.com/Antiz96/oniri/pull/27)) - ([3c95434](https://github.com/Antiz96/oniri/commit/3c95434c95c5f9c3a271cdaea544f1d9c4f47523)) by @renovate[bot]
- Update release script to avoid bumping dependencies version unexpectedly ([#28](https://github.com/Antiz96/oniri/pull/28)) - ([7ae5439](https://github.com/Antiz96/oniri/commit/7ae543911ee127ce21068503f472fbadc4477b09)) by @Antiz96

## [v1.0.1](https://github.com/Antiz96/oniri/releases/tag/v1.0.1) - 2026-03-20

### Fixes

- *(log)* Update ERROR log message ([#25](https://github.com/Antiz96/oniri/pull/25)) - ([0b55c07](https://github.com/Antiz96/oniri/commit/0b55c07dd1fe6e9a2f579b680949d8e6d2512c9f)) by @Antiz96
- *(log)* Improve logging ([#24](https://github.com/Antiz96/oniri/pull/24)) - ([cf5b6a3](https://github.com/Antiz96/oniri/commit/cf5b6a37859b5ae17d64dba0855183bd600b221b)) by @Antiz96

### Documentation

- Add modules and functions documentation comments ([#26](https://github.com/Antiz96/oniri/pull/26)) - ([dfe744b](https://github.com/Antiz96/oniri/commit/dfe744b256d0dd8995be181eb49f31805da970c9)) by @Antiz96

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
