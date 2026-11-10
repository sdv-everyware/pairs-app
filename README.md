# pairs-app

A memory game for the hallway panel on harry (1920 x 1080): 12 cards face down in a 4 x 3 grid,
six pairs of weather kinds dealt from a seed (`--seed` fixes the deal). A tap turns a card up; a
match stays up, a miss turns back after a second. The kind of the weather now (the dashboard
bundle on port 7100) is the joker: always one of the six pairs, and its pair counts double. The
sidebar shows the moves, the pairs found, the round's time and, at the end, "Done in N moves,
mm:ss" with New round. A card is two Library slots: a `#button` back that takes the tap and a
face over it from the app's own picture set `#pairs-weather` (`design/weather-pictures.js`,
`src/weather.rs`) that the app hides. It is written against the EveryWare UI library
(`vendor/sdv-everyware/ui-lib`) only; the rules are `src/game.rs`, tested on the host
(`atest pairs_app_game_test`).

## The design

`data/pairs.dcf` is made without Figma: `design/pairs-patch.js` (the screen, `#pairs-root` from Library
slots) on the EveryWare Library, then ui-lib's `design/dark-theme-patch.js`, then `fig-to-dcf.sh`.
`design/build.sh` runs the whole chain:

```bash
packages/services/display_safety/utils/pairs-app/design/build.sh     # from anywhere
```

It builds the Library (ui-lib `design/build-library.sh`, `design/build/library.fig` there), this
app's `design/pairs.fig` (in git: the file a designer opens, see below) and
`data/pairs.dcf`; `CHECK='#pairs/time #pairs/...'` names what must be in the .dcf.
fig-to-dcf.sh imports with everyware's `tools/dcf` (dcf-fetch `--local`, on our fork of
DesignCompose). A new board like this one: `vendor/sdv-everyware/ui-lib/skills/new-app/SKILL.md`.

The names under `#pairs-root` (`#pairs/*`) are the contract with `src/ui_model.rs`: rename both or neither.


## The design in Figma, without scripts

`design/pairs.fig` is this app's screen as a Figma file: what is built, checked into git, and
what a designer opens. The plain path, no agent and no tools:

1. **Open it**: Figma, File > Import, `design/pairs.fig`. The screen is the frame `#pairs-root`;
   the Library (the components the screen is made of) is on its Components page.
2. **Change it** as in any Figma file: move, recolour, add a layer, swap an instance's variant.
   The layer names starting with `#` are the contract with the code (`src/ui_model.rs`): keep
   them; a new element the app should drive gets a `#pairs/<name>` of its own, and the app a
   field with that name.
3. **Hand it over**: share the Figma file. The developer fetches it into the app once with
   `scripts/fetch-dcf.sh data/pairs.dcf <file key>` (the key is in the file's URL), and the
   board shows it.

What the scripts add on top is only the other direction: `design/build.sh` writes this `.fig`
from `design/pairs-patch.js` (an agent's or a developer's drawing in code) and the Library, and
makes the `.dcf` without Figma. Both ways meet in the same `.fig` and the same `.dcf`; the
`#names` are what keeps them together.

## Build and run on the RB3

```bash
# from the tree's top: build (sdv_media_db845c, out_rb3)
source device/rb3/sdv_media_db845c/tools/env.sh; m pairs_app

# the panel's dashboard bundle on 127.0.0.1:7100 serves one app at a time: stop the other one first
adb shell mkdir -p /data/local/tmp/pairs
adb push out_rb3/target/product/sdv_media_db845c/system/bin/pairs_app /data/local/tmp/pairs/
adb push packages/services/display_safety/utils/pairs-app/data/pairs.dcf /data/local/tmp/pairs/
adb shell "cd /data/local/tmp/pairs; LD_LIBRARY_PATH=/system/lib64:/vendor/lib64:/apex/com.everyware.smarthome/lib64 \
    ./pairs_app --data-path /data/local/tmp/pairs --touch-size 1920x1080"
```

The fonts are compiled in (`data/*.otf`); the time comes from the board's clock, the weather
from the dashboard bundle.
