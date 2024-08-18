# Changelog

## [2.0.0](https://github.com/timkolloch/tandoor_importer/compare/v1.2.0...v2.0.0) (2024-08-18)


### ⚠ BREAKING CHANGES

* **main:** The program does not override data by default anymore. The user must now run the program with --override set so that currently present properties are overridden

### Features

* **main:** Added interactive mode ([dbcc8df](https://github.com/timkolloch/tandoor_importer/commit/dbcc8dfd475f5c188df61106228f60783939843e))
* **main:** Added override mode. ([4d7b026](https://github.com/timkolloch/tandoor_importer/commit/4d7b026d0f141feb72029680e662ccb0a4c642a3))


### Bug Fixes

* **main:** Directly skipping if a food item already has all properties defined in Tandoor ([360f580](https://github.com/timkolloch/tandoor_importer/commit/360f5806639a741b824263d58db054b539f80e19))
* **main:** Updated README.md to reflect changes in behavior ([025453d](https://github.com/timkolloch/tandoor_importer/commit/025453dbed06bb8009e9191e7238eff4d6f309cc))
