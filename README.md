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

# FAQ
## Will there be a Smashline 1 version?
I leave this code open source for the sole purpose of allowing anyone willing to backport it to Smashline 1, but I personally will not making mods for Smashline 1 and will purely focus on Smashline 2 for plugin development.

## Is it Wi-Fi safe?
As only effects were changed and hitboxes remained unmodified, this is Wi-Fi safe.

## Can this mod be made single slot?
While I do not personally have an interest in making it one slottable, it can be done via changing the function names in the install section to something unique and modifying the motion_list entries to use this unique function.

## Does this mod work with One Slot Effects?
I have not personally tested the compatability with OSE; however this may adversely affect those effects and any custom trails will be overwritten while Limit Break is present. The eff file present can be used as a template for OSE edits, but it is user discretion to go this route.