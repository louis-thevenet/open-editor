# Changelog

## [2.0.0](https://github.com/louis-thevenet/open-editor/compare/v1.2.0...v2.0.0) (2026-10-01)


### ⚠ BREAKING CHANGES

* streamline editor invocation by making edit_* functions final methods of EditorCallBuilder ([#4](https://github.com/louis-thevenet/open-editor/issues/4))

### Features

* add column support for gvim, vi, vim and nvim ([ad591c2](https://github.com/louis-thevenet/open-editor/commit/ad591c29998f827665b0fc60a3b0c0d6136f2b2d))
* add edit_mut_in_editor(&mut string), edit_in_editor(string)-&gt;string and open_editor()-&gt;string ([96a8a2e](https://github.com/louis-thevenet/open-editor/commit/96a8a2e6dcd54bfc4cf713ec44d1bc528b1155eb))
* add open_editor function to create Strings from default editor ([31e00a8](https://github.com/louis-thevenet/open-editor/commit/31e00a8b1749a96d2ba01a05f56f13838d9eef95))
* add option to wait for the editor to return or not ([0f33b8e](https://github.com/louis-thevenet/open-editor/commit/0f33b8eb0dbe3483d248fe3a5ca4e291707a009f))
* add public convenient methods for EditorCallBuilder final methods ([#6](https://github.com/louis-thevenet/open-editor/issues/6)) ([e87f6f8](https://github.com/louis-thevenet/open-editor/commit/e87f6f89d6e5e99945759530ba221dc82b15cf7c))
* add with_editor() to EditorCallBuilder to set the Editor to use ([#3](https://github.com/louis-thevenet/open-editor/issues/3)) ([e6d7350](https://github.com/louis-thevenet/open-editor/commit/e6d735018ca518f5536b2d58971db16f44c8933b))
* add zed ([#9](https://github.com/louis-thevenet/open-editor/issues/9)) ([cebb7ff](https://github.com/louis-thevenet/open-editor/commit/cebb7ff2b9fac32a2cee23daea719b3d4ceccf81))
* allow using custom env vars ([fbcf850](https://github.com/louis-thevenet/open-editor/commit/fbcf850b5236344813691eab038b784dc5f06825))
* call_editor working ([6bb9a50](https://github.com/louis-thevenet/open-editor/commit/6bb9a5073c208861267e6657ffe0dbb152843f18))
* remove Atom and Mate from EditorKind ([8fd84e5](https://github.com/louis-thevenet/open-editor/commit/8fd84e56ff1fd832b501aa3d16e777ccea55250a))
* support kakoune ([667e0d8](https://github.com/louis-thevenet/open-editor/commit/667e0d8804fd07df56fce241cae81e3397c264ef))


### Bug Fixes

* add wait flag to code editor ([78fa441](https://github.com/louis-thevenet/open-editor/commit/78fa441fe38dcdf8f0d821bf769f24bd97215e69))
* emacs line/column option ([4c29089](https://github.com/louis-thevenet/open-editor/commit/4c290899930c4de6f4cdcc6afb1e62b15136510d))
* fix no wait mode for vi, vim, nvim, gvim editors ([314cce5](https://github.com/louis-thevenet/open-editor/commit/314cce55dee077dfc51da40b312bec6bc27c70ae))


### Code Refactoring

* streamline editor invocation by making edit_* functions final methods of EditorCallBuilder ([#4](https://github.com/louis-thevenet/open-editor/issues/4)) ([ceb6c13](https://github.com/louis-thevenet/open-editor/commit/ceb6c132cdbc6e9e43f45cc8a72d19651c83ec07))

## [1.2.0](https://github.com/louis-thevenet/open-editor/compare/v1.1.0...v1.2.0) (2026-05-28)


### Features

* add column support for gvim, vi, vim and nvim ([ad591c2](https://github.com/louis-thevenet/open-editor/commit/ad591c29998f827665b0fc60a3b0c0d6136f2b2d))

## [1.1.0](https://github.com/louis-thevenet/open-editor/compare/v1.0.0...v1.1.0) (2025-07-13)


### Features

* add public convenient methods for EditorCallBuilder final methods ([#6](https://github.com/louis-thevenet/open-editor/issues/6)) ([e87f6f8](https://github.com/louis-thevenet/open-editor/commit/e87f6f89d6e5e99945759530ba221dc82b15cf7c))
* add with_editor() to EditorCallBuilder to set the Editor to use ([#3](https://github.com/louis-thevenet/open-editor/issues/3)) ([e6d7350](https://github.com/louis-thevenet/open-editor/commit/e6d735018ca518f5536b2d58971db16f44c8933b))


### Bug Fixes

* fix no wait mode for vi, vim, nvim, gvim editors ([314cce5](https://github.com/louis-thevenet/open-editor/commit/314cce55dee077dfc51da40b312bec6bc27c70ae))

## [1.0.0](https://github.com/louis-thevenet/open-editor/compare/v0.2.0...v1.0.0) (2025-07-02)


### ⚠ BREAKING CHANGES

* streamline editor invocation by making edit_* functions final methods of EditorCallBuilder ([#4](https://github.com/louis-thevenet/open-editor/issues/4))

### Code Refactoring

* streamline editor invocation by making edit_* functions final methods of EditorCallBuilder ([#4](https://github.com/louis-thevenet/open-editor/issues/4)) ([ceb6c13](https://github.com/louis-thevenet/open-editor/commit/ceb6c132cdbc6e9e43f45cc8a72d19651c83ec07))

## [0.2.0](https://github.com/louis-thevenet/open-editor/compare/v0.1.0...v0.2.0) (2025-07-01)


### Features

* allow using custom env vars ([fbcf850](https://github.com/louis-thevenet/open-editor/commit/fbcf850b5236344813691eab038b784dc5f06825))
