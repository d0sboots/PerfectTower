# Factory Automation Source Code

This is the source directory for the factory automation. This document explains how to make changes, as well as a change history.
To download and use the package, [go up a level](/README.md#factory-automation).

## Making changes

Modifying the code requires the enchanced editor at https://d0sboots.github.io/perfect-tower, because it makes heavy use of the `lua()` macro
to do data and code generation in Lua. It also `:import`s three files that are never part of the bundle import:
`factory_constants.tpt2`, `factory_macros.tpt2`, and `recipes.tpt2`.

When setting up the editor, I suggest creating two new workspaces: One for the three :import files, and one for all the others. This makes it easy to use the
"Export Workspace" button effectively.

Import the main files in this order, so that the eventual bundle export is consistent:

`run_recipes 0 0 15`<br>
`produce 2 1 16`<br>
`produce dust 0 0 16`<br>
`produce plates 0 0 14`<br>
`produce circuits 0 0 12`<br>
`produce misc 0 0 13`<br>
`craft 2 1 14`<br>
`init 2 1 15`<br>
`ui 1 1 16`<br>

## Changelog

### V3.0.2

Bugfix: Fix script getting stuck (and having multiple copies) when exiting
during crafting.

```
Bundle size: 75528   Scripts: 9   Max lines: 16
```

### V3.0.1

Bugfix: Crafting should only be enabled in the Factory.

```
Bundle size: 75364   Scripts: 9   Max lines: 16
```

### V3.0.0

The UI rewrite.

* Editing "launch factory craft" is gone. (That whole script is gone.)
* There is now a shiny new UI, which is navigated with WASD. Items, tiers, and
  quantities are selected on-screen dynamically. The selections are remembered
  across AI restarts by using worker-name storage.
* More robust checking for when Turbo Exec isn't functioning.

```
Bundle size: 75316   Scripts: 9   Max lines: 16
```

### V2.2.0

The "more helpful errors" release.

* Add spell-check support (contributed by cl1694) to suggest correct item-name
  spellings when the item can't be found.
* Add error-handling for if target_type is changed by name instead of by value,
  as well as running without modifying target_type.
* Finally add checking for insufficient ore/rubber at startup, and show errors
  instead of uptiering needlessly.

Also group everything into a package, for convenience and orginazation.

```
Bundle size: 81073   Scripts: 10   Max lines: 16
```

### V2.1.1

Fix a bug where trying to produce ore made producers instead.

```
Bundle size: 68768   Scripts: 9   Max lines: 16
```

### V2.1.0

***I forgot to update the version number in the script, it will show as v2.0.1***

Fix a bug with crushing and mixing dust that would cause various issues
if you were low on dust.

Add "line-height=0" so that there isn't a big blank space in the variables list.

```
Bundle size: 68768   Scripts: 9   Max lines: 16
```

### V2.0.1

Add rubber saplings.

```
Bundle size: 68700   Scripts: 9   Max lines: 16
```

### V2.0.0

Change to using Turbo Exec V2.

All global variables are now hidden internally, instead of relying on turbo
exec to do it (since V2 no longer messes with the variable display).
```
Bundle size: 68517   Scripts: 9   Max lines: 16
```

### V1.0.1

This moves hammers and T1 pressers to tier 2, so that when you craft the item groups you can
immediately scan the entire group, instead of needing to wait on items in the next group.

Also, implement a versioning scheme that shows up in the script names.
```
Bundle size: 69146   Scripts: 10   Max lines: 16
```

### V1.0.0

Initial release. This release is identifiable with plain names, because I hadn't thought to do versioning yet.

The main feature is the much smaller size, compared to Xenos', which it was evolved from.
Xenos' bundle is 309696 bytes, taking more than a minute to import and often causing people to close the program due to the "not responding dialog".
It is also 32 scripts big, and requires 22 lines.

The other feature is built-in item groups, instead of needing them added as a separate download.
```
Bundle size: 69142   Scripts: 10   Max lines: 16
```
