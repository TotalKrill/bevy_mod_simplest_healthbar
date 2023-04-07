# bevy_mod_simplest_healthbar

Helps spawn and position small counter "health bars" that take a trait for max and current health, and then just shows them as text like this: "current/max" 
at the location of the specified camera component, make sure that the camera component specified only exists once, or this will panic

## Example

``` rust
