# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased 0.1.2] - 2024-07-22

## Fixed
- SymbolInfo data types now matches with MQL5 Symbol Properties

## [Unreleased 0.1.1] - 2024-07-21

### Fixed
- TradeRequestBuilder can now be used without using the `as` keyword for type conversions in enum
- Fix Order struct data types so that it can match MQL5 Order properties.

### Changed
- Changed HistoryDeals struct to Deals struct
- Change Testing symbol from `EURUSD` to `BTCUSD` so that it will sucessfully passed the test even on weekends
- Change all unwrap() to expect for more clearer error output

### Added
- Added CHANGELOG.md
- Added ReturnCode enum for clearer view on what code really means
- Added PositionType enum for identifying position type
- Added PositionReason enum for identifying position reason
- Added DealType enum for identifying Deal Types
- Added DealReason enum for identifying Deal Reason
- Added DealEntry enum for identifying Deal Entries