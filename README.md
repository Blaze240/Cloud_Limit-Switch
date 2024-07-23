# Limit Effects Cloud
Skyline plugin that modifies Cloud's effects while in Limit Break form.

## General Changes
* Sword trails and swordflare effect are changed to the blue variant during Limit Break for the following moves:
  * Neutral Aerial
  * Up Aerial
  * Back Aerial
  * Forward Tilt
  * Up Tilt
  * Up Smash
* Blue variant of effects for the following moves (this is due to these moves utilizing a unique effect from other moves and so a duplicate was made courtesy of effect converter):
  * Forward Aerial
  * Dash Attack
  * Forward Smash

# Implementation
This plugin predominantly rewrites each move's ACMD effect script to include a check if Cloud's Limit Break is active; if it is then the preceding effects will be the swordflare and trail effects used for Limit (which are preexisting due to Limit Cross-Slash utilizing these effects).

For Forward Aerial, Dash Attack, and Forward Smash, a new effect is pointed to instead since the effects for these moves are unique emitters to these effects rather than the standard sword trail effect used for other moves. Most of the mod's work actually comes from the .eff file modified with EffectConverter, in which the original emitter effects were duplicated and given their own name and entry in the NamcoFile files. The inner textures witih these effect folders were given a unique ID number as well in order to distinguish itself from the preexisting texture used normally in battle.

