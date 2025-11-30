# AI Scripts and Utilities for Perfect Tower II

#### Quick-Navigation
- [Improved code editor](#improved-code-editor)
- [Turbo Exec v2.2](#turbo-exec-v22)
- [Simple Skill script](#simple-skill-script)
- [Factory automation](#factory-automation)
- [Auto mining script V3](#auto-mining-script-v3)
- [Example miner](#example-miner)
- [Museum combiner v4](#museum-combiner-v4)
- [Auto Adventure](#auto-adventure)
- [Adventure Map](#adventure-map)
- [Timed Task Looping](#timed-task-looping)
- [Autoclicker](#autoclicker)
- [Factory auto-trasher](#factory-auto-trasher)
- [Dust Up-tierer](#dust-up-tierer)
- [Button coordinate finder](#button-coordinate-finder)
- [AI Benchmarker](#ai-benchmarker)
- [RNG Manipulation](#rng-manipulation)
- [Auto Era Disabler](#auto-era-disabler)
- [Infinity phase scripts](#infinity-phase-scripts)
- [Other Utilities](#other-utilities)

This is my collection of AI scripts and other useful things. I focus on low-RAM scripts (i.e. 14 actions or less), so that they'll be usable by everyone who has unlocked the AI.

Almost all of these also while `turbo exec` is active, although with some quirks - for instance, the autominer will cause noticable lag on entering the mine, and mine everything before you can see it happen!

## Improved code editor
I maintain a fork of the external editor at https://d0sboots.github.io/perfect-tower. It has several enhancements, such as improved loading speed, better responsiveness when editing complicated scripts, and support for enhanced macros and syntax. You can see the full list of features at https://github.com/d0sboots/perfect-tower.

It's kind of a pain to move source code between editors, (as opposed to importing compiled code, which is easy), so if you already have a lot of work saved on the base editor at https://kyromyr.github.io/perfect-tower, it may not be worth switching. If you aren't using either yet, you definitely should use one instead of the in-game editor, though - the difference is night and day. Also worth noting is that compiling the source for many of my scripts requires the enhancements in my editor. (You don't need it to just import them.)

## Turbo Exec v2.2
With the release of TPT2 v1.0, **Turbo Exec no longer works**. Any script that requires it is outdated. You can find out more at https://github.com/d0sboots/TPT2_scripts/blob/main/common/turbo_exec/README.md.

## Simple Skill script
All this does is use the first active skill, constantly, as long as you are in tower testing.

`D0S.SimpleSkill 2 1 5`

```
lZFNb5tAEIb/i6++wBIqb6UcAIeFBCOg5rPqYWc3MtBdjIwBQ9T/XmG3UizlkOxpdt7RzDvPvK06dqrac7f6/nO1VcIxnvQ9TdURSLKnSHRguIZhGIQj+1RkybhNeQlp3INMNCaxCjI0l4IndMvTzI9B44KJSDAtakEye9G3kguQyUTTXV8grAAS0iL+AI2v5KmuGHK3lEU5wmdG7Ilm/gCVPnPneekRFegy5NLuvMZXmBR9MelNkYVujnDPtKiEJnyy6stMs1YEe2PNlO5OM2oFv59nWseNp/ntq4zXwZIneAoyfqRZJALhn2h66dhkTjzVe5rqTWDhGtBlYPXDR17mr3h5qceBIfGt2D9scoQ70FzMUdlyEq9jdG6BXGbXeVZB6svuGw/981dy5cq20s+AIhVI3PPMFzTVZ07snhtlDeh/7Jqf4n74kPtv0JIKSBxygrUic3tKkhKc6OhJtXy9mxOaweH9/9rv9ky7AYKrPB2X3aeFo9f44t5n+BI41zvg3Vaff4x3mhNY+Mbx8Pi4+vXnLw==
```

It's not hard to make this yourself, but it's a bit tricky to get the conditions right so you don't
get extra copies when clicking "restart," so here's a canonical example. It's
straightforward to edit this to change or add more abilities if you need to.

## Factory automation
**v3.3.0**

This set of scripts manages auto-crafting of everything in the factory. It is an (extremely rewritten) fork of Xenos' factory package, with 1/5 the size, only two scripts, budget support and scannable item-groups built-in to the base package.

`Import code:`
```
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
```

┌─────────────────────────────────────────────────┐<br>
│ The list of all valid items can be found in the source code of [factory_constants](factory/factory_constants.tpt2). │<br>
└─────────────────────────────────────────────────┘

### Details

To use this, import and enable both scripts in this package. (You can right-click a package to quick-toggle-enable the whole package.) Then go to the factory and use WASD to select a quantity, tier, category, and item.

The available categories are Producers (PROD), Machines (MACH),
Misc Crafted (CRFT), Produced Parts (PART), and Item Groups (GRUP).

Press "F" to craft when you're ready.

#### Basic Features

The package will prioritize using items already present in your inventory instead of crafting new ones, and it will always leave at least one dust of each tier.

It can manage ore processing, dust tiering, and everything related. If more dust of a certain tier is needed then there is in the inventory, the package will first use all ores of that tier, then use dust of the previous tier. (***This package assumes you have the chemical lumps exotic skill***; if you don't have it, it will likely get "stuck.")

You can upgrade machines with it, but the process is slightly involved: First, take the machine out of its slot. Then, start the upgrade with 'F'. If the machine is needed to do the work, you will then need to put it back into its slot, and take it out again at the very end, to finish the upgrade.

It uses a different up-tiering mechanism than [the uptierer below](#dust-up-tierer).

If there isn't enough ore or rubber, the package will give an error message at startup. (New feature in v2.2.0!)

You can safely leave the factory and come back while the scripts are running, the craft will just continue where it left off.

#### Improved Features

"Item groups" are added to allow quickly crafting large batches of items, for scanning in the Crafter. Some of them take a lot of space and/or time, especially the "all"
group, so use with caution. (Maybe try out the narrower groups first.)

This uses the names of workers to store state about what item/tier is
selected, so that it survives across restarts. By default, it will overwrite
the first worker name, replacing it with "[factory]" followed by some numbers.
If you want it to be in a different slot, put the "[factory]" tag there and
get rid of it in the old location. It also won't ever overwrite a location
that starts with a "[" (open-bracket).

If you assign "Adding items to the dissolve queue" to the worker that the
factory uses, then it will auto-pause that worker when crafting, to avoid
losing items. Otherwise, it's just a regular worker with a funny name.

## Auto mining script V3
This script mines across all tabs, in 1 frame. However, the amount of floating text often drops the framerate lower, such that it takes longer. Turning off floating text (shrinking it to 0%) will greatly improve the framerate; switching to mine floor 2 will improve things even more. To use, just enter the mine with AI enabled, or turn on AI inside the mine.

`D0S.AutominerV3 2 1 30`

```
xVhfa9swEP8uHnSwpa5lh3aEJGxl7Gl7GgzKUoqsyImoIxlJJuvafvch2WlT+9TITtK9mMS6O9397q/vPlBEskKrYPT7PsBEM8HN72BWRlFEc0FwHqZC5KGiepYSwZXGXNvToX2mainKfH6TC1E0CJB9RsGglragnEpGwoXQgmUN4tg+zyuW1jNTWjK+CIngGjOuKoGV7iFbFWWuKKhe9jX6GX4ptVgxTuWv5EmZHaZlGZNK31Q3gHYhI0pWYhjXToCSQqimrVdtM420bBshEJ8UZvRRQ+OKOVExm2lVphWk5h1Ci1ykOA9rlBcuX48V+0sn0fTz6gOoHgLcB9HFnnSAvC6oH9vc4WHNRe24B8LCaqap5FjeGQgsZ6zFmodrxudiHTIlCspBk4YmDUARESViVWDJlOD23RbILoBsTEEH8XjSMjDzBAsIcYfCWynsdOF2deol5bVCEM+0p2exJx3tT9elDkC4O4KvZ4U7hEjrHiyZXq6oZsQ7KtvqVAfotEuRKf1aFpS0vfPJpXk8befTcXzmqkHNVAF75ibZ8B2V9l22hYFhO3yhcQCzuxHV/CcnlU6mLoZLrL4b3VXXntonSl0moY9woTo6lmjao2bXMD48+BXTFxHiGdGdk3ZHmPIlVjdpOTcKQhi+SPyazg+v+X6VJHEgAHt+B9TbVr4a+W/f9mtMmmOY8wtjewwzL86J4ATr1u+ExXDT2O0mVyb6ovEfr3/qlJuC8TzovkH97TXodSsa5etfmcP34zMTINMxEbmQk3cXl8m3KJr+YNxEVRiOz+xBM1srwRc1s9+cfVjsJt5NHSwX0NfH0ZX2bBKwHvs5ev9J+mDoOOc02KXOGbH718iuBoo+eeIDbVogOuKs39FTvtA/lJQaXMCgaARsYHYuOpCjlaZ2TjPB6RnEe90m7W2cri3ce4mq+l3NvMZMZxKvqN/SB3DVMxZztgCbzj5hnKzE3HvhcejL0Znv1b41u/ek5F5IdHBT30mnSSARSlAUbfrV9SAggs/Zi41ttz2QkVHvTq2ENb6lZREMAsMWbii4idNR0M7iApNbvDBnwSCo8AtGp2gQlIpe1n+1LOnj9eM/
```

You can also change the first line to make it run infinitely, instead of stopping after mining. Here is an import code with that, for your convenience:

`D0S.AutominerV3 2 1 30` (looping version)

```
xVhfa9swEP8uHnSwpa5lh3aEJGxl7Gl7GgzKUoqsyImoIxlJJuvafvch2WlT+9TITtK9iMQ6ne5+91d3HygiWaFVMPp9H2CimeDmdzAroyiiuSA4D1Mh8lBRPUuJ4Epjru3u0K6pWooyn9/kQhQNAlStwaDmtqCcSkbChdCCZQ3i2K7ndo1aa6a0ZHwREsE1ZlxVDCvZQ7YqylxRULzsa/Qz/FJqsWKcyl/JkzA7VMsyJpW+qW5w6yUrNoxrJ0BJIVRT16u2moZbto0QiE8KH/QRQ+PqcKJiNtOqTCtIzTeEFrlIcR7WKC9cth4r9pdOounn1QdQPASYD6KLPekAfl1QP7a6w8Oqi9p+D7iFlUxTybG8MxDYk7EWax6uGZ+LdciUKCgHVRqaMABZRJSIVYElU4Lbb1sguwCyPgVtxONJS8HMEyzAxR0Cb4Ww04Tb2akXl9cSQTzTnpbFnnS0P12XPADh7nC+nhnuECytebBkermimhFvr2yLU22g0y5JpvQrWVDQ9o4nl+TxtB1Px7GZKwc1QwWsmZtgw3dU2m/ZFgbm2OETjQOY3YWoPn9yUslk8mK4xOq7kV11ral9vNSlEvoIJ6qjY4mmPXJ2DePDg18yfeEhnh7dOWh3uClfYnWTlnMjIIThi8Cv6fzwmu+XSRIHArDld0C9reWrnv/2Zb/GpNmGOV8Y222Y+XBOBCdYt34nLIaLxm4zuSLRF43/eP1TpdwkjOdG9w3yb69Gr1vSKF9/ZQ7fj8+Mg0zHRORCTt5dXCbfomj6g3HjVWE4PrMbzWitGF/Uh/367MNiN/Eu6mC6gF4fRxfas0jAcuxn6P076YOh4+zTYJM6e8Tur5FdBRR98sQHmrRAdMSZv6OneKF/KCk1OIBB0QiYwOwcdCBHKU1tn2ac09OJ97pN2ts4XVu492JV1bv68BoznUm8on5DH8BUz1jM2QIsOvu4cbISc++Bx6EvR2e+V/vm7N6dknsg0cFMfTudJoFEKEFRtKlX14OACD5nLya23eZAhkc9O7Uc1viWlkUwCMyxcEPBjZ+OgnYUF5jc4oXZCwZBhV8wOkWDoFT0sv6rZUkfrx//AQ==
```

## Example Miner
This script is very similar to my other autominer: It mines all layers across all tabs. Unlike the other script, it is very simple, with copious comments. You can use it as-is, but it is more meant to be an example of how the AI and the external editor work. See the code at [D0S.ExampleMiner](D0S.ExampleMiner).

`D0S.ExampleMiner 2 1 13`
```
5VVNT8MwDP0t5MCFUjXj4zDBBcENThzphNLU7aK1cZWk6iq0/46SDq3QdGoHNy7WVsfv2X5O/EE0V6IymizfPgjjRqC0v0lcR1EEBXJWhAliEWowccJRasOkcd5rZxO9xrpI3wvE6scB6mxEAhKrDklIMwp0ZVjSsTIlzLoEI7gN8H7r4eXH8LyOEtMfjkWcdbl+s75gejEI7ZXat7bspBQSQpvIxIRtUJaDBCV4mKNBL9utn005NglNwVpQB/5U5FP46XbSqXainnT7l2rS7S/lmD4KB/egwy75vjwimypQF8uxrJgSGuUvC1+c3XuZh5xT7x9t/1Sv9r/q5S98pl6+xN1ra0BJptqDNO6Gr5l+tndeezluRpiGYPPLHX1l6d0Jb+xitDNTsOoooguDjQwbIVNsQqGxAulN79r2bd9pSr863TBhMsVKmDM8vuG1Nut1067QDrC3Usd6emSl7ofp/PyEYlcB4ShT8W3Hz8cQZVUXGhxCwzZQVyQgNiz8OiFtA5fkMXoNn7asrAp4EdItpIrxDcutkwQkqdMcDFle0oDUGh72f42qYbfafQI=
```

## Museum combiner v4
**v4.7**

This package combines museum powerstones, *really* fast. It is very close to
the theoretical maximum speed. On my machine, it can upgrade a stone 11 tiers
in 1.25 seconds, and do the entire equipped grid in ~2.5 minutes.

**This has been extensively changed for the museum update in 0.11.0.** It is
also slower than it used to be, because the new, larger inventory has made
combining slower.

`Import code:`
```
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
```
**This requires `turbo exec v2.2` to function, found here: https://github.com/d0sboots/TPT2_scripts/blob/main/common/turbo_exec/README.md**

### Details

To use this, import and enable all the scripts in both this package and `turbo exec`.
(You can right-click a package to quick-toggle-enable the whole package.)
Then go to the museum and use W/S to change your *budget*.

The budget is (roughly) how much will get spent to upgrade every stone in the
grid; it's not what will get used every time. A good rule of thumb is to set
this to the amount of Museum resources you have, or maybe a little more.

Press "M" to start the combiner. (If you don't like these keys, see [the
modification instructions](/museum/README.md) for how you can change them.)

**Once you understand how the combiner works, switch to the "Artifacts" tab
while it is running for best speed.** It runs ~11x faster without graphics
(i.e. when you aren't looking at it) then when you are. You still need to be
in the museum for it to work at all, though.

![The combiner running](/museum_combiner.png)

### Features

The combiner tries to upgrade every stone you already have in your equipped
grid, while respecting the budget. If you want more of a particular type of
stone, buy it from the basic market, and it will then get upgraded.

Internally, it's allocating 1/100 of the budget for each stone. (So if you
have very few stones, you can increase the budget a bit.)

The "preferred tier" of the offshore market will be automatically set to the
best value for your current budget. You can manually change it, but it will
get overwritten when you run the combiner again.

Once the combining is done, a countdown timer will show, synced to the
offshore market's refresh. If you let the combiner sit like this, it will try
to combine once every hour (or faster, if you are boosting the museum), to
take advantage of new offers in the offshore market. If you don't want to do
that, you can press "M" to cancel, or just leave the museum.

Pressing "M" will also cancel a running combine. You can also exit the museum
or turn off AI, but pressing "M" cleans up the inventory so you don't lose
stones.

The script gracefully handles running out of resources and offshore market
rollovers. (In both cases, it continues on as best it can.) It also handles
not having an offshore market at all (it will just use regular stones).

This script uses worker-name storage to hold the budget even when AI is turned
off. This will overwrite one of your worker's names (generally, the first one).
If you want to use a different worker, name one of them `[museum]` and delete/rename
the old name. You can also manually change the budget by editing the value stored
here, which can be faster than hitting "W" or "S" a lot of times.

### Why are my gems stuck at T12?

The combiner can only increase tiers 11 levels. This is a time-based
limitation: Every additional tier requires 3x as many gems, and thus takes 3x
as long. Beyond 11, it starts taking too long to be practical.

Thus, without offshore market the maximum tier will be 12. With offshore
market you can go higher, but only when useful gems appear in the market. It
may take a few cycles for that to happen for all your gems. (You can influence
the chances by enabling/disabling specific elements in "Preferences" in the
Shop.)

### Transmuting

The script won't transmute for you. With the new museum design, you only need
to transmute once, to unlock universal stones in the market. Once they are
unlocked, you can adjust preferences to get them to show up as often as
needed.

### Troubleshooting

The script will show some error messages for common conditions, like a
bad/missing Turbo Exec installation. Other common issues:

If the script finishes instantly, without ever doing anything (even though you
have budget set plenty high enough): Is your Museum fully upgraded in the HQ?
The script requires all the inventory slots to be unlocked.

If the script hangs in the middle of combining: Have you bought the "Quick
Combine" skill?

## Auto Adventure
Automatically plays the adventure mini-game for you. For historical reasons,
this is at https://github.com/d0sboots/TPTAdventure/blob/main/README.md

## Adventure Map
**v1.2**

Draws the entire 256x256 adventure map at once, so you can see what you've cleared and what you're missing. This is somewhat obsolete with the in-game map existing, but it draws it larger.
![fullmap](https://github.com/d0sboots/PerfectTower/assets/459704/917ba9b3-5464-4c55-954a-087844b5372d)

`Import code:`
```
7Vvdbts2FH6VQsFutoQhlawdAsfd1mJ3wy6KXs0dRlO0Q9QiBYqynbV9hz3CHqHvkBcbKEqJLJMxRStxGttI5ISijs7/+Y5IfYpyIlmm8ujiz08RJooJrv+ORpMp5VQyAqZCiVEBIRwrKjmW14BxVQ5MiEgzLFkuOBgLMTODuZKMTwERXGHG83KQmrsAlmbFLKejMRE8V7iic66PKH4L34G3BZ9SwX/H2QsEYts8GH/+XM5HjbsnohjPqBnGkqmrlCpGmsNQsZTmCqcZKBThYmGljU7Kr7M8TkYqL8ZGFkN3OhNjPAOVeFOq1ijIQc7+oZdw+BdO5ket03F5hI4jrcgyntDlH5Ne7liJNPBmZHXe2b0Mb77ecXsn2Zt/jRu8+8XKsIvtda+0OGBPunw5IGImpI0/ZOPPJseP6/Oi48hwXoccXVJSKO8wqa5HqL5+gZmaSJxSfWY1kD1VW7NEZ4LgGZhTEoPcorRSopxgE2WQzikBEynSN0LIJHcFJDffIGW8MhmRlHJwRdn0SoHkNmfowQVL1BVI7N51GuK02c9PlrEuen8pJhN9roPiJ5V8UhQ82agEvNw6Jbgzsp8aT+xmsM79PoTbm/9e26n90Kb2vvx89/pb0K2fwk787fBIur3LZCvp2hYBrXRdBgQRnOCKZBKbKNhY+M8Hp5rS0OT2y6Pfys+wSrEvdI5NJF5oPgAYnJazhjWjkGA+xznQM4CkRK1Fr63UtKN30rCivsibSJV6HSazJYawdBmcZCGty3MDShEsCU4owMmcclVIClj+RqTZjCqaeMt+LoVIrWdeHUH4kwuQ6JONCoeguVUVl65ce844kasCNTGnHYquknbKUZK2It3Ly54xmdf1fzvLYx9CBaeSLqXR+EZP/r+tPptHLYQ0AjCu7oFUCquqTDbMUENc37E2Nn7AyDM5C23UmG/3cVspAuUITqnutFIxY4Na4UGxrQe6m6Pnrvc+4FO43eLG+f3Se1e9bavncbiO19tfV6Y8y2PWfOJj67xhDBFCCJY/CMG4mcndCK3M5IHJkvj7TfsO0q4ON2gN9qAayd7jQ1ba+5IqVvG4P6rvprgdqzgkK1gQZrCl2j1KU+k7sF24c94+ZvAbX23lW6S9HkmUIVzKtHwwn+lLfy6d+ArabbhSy3VvUdOXI3blK0ypxQ7bd2elq3v7va1lhf0ZxPqyxyY0o3FMCWTMN3xwNFOJtf06U0Ct6UlltdLM7zNW2X4imN4CC9WBhR41sJ4NCOvRFvCuZYMHWxwA8cPhxGWPuelQ4HuFyv6medb9xwEFHVowL3fp0oJtuSC3+2z77ZTds1S092dsueHpsbJMt92ae2zieBi05u+28FMTM2jJfPNmZIeD9enc3jtZW4axFT3bolYHsbavYsMQYW6+ttF2PFKWyRaJkaXMP61neGi8VlS1L1ibkt09EAngsrc1sXXzB17WaxPp2eltXON4DPu7uLreKVd2kGf2QW63I9OGXZ/wZrn+3hR4VW0qjT4cR0TwhN29WFNu4ldioXfc8kQsAMtFRrk9lRnjayrVqzMljY/0GmR6kOs9/heR9ZWADJOPeKpPR8fRuEimVEUXJ+g4KnL6a/WvkgX98uHL/w==
```
To run, go to the Arcade and press **P**. It requires the compass and map to function. If you don't have these, all you will see is a black screen.

If *nothing shows up* when you press P (and the AI overlay is on), you may not have the `boots.d0s` special software. You can acquire it by turning on the debug overlay (F2) and then crashing the AI. The script includes a "crash the AI" function for this purpose - press P twice quickly.

It should take a second or two to draw the whole map. Much longer than that indicates an issue.

## Timed Task Looping
Do you constantly find yourself forgetting to check the Trading Post, or the Museum? This is the script for you!

Press "t" and a customizable set of tasks will run. After that, a visible timer will count down (default 60 minutes, customizable by editing the first line) until the next run through of the tasks. The tasks are set in `D0S.Timer_Actions` and by default it farms the Asteroid Mine and upgrades Powerstones before returning to the Factory (assuming you have the relevant scripts installed), but you can customize that to be anything you like.

Pressing "t" again cancels the countdown, if you want to stop looping without shutting down the rest of the AI.

`D0S.Timer 1 0 12` `D0S.Timer_Actions 0 0 11`
```
7VdLc6s2FP4v2bYLLEwndOYuJAwyxOYaYiNQpwuEEmMsEd9rg407/e8dkcQxOJkkvdPpovXCj+PD0Xl+39EfV9vs+2qz2179+tuVtQj2i8ZYpGQgsqUNIYROCiJjUgbqOxypN4S2DPg5s4w106MVw4sqA5FmYb9mpa8lxNAQPGwZMPcxGGxYGWk0nroJMKtMD3NWBq0Z9Zqv4VcKoorG3iZpjJLpYZ2S5FzXsvbqTKdgYLBPYm+TAbOalL6WSVHRg1MybK4Sslf6TUqMciK54DAvGDCOHDsVh8FyNvY3d3JhTgv400z5ic1mFvOHNA7FTB5qpqN7jsWOxu752TZc2WY3LhgwbBbqPIrNQSIPYlL6/fPGrS1iDjg0NBp7VRJ7xkSGNSfeloa9WE7yy7zSi7xyTdWG6X25a86Ccx+m8OU1aJ9pY4phxaSp28Rp0jh8YCTSUjI9xWLjQ50AZ3vy6dYoaRx0coL0cMNIVPOeHN5uQxUTx/mOxqGq53Mclzm7rNGUYbPOHHOXEmPAcXTsxmd/O/8NVU9AqxhmDrIJ3zLgqbwd+djbMJld9qMFrzMgfqFzrZmOhtcJMLdMd00O8g3Hi5+ezn7M05Iqv4okDrCKLwGOhqRfM+nnHJ7+G9GRdxzhU55kayN38kwKLcUDwbEozmr+SfmP9IK9G8VtjVRud5llVEzP+rW6nEcf3iaa3dO7vtS7meX7RU9v/tDm462++k/myZ3Hoq8HBjWFr85pEkB/PS2nb83Q9GV2qMLW5s0Z/qz8h2Ze8x9xU+V0sJ9Io+b7zlyjm/1lvBryiiDo6k0+pjdixFxfYoGCgxB263t9PQFPWF/s68f5H077fGPpocikkWflukqJodnEr1mMVL8dGRieeOU1nLnotU/hTCcuazbXurnt9Ik7aUN8x9fTvL3X1/PhRzD5jMc6vuDWl/MaLNv9IODY1NnK0FOi+lbhcF7zvThy4j3nrOOHHjga07nIylDxk/p/6Vpw6Y7DJiHqN98z3ddamQWXGTZ1Gnt7pvx4krkWkgnxNaZ7hnuSwaWLD3kizSaJwzor1yd5BvJNNhZ5JoMXXQvpTPe+K3+zjg2nSYCzprdnMgsuWRwdaRztOudZ6IESZ53FUd7GU06fbDzmKSujgmNRM0k3mdSWKgd8HAkGaMGJV2dNa0Pxn+jahUsXGj38E1WvJmG/JoozR+R5tkXFrcu5hmT9tYc3G1YGX192KVFRHA3nIPm/bn+rbl2M+wfr2Mfl1+ra2avtdmR7WL0cPPOp0n+FS51NZ+4D6D/jiMKjV3ZhlAavxfya7HNnI2h/72JQe195h2/dNuqR5ILJqGl5EJgaA0J2bSVK7WsCzF2GFW/6NVu1fnXjvcRMlHb5ynK7uG61XIX8khJDZFLFHg0p8dud944YRSdG5NhTZ1pFLcd69wHxtZSYVbY3zp7nNcfmhnb4xG13kH+Ly9wLLjuc7mmcGMd+/7qjwzGNN2I2d/ez4lAwcKizYqCnWGh0bjzeJR93kGaET74P2GW/feau9+Xq5yv7/MorTJSoY4BxPLs/tdRrg7OWiXOR6Opa6xtMdq629o01/DZZPVQ3q/ZzGcioTMlwmTyXrkEPNPaEa5101Of8bC0RdzgqeKygVVRJd51iAchzJrlwsVPd3aKarpCW4uhIb5HGGqSn2NE+6us4FjW/Rboq98RCgsuocrGp7DUudh5hr5xW9jisORhWHCg6MPVJ6T/0VwqEBxsmF+cyBMPPtXkQ++q6XKckvA+l07AQKZqpJiXfMLWeNsYxxabehYGwhd4eLL2xinRyiZ/poPus/dFaLOZPz8eaM+DYvI9iVGbSWdPY/YHzP+T7mEpVH7O568CNjT7cp6tOn1Y3K+QwGbzXp98myy9frn7/8y8=
```

## Autoclicker
A small, simple script that spam-clicks where the mouse is pointing. It uses the budget system to click _really fast_. Press "c" to enable, and "c" to toggle it off again.

__Warning__: This is __not__ compatible with turbo-exec, if you try the sheer number of clicks will lag your game to heck and also the toggle won't work right.

`D0S.SuperClick 1 0 4`
```
lVDLasQwDPwXnV3j7vaUY9s/2GMTiqMoRsSxjB+Hsuy/F5sGSvew9CKkeYhhrpAxcSwZho8rWCwsoe0wVmPM6rzM1utZxOtMZZxRQi42lE6/jOsiHNwnesYtd+wcpNx73SMvKBiTo0CJUXeoq2iXmklHydySNVWHD6WTIrz+eX36Nc3d/H+uw1HDwwpgUoASFj56nBTwHqvP1Fvd6EtjEwW7Ewzwbi76UiOlt2Y/gYJocbOucaBgroujAsPTs4Ka6fXnLKnSbbp9Aw==
```

## Factory auto-trasher
This script trashes unwanted items that you pick up (from the mine floor 2).
It runs automatically whenever you're in the Factory, and pauses itself when my [factory automation](#factory-automation)
is running so that it doesn't get screwed up.

Turbo exec v2 is not required, but it will speed up trashing greatly.

The [download and instructions](auto_trasher/README.md) are on a separate page, because there are a couple of different options
depending on if you want to customize it.

## Dust Up-tierer
This is a <s>simple</s> ore crushing/dust up-tiering combo. It works stand-alone, but pairs nicely with other factory automation that expects dust to be available. At __10__ actions, this should also work for everyone. It tries to crush any un-crushed ores, and up-tiers dust while trying to make the dust distribution match its built-in target ratios.

`D0S.DustManager 2 1 9`
```
7VxLb9NAEP4rwRckCsZ22hRVVM2NG/wAgirb2SSr2l7LXpPyEje4wZlbuXIAbnBDiv8YfqV1nNl2nIdjt1ZbJ1p7vbPfzHwzO2v3neSbHnW5Lx29fCfpJqfMib9Lg0BRVMVipm7JQxYYFpF9wgeGGZ3nusPj88r+wGEeOTWC0Yh4hXPd5KiIjv3Z1770cGEYn3vUGYPDJFcNA5+f2oHFqWtR4vnFq+6rHbUj97SDjhz9KodPDrXoo9dV4w81+1M0tTBsbnbxbXrRTU0dFEHpZTNN5kc1P/lCdI/yiU04NWWaXV5s89Khoq/yGJ6eolK4+XGhWUtlv78AKdhzD+qpqAKNdH1tOOB+YKRqABQjkBxQzGYh6NpsiAFBqIxMfuoMyfmL0YKW15klOIWOqHk3mIC3fFQLYW4giPQY/v0Yu2tONgE9xLJV4I2gTyG9Mf4wOPEc3XtzJZvJbDeSz2fOdQa7aSOtgwGkfbXj42LX8F/6A/KXyFQwim/ZYK0wUScyyYK4QsbEIV406phxRkfgXERMU/S9NXjhaXHg2cXsUx5IJIsdRA2clDPozTBeNjI+B1EMrIk8WK87zJxqbxSlqyxqsHVzQh0ix9nra3gGh6YX+JMsc1vrTgc2PV/KdcXMhJ04KJeas880S02au3NxKSe2L5sscGCVdqPkfEkCBwm79vR44GS5sU2dPDdmrZAl5dcDpWTdjwkVJWx1UPEmQQUJWx1UWGeuBVS7dcCEj5oEl0jgaiHrNQ0ySOBqITtoGmSQwNVCtt80yCCBq4UMyq5rDZloOVAdZFC+WGvIRAludZBh16O1gUxUZ63j6mM1wLG2jKUJLANjgxs2b8CmZJgsF7NowK/BoFqIudx3vqGhzmshU53y6YRaWSEBKjGOLWbcUBEgmWGect0TXaPdW6reiawxqYDMbd309JHAo6zAdtPxC0WMtcpUe+AtsW27rZ8sV+iwXcFimmilSObMdEXcyihnOwZjltigNoqYqAiIxqyEXcL9379PXGptckbtciwci9FLzSGbaaYwxN7scyps+C38Ev4Jf52I6nCC+wXlQx7WQbGpTuug5etOXZdNi7YQfn+bqHN/9vNZ+D38cXKdzrfYDLIJhjhalq/WiID9NyTm5amtHM9cJgKHsUZs/bxUno2xK2w0aO2qvF2lmG0rrLSMsEXNpXnC8qo5Prn241FtklCxNpMN+h0G71a5bfDeRvBuQ0BTg3fLCLcpeLd+uEVtPk60KQjerZJvh5L3REpeIWR3+jWhhZbkt2oxK5hGeHEyf2p2NA/srseGgVlI+q/eALkLpX5gy7mYw+Ke2m8uLPFxI3l6hXPG8lAbuTbDQwsMUbIs0CBXuNx9izwh2MBuWcIuTfSva2pJ5Z+n2UxuE/4WbGwKE6hWg7AGV1jY34rZF2kWVwUz+iVBax9h2XWAulsp7OXrTaDgyWMpZd6XEzy5rmqcTR15SqMYOZUjPF3iwOJkjiK9eihFp4d08b8KrHIbaruB5ZPkJlP9jARuNKG4p5y7yNFtIh1Jz8m0E7/k7+rmmT6OW6QPrz78Bw==
```
### Details

Edit the first line which sets the variable "ore_buffer". The default of 1000 means that 1000 of each ore tier will be kept, although initially it will be crushed so that you have dust to work with. This buffer amount is so that you can scan it with the Crafter. If you've already scanned all the ore, or don't have access to the Crafter, you can set ore_buffer to 0.

You can edit the second line which sets the variable "dust_multipliers" to customize the ratios; this sets the shape of the target distribution as a space-separated list. Each number is a multiple of the T1 dust amount, so the first entry should be 1. The default of "1 1 .625 .5 .07872 .06312 .01 .01 .0021 .0021" is based on making high-level producers and should reasonable for everyone.

![The script in action](/crushing_in_progress.png)
This shows what you can expect after the script has run for a while. There's only 2.75K of the low-tier dusts, but 138 of the higest-tier dusts. This matches the ratios from a previous version (138 is 1/20th of 2750.) When you first start it, you might have millions of T1 dust - don't be alarmed if it drops this to just a few thousand.

You also might notice that when you first get a new tier (say, the first time you get T10 dust), your dust levels will drop suddenly. This is expected: the script cuts off the tiers you don't have (since you can't uptier to them yet), and once you gain access to a new tier the rebalancing will give you a bunch more dust of the new, higest tier, but ~1/4 as much dust in the other tiers.

This works equally well with or without Chemical Lumps, but if you don't have Lumps it will undershoot the buffer by 4 dust (but it will never consume the last dust, no matter what). It also may be good to delay wholesale up-tiering until you have Lumps, since you're operating at a significant loss until then.

If you want to manually change what the factory is working on, you can shift-right click to remove the current lump from the machine. Due to an intentional race condition, this will usually cause the script to advance to the next tier.

## Button Coordinate Finder
AKA "Click Trainer", this script tells you the positions of buttons on the screen. It's better than the F2/F7 methods though, because it outputs code that is
resolution-independant: It will work regardless of the user's screen size (with some caveats).

To use, move the mouse over part of a button you can find exactly, and press "p". Then resize the game window, move the mouse back *to the same pixel of the button*,
and press "p" again. This will give an initial estimate. Do this a third time, and you'll get an error measurement, telling you how accurate you are (in pixels) in locating
the same part of the button. If the error measurement is 2 or less, the numbers are likely accurate enough to work with.

You can press "r" as a quick shortcut to reset the measurements. Toggling F4 also works, of course.

**Note: There is a more advanced version of this available at
https://github.com/cl1694/My-TPT2-scripts/blob/main/Miscellaneous/ClickTrainer%20v2/README.md**

`D0S.ClickTrainer 1 0 13`
`D0S.ClickTrainer2 1 0 11`
```
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
```

### Example
Here's the result after pressing "p" four times in various resolutions, always pressing on the labeled corner of the workshop resource icon on the upgrade button.
![Using the script in the workshop](https://raw.githubusercontent.com/d0sboots/PerfectTower/main/click_trainer_result.png)

For the X component, the coefficients came out to 0.5 (exactly) and 0.0249. All numbers are rounded to 4 decimal places, but that's more accuracy than you typically need
(or can achieve with the mouse). In this case, the offset of 0.0249 can be dropped; omitting it yields a spot slightly to the left on the button, which is fine.

For the Y component, the coefficients are -0.0008 and 0.6291. The first is actually 0, and the 2nd we'll round up to 0.63. This yields a final expression of
`vec(width.d()*0.5, height.d()*0.63)`.

Note that this is only half of the equation, specifically the widescreen half. Every screen scales differently depending on if the resoultion is wider or taller than 16:9.
To be properly resoultion-independent, you must capture the coefficients for both the widescreen layout and tallscreen layout. Also, it is **very important** that you
reset between the two types of measurements and do not cross between widescreen/tallscreen when doing consecutive measurements. The error will help tell you if you mess up by
turning red.

The final equation for a button click should look like this:
```
click(if(height.d()/width.d() >= 0.5625, vec(_tallscreen_coords_), vec(_widescreen_coords_)))
```
## AI Benchmarker
A script/execution suite for timing how long various things take to run. Using this is a bit more involved, so it's moved to its own subpage here: [benchmarker](benchmarker).

## RNG Manipulation
I wrote a library to help with RNG manipulation. It includes a couple of examples, one of which gets 5 stacks of Rak's Curse immediately. More details at the subpage: [rng_manip](rng_manip).

## Auto Era Disabler
SPOLERS BELOW FOR ERA PHASE! YOU HAVE BEEN WARNED!
<br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>
This script automatically disables era powers and upgrades era divisors, as
soon as XP is available to do so. It uses a configurable order of era
disables, so it should work with any blueprint.
```
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
```
The order of disables is set in the local variable `element_config` on line 1.
Each element is separated with `<`, and the default is:
`nature<elec<dark<uni<fire<light<air<water<earth<neut`

You can use any unambiguous prefix of the full name to specify an element. So
"electricity", "elec", and even "el" all work. Capital letters won't work.

The Era Shield will be disabled after the first two elements are disabled,
since that is their natural position in the cost hierarchy almost always.

This script will function properly for 0 cost disables, all the way up to full
1000-cost disables. It does require the era divisors to be fully upgraded to
355 XP cost, though. It will exit once all the elements are disabled, although
in many cases that will never occur.

An external blueprint script can also change the elements, in order to use the
disabler as an (optional) part of its BP. To do this, set the global variable
`<size=0>element_config` sometime early in your script. Also make sure to add
`</size>` to the end of the value. (So instead of "nature<darkness", you would
set it to "nature<darkness&lt;/size>".

## Infinity Phase Scripts
SPOLERS BELOW FOR INFINITY PHASE! YOU HAVE BEEN WARNED!
<br><br><br><br><br><br><br><br><br><br><br><br><br><br><br><br>

### Trading Post Crate Spam
I believe this to use the most efficient crate-gaining strategy currently possible, much better than using the e60 perk. It gets me 5e15 crates/sec from an idle income of 6e56/sec, and scales 10x per e5 of additional resources.
It uses turbo exec v2.2 to rapidly move between crate trading and idle mode, such that every rendered frame is in idle mode. **Turbo exec v2.2 is required:** (https://github.com/d0sboots/TPT2_scripts/blob/main/common/turbo_exec/README.md)

This requires the Air Stone to be active, and remember to put all your Town Perks into crate gain. Go to Tower Testing and enable 100% for cubes in Idle Mode, then press "o".

**This script is a bit finicky. It is working correctly if you are "staying" in idle mode, but the gains keep jumping around instead of increasing.**

`D0S.Crates:Idler_v2 1 1 22`
`D0S.Crates:Benchmark 1 1 12`
```
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
```

Also included is a benchmarking utility, useful for tuning the loop constant for your machine (for maximum performance). If you don't care, you can safely delete/disable it with no ill effect. Pressing "b" will run the crate trader for 15 seconds, and display the *exact* amount of time it ran, to let you calculate precise crates/sec. Obviously, you have to record crates before/after to make this work.

## Other Utilities
### Power Plant Optimizer/Calculator
https://www.desmos.com/calculator/iryztwyeog

Allows for fine-grained control of power-plant building counts to determine the optimal layout, accounting for Town Perks and what building is powered.

### Exotic Lab Simulator
https://d0sboots.github.io/PerfectTower/exotic_sim.html

A bit quick-and-dirty, this simulates the mechanics of the Exotic Lab. Mostly it just validates that the best strategy is to put one point into everything and then pump Time Factor and Boost. (But you can see how much time you're gaining for your gems...)
