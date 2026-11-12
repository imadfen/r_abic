# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Releases are automated: [`release-plz`](https://release-plz.dev) derives every version, git tag, GitHub
Release and changelog entry from [Conventional Commit](https://www.conventionalcommits.org/) messages
(policy: `AGENTS.md` §8). Sections below are generated from the commit history, so **never edit a
released section by hand**; the `[Unreleased]` heading is kept as the placeholder for notes that no
commit expresses.

## [Unreleased]

## [0.1.0](https://github.com/imadfen/r_abic/releases/tag/v0.1.0) - 2026-11-12

### Added

- *(normalize)* add normalize_hamza with the HamzaMethod enum ([#14](https://github.com/imadfen/r_abic/pull/14))
- *(normalize)* add normalize_ligature and normalize_lamalef ([#13](https://github.com/imadfen/r_abic/pull/13))
- *(normalize)* add strip_diacritics mirroring the Mn category rule ([#12](https://github.com/imadfen/r_abic/pull/12))
- *(normalize)* add strip_small and the is_small predicate ([#11](https://github.com/imadfen/r_abic/pull/11))
- *(normalize)* add strip_shadda and strip_tatweel ([#10](https://github.com/imadfen/r_abic/pull/10))
- *(normalize)* add strip_lastharaka mirroring the upstream regex ([#9](https://github.com/imadfen/r_abic/pull/9))
- *(normalize)* add strip_harakat with zero-copy fast path ([#8](https://github.com/imadfen/r_abic/pull/8))
- *(normalize)* add strip_tashkeel with zero-copy fast path ([#6](https://github.com/imadfen/r_abic/pull/6))
- *(unicode)* add crate scaffold, error type and Arabic classification

### Documentation

- *(parity)* correct normalize_hamza against the upstream oracle ([#7](https://github.com/imadfen/r_abic/pull/7))
- require approval before any commit or git operation ([#5](https://github.com/imadfen/r_abic/pull/5))
- rename the canonical ruleset to AGENTS.md ([#4](https://github.com/imadfen/r_abic/pull/4))
- add a contributor guide for humans ([#3](https://github.com/imadfen/r_abic/pull/3))
- add the agent ruleset, parity tracker and decision records

<!-- Links -->
[Unreleased]: https://github.com/imadfen/r_abic/commits/main
