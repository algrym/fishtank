# https://github.com/algrym/fishtank TODOs

## Features to Add:

_(in no particular order)_
* Switch to 3D: Add depth?
* Species of Fish:
  * Same colors means same behavior
  * Schooling like [Boids](https://en.wikipedia.org/wiki/Boids)
* Make fish movement less jittery
  * Migrate to Rapier2D
    * Setup environment
    * Add bubble wobble
    * Setup fish as dynamic objects
  * Address acceleration and drag
  * https://lib.rs/crates/bevy_easings
* Fish should move toward a destination
* Fish should look toward their destination
* Fish should notice collisions
* Maybe bubbles should be particle systems?
* Improve the CI pipeline
  * https://github.com/bevyengine/bevy_github_ci_template
