# Changelog
All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

## 0.9.0 (2025-07-06)
- Upgraded dependencies.
- Improved `TextRange#extract` to handle edge cases.

### ⚠ BREAKING CHANGES
- `TextRange#extract` now returns `Option<&'a str>` instead of `&'a str`.
- Changed Rust edition from 2021 to 2024.

## 0.8.0 (2024-02-28)
- Upgraded dependencies.

### ⚠ BREAKING CHANGES
- Replaced all `u32` with `usize`.

## 0.7.2 (2023-01-15)
- Added `crate::extract_consecutive_letters`.

## 0.7.0 (2023-01-12)
- Rewritten `ngrams`.

### ⚠ BREAKING CHANGES
- The return value of `ngrams` is changed from `impl Iterator<Item = String>` to `impl Iterator<Item = &str>`.

## 0.6.3 (2023-01-12)
- Added `crate::extract_consecutive_chinese_chars`.
- Added `crate::ngrams`.

## 0.6.2 (2022-07-17)
- Fixed the edge case about chars on the edge.

## 0.6.1 (2022-07-16)
- Fixed an edge case about checking pattern uniqueness.
- Added `crate::hashmap`.

## 0.6.0 (2022-07-15)
- Added `Match::index_of_patterns()` method.
- Added `Match::value_from()` method.

### ⚠ BREAKING CHANGES
- Removed `*Dictionary::new_with_values()` methods.
- Removed `extract_keywords` function.
- Removed `Match::value()` method.
- `cedarwood::*Dictionary` no longer accepts empty patterns or non-unique patterns.

## 0.5.0 (2022-07-13)
### ⚠ BREAKING CHANGES
- Renamed `Match::tf_idf()` to `Match::value()`.

## 0.4.0 (2022-07-09)
- Added benchmarks.
- Added `crate::cedarwood`.
- Improved casting.

### ⚠ BREAKING CHANGES
- Moved `*Dictionary` and segmenters into `crate::daachorse`.

## 0.3.1 (2022-07-07)
- Upgraded dependencies.

## 0.3.0 (2022-07-07)
### ⚠ BREAKING CHANGES
- Added `UltraError`, `UltraNLPResult`.
- Changed result types of `*Dictionary` constructors.

## 0.2.0 (2022-07-06)
### ⚠ BREAKING CHANGES
- All fields of struct `TextRange`, `Match`, `*Dictionary` are private now.

## 0.1.0 (2022-07-05)
- Initialized.
