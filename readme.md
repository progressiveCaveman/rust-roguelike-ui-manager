# Minimal POC for a Roguelike UI Manager

This repo is a proof of concept for a roguelike UI and a generally expandable template for a rogue/df-like.

Uses winit + pixels for window + rendering. Intended to replace [bracket-terminal](https://github.com/amethyst/bracket-lib/tree/master/bracket-terminal) in my own project, so has some features (and code) from that

A few goals:
- General features you'd expect in a terminal emulator library such as Bracket-lib
- Ability to zoom a game map from tile-based display to pixel-based display
- Project skeleton usable for future development - Separate input handling, assets, etc
- ~~Enough abstraction for menus to keep new code clean~~ (This isn't done and I'm not sure how to do it without over-abstraction, so it's incomplete for now. I'm open to suggestions/PRs!)

Controls:
- +/- to zoom in and out
- Arrow keys to move around map and menus, enter to select
- Shift+arrows to move around map by 10 steps
- r to refresh worldgen

![Example GIF](https://imgur.com/a/IFWbGER)