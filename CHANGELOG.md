# Changelog

## [2.2.0](https://github.com/timkolloch/tandoor_importer/compare/v2.1.1...v2.2.0) (2026-02-22)


### Features

* ignore properties that do not have an FDC ID set. ([46de394](https://github.com/timkolloch/tandoor_importer/commit/46de394f38584e2ba998ea211b7c0f6af3455aa4))
* improved model ([41edd93](https://github.com/timkolloch/tandoor_importer/commit/41edd93deb7939f8e64a2ff97aaba312da7d1c3b))


### Bug Fixes

* keep properties if no ID is in database in override mode ([884ac7d](https://github.com/timkolloch/tandoor_importer/commit/884ac7d570a8e2eed8572fbf3bc498c509b677db))

## [2.1.1](https://github.com/timkolloch/tandoor_importer/compare/v2.1.0...v2.1.1) (2025-07-02)


### Bug Fixes

* Added distinction in property deserialization due to API changes in v2 ([76c7803](https://github.com/timkolloch/tandoor_importer/commit/76c7803b185d86d5e580c8062d6db1e145a14b4c))
* Added support for different APi versions ([d7df1cf](https://github.com/timkolloch/tandoor_importer/commit/d7df1cf8f3821bd2d9b8505760e4873336d126df))
* Handling change in object format returned from API ([7938b2f](https://github.com/timkolloch/tandoor_importer/commit/7938b2fa2f67951d4bdc9b92298e99aa262267be))

## [2.1.0](https://github.com/timkolloch/tandoor_importer/compare/v2.0.0...v2.1.0) (2025-05-20)


### Features

* **main:** Async calls and tasks ([2c93e74](https://github.com/timkolloch/tandoor_importer/commit/2c93e74b86d40c8a4a3b409a35aaaedcf530af7b))

## [2.0.0](https://github.com/timkolloch/tandoor_importer/compare/v1.2.0...v2.0.0) (2024-08-18)


### ⚠ BREAKING CHANGES

* **main:** The program does not override data by default anymore. The user must now run the program with --override set so that currently present properties are overridden

### Features

* **main:** Added interactive mode ([dbcc8df](https://github.com/timkolloch/tandoor_importer/commit/dbcc8dfd475f5c188df61106228f60783939843e))
* **main:** Added override mode. ([4d7b026](https://github.com/timkolloch/tandoor_importer/commit/4d7b026d0f141feb72029680e662ccb0a4c642a3))


### Bug Fixes

* **main:** Directly skipping if a food item already has all properties defined in Tandoor ([360f580](https://github.com/timkolloch/tandoor_importer/commit/360f5806639a741b824263d58db054b539f80e19))
* **main:** Updated README.md to reflect changes in behavior ([025453d](https://github.com/timkolloch/tandoor_importer/commit/025453dbed06bb8009e9191e7238eff4d6f309cc))
