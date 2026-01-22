# fury-renegade-rgb

A small CLI app that allows you to control the RGB lights on Kingston Fury Renegade RAM

Note1: Currently this only works on linux, though contributions to make this work on windows as well will be welcome.

Note2: This was only ever tested with 4 sticks in my system. While you can probably use the timing synchronizing for less than 4 sticks, I currently do not account for that.

Note3: You must have permission to write to the i2cbus to use this. This would involve either running the command as root (sudo) or adding yourself to the `i2c` group.

## Usage

```
fury-renegade-rgb -b /dev/i2c-1 -1 -2 -3 -4 COMMAND
```
Where `/dev/i2c-1` is a path to the i2c bus where your RAM is. I wouldn't know where it is on your device, but that's where it is on mine.

You can omit any of the `-1`, `-2`, `-3`, and `-4` flags if you don't wanna send the command to any of the sticks.

`COMMAND` is any of what's below

## Commands

* `noop` just sends a noop command to the specified sticks
* `reset` does the following
  * Syncronizes the sticks
  * Sets all brightness values to 100%
  * Sets pattern to rainbow
  * Resets timing delays and offsets
* `sync` syncs the sticks together
  * Note that you can only syncronize sticks that are beside each other. E.g., you can sync all of them together, 1 with 2, and 3 with 4, but you can't sync 1 with 3.
* `colour-brightness --red VALUE --green VALUE --blue VALUE`
  * multiplies each RGB component by the specified percentegaes after the animation. For example, if an animation naturally has a "grey" background, it can be changed here.
* `brightness VALUE`
  * Between 0 and 100, dims the overall pattern with 100 being the default
* `pattern-start-offset VALUE`
  * Sets a delay before starting the pattern. On a syncronized set of sticks, the offset appears to be additive
* `pattern-repeat-delay VALUE`
  * A delay before the pattern repeats, this should be set to the same value on all sticks that are syncronized. It appears that if one stick restarts its pattern, all of them do.
* `pattern STYLE [COLOUR ...]`
  * `STYLE` can be any of the following
    * `solid` - Always shows custom colour
    * `rainbow` - The unicorn barf we all know and love, no custimization options have any effect
    * `scan` - A dot scans from top to bottom, then settles on the center
    * `breathe` - Fades in and out
    * `fade` - Only fades in, most interesting with the colour cycle
    * `stripe` - Wipes from the bottom to top
    * `trail` - Trailing light, bottom to top
    * `lightning` - Electrical pattern, not unlike a plasma ball. Looks best with 4 sticks
    * `countdown` - Counts down from 9 to 0 repeatedly. Looks best with 4 sticks
    * `fire` - Fire pattern, no custimization options have any effect, looks best with 4 sticks
    * `sparkles` - Sprikles random colours around the ram, non-customizable, looks best with 4 sticks
    * `fury` - Writes "F" on the sticks, then "U", then "R", then "Y". Looks best with 4 sticks.
  * `COLOUR` can be comma-seperated values, like "255,255,255" or a hex-colour, like "#FFFFFF". Up to 11 colours can be specified, each "loop" of the pattern will use the next custom colour.
