# Contributing

Thank you for considering contributing to [oniri](https://github.com/Antiz96/oniri)!

With the exception of the [general rules](#general-rules) (which **must** be acknowledged and applied in any contribution / interaction in this project), these guidelines represent an ideal target & standards that I would like this project to follow but may not all be **strictly** enforced (depending on the situation).

Please, don't refrain yourself from contributing if you feel that your contribution may not entirely follow these guidelines (or if you're struggling applying some of them). I value your contributions much more than the strict application of these guidelines!

## Table of contents

- [General rules](#general-rules)
- [Open an issue](#open-an-issue)
- [Open a pull request](#open-a-pull-request)
- [AI Policy](#ai-policy)
- [Donations](#donations)
- [Thank you](#thank-you)

## General rules

These general rules apply to **every** contributions (whatever the type). They should **always** be acknowledged and **strictly** followed in any circumstances:

Basic common sense applies to every contributions & discussions: stay polite and respectful, no flaming / trolling / spamming or any kind of discrimination / harassment, avoid controversial topics *(specifically if it has nothing to do with this project whatsoever)*, etc...

Use English as much as possible for contributions & discussions. If required, I can also speak French, but it's important that contributions & discussions remain intelligible to most people.

## Open an issue

**Note:** To report security concerns, please follow the instructions in the [SECURITY.md](https://github.com/Antiz96/oniri/blob/main/SECURITY.md) document.

Before [opening an issue](https://github.com/Antiz96/oniri/issues/new/choose), verify that there isn't one already open on the same (or a similar) subject.

Make sure to use the correct type for your issue (`Bug Report` or `Feature Request`) and to provide the requested information. If you have a doubt about which one is the most appropriate for your issue (or if you think that none of these types apply to your issue), feel free to use the general `Other` type.

Providing as much details as possible in your issue will ease its processing.

## Open a pull request

Read the following sub-chapters before opening a pull request.  
Make sure to create your merge request from a dedicated branch (do not use the `main` branch) and to provide the information requested in the [pull request template](https://github.com/Antiz96/oniri/blob/main/.github/PULL_REQUEST_TEMPLATE.md).

### Open an issue first

Apart from trivial changes (like simple typo fixes), it is advised to first [open an issue](#open-an-issue) to expose and discuss your changes, verify its feasibility / necessity and agree on the specifications.

### Coding style

When submitting code changes, try to respect the current coding style.  
For instance:

- Prefer using `unwrap_or_else` for simple fallback / error handling (unless any specific handling of the `Ok` variant warrants using a `match` expression instead).
- Avoid calling functions through their fully qualified paths directly. Instead, import the relevant module (or type) with `use` and keep only the path components that provide meaningful context. As a *general* rule of thumb:
   - For free functions, keep the module path only (e.g. `io::stdout()` rather than `std::io::stdout()`).
   - For typed functions and enums, omit the module path (e.g. `HashMap::new()` rather than `std::collections::HashMap::new()` and `ErrorKind::NotFound` rather than `std::io::ErrorKind::NotFound`).
   - As an indicator, this will often naturally end up with roughly one `::` per call / reference.
- Use `eprintln!` for user-facing error messages.
- Generally avoid executing processing logic from `main.rs`. Ideally, it should remain a "wrapper" around functions called from separate modules under `/src`.
- [...]

Rust code is checked with [rustfmt](https://rust-lang.github.io/rustfmt/) & [clippy](doc.rust-lang.org/cargo/commands/cargo-clippy.html).  
Bash code is checked with [shellcheck](https://www.shellcheck.net/).  
Markdown syntax is checked with [markdownlint](https://github.com/markdownlint/markdownlint).

### Commit message format

Commits must follow the [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) specification.

This project uses the following commit types:

- chore: for internal / miscellaneous changes
- feat: for new features (or improvements / additions to existing features)
- fix: for bug fixes
- doc: for documentation only changes
- style: For changes that do not affect the meaning of the code (white-space, formatting, typo fixes, etc...)

An optional scope can be provided to the commit type if relevant (for instance when a change is specific to a precise part of the project), like so: `type(scope): commit message`.

If a commit introduces a breaking change, its type must contain a `!` (e.g. `feat!: commit message`) and / or a `BREAKING CHANGE:` mention should be added at the end of your commit message (e.g. `BREAKING CHANGE: description of the breaking change`).

### License

By contributing to this project, you agree that your contributions will be licensed under the [GPL-3.0 license](https://github.com/Antiz96/oniri/blob/main/LICENSE) (or any later version of this license).

## AI Policy

I personally do not endorse the extensive usage of AI for various reasons:

- Technical concerns:
   - AI has a tendency to produce overwhelmingly verbose and complex content, as well as unnecessary and unsolicited changes
   - The quality of the produced content often varies significantly as of now
   - It is frequently used to submit changes that the submitter is not even able to understand or explain (partially or entirely)
   - It generally results in additional burdens for maintainers when it comes to reviewing and maintaining the submitted changes overtime

- Ethical concerns:
   - It is often used as a substitute to actual people / human work
   - It generally results in a lower quality of service
   - It is subject to licensing implications
   - It raises environmental concerns

As such, this project will not accept large-scale AI assisted contributions at all.  
For smaller-scale contributions, please take note of the following rules (which apply to both issues and pull requests):

- AI assisted contributions (partially or entirely) should be clearly disclosed
- Low quality / low effort AI assisted contributions (generally referred to as "[AI slop](https://en.wikipedia.org/wiki/AI_slop)") will be closed
- Excessively verbose and complex AI assisted contributions that are judged too difficult and involved to read / review (or that contain unnecessary / unexpected changes) will be closed
- AI assisted contributions that the submitter seemingly does not understand or is not able to explain (partially or entirely) will be closed
- If necessary, using AI to translate the description of an issue or a pull request in English is tolerated (but a clear disclosure is still expected)
- Don't be a "meat proxy" (see [this](https://gruhn.me/blog/2026-08-03/) and [this](https://nomeatproxy.com/))

## Donations

You can also support this project development (and my work in general) by making a donation via my [GitHub sponsor](https://github.com/sponsors/Antiz96) or [Ko-fi](https://ko-fi.com/Antiz96) page.

## Thank you

Once again, thank you for considering contributing to oniri!  
I'd also like to sincerely thank everyone that gave oniri a star, opened issues, feature requests, pull requests or contributed to this project in any other way! :heart:
