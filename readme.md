# Minimal POC for a Roguelike UI Manager

This repo is a proof of concept for a roguelike UI and a generally expandable template for a rogue/df-like.

Uses winit + pixels for window + rendering. Intended to replace [bracket-terminal](https://github.com/amethyst/bracket-lib/tree/master/bracket-terminal) in my own project, so has some features from that (Also some code)

A few goals:
- General features you'd expect in a terminal emulator library such as Bracket-lib
- Ability to zoom a game map from tile-based display to pixel-based display
- Project skeleton usable for future development - Separate input handling, assets, and game engine
- Enough abstraction for menus to keep new code clean