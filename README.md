<img src="metra-icon.svg" width="128px" alt="Metra logo"/>

# The Metra Game Engine

Metra is a framework/game engine for making 2D games for the web using Rust.
Note that Metra is currently under active development and is not yet fit for use in production applications..

# Features

## Declarative rendering

Rendering in Metra is *declarative.* This makes it closer to a traditional game engine than many
other frameworks (such as Raylib or low-level APIs like Vulkan or SDL3GPU). In Metra,
after creating a Model (low-level mesh) and a Material (shader program), you may construct
a Mesh, which renders on-screen for as long as it is alive. Dropping the Mesh stops rendering it.

## Post-processing effects

A post-processing pass in Metra looks something like this:
1. render the entire game to a texture
2. render a screen mesh using that texture and the effect material to another texture
3. render that texture to the canvas

## Real-time 2D lighting

## Asset loading

Most assets will be preloaded for you.
On-demand loading of assets is a future feature.

## Audio processing

Current plan is for two modes:
- stateful (music, ambience)
- instant (play-and-forget effects)

Allow for pausing

## Persistent data

Basic local storage.

## High-Level game input

Action Spec.

- Action Type:
	- Plane
	- Axis
	- Burst
	- Hold
- Gamepad Actions
	- p (plus)
	- m (minus)
	- tr/tl (triggers, digital)
	- br/bl (bumpers, digital)
	- ls { x, y, b } (left stick)
	- rs { x, y, b } (right stick)
	- f { n, e, s, w } (face buttons)
	- d { n, e, s, w } (d-pad)

```jsonc
{
	// outer object stores the action names
	"movement":{
		"type": "plane",
		"normalize": true,
		"keyboard": {
			"w": "+y",
			"a": "-x",
			"s": "-y",
			"d": "+x"
		},
		"gamepad": {
			"ls.x": "x",
			"ls.y": "y"
		}
	},

	"jump": {
		"type": "burst",
		"keyboard": { "space": true },
		"gamepad": { "f.e": true }
	}
}
```

# Future + Ideas

- Persistent data; base64, JSON, other? maybe compress?
- Worker; do we put WASM on a different thread?
- Static preparation
	- HTML file (game name/icon)
	- Rust/JS imports (action bindings, asset loading)
- Shaders; do we only allow fragment shaders (tragic) or do we implement two different lighting algorithms at once?
- Loading
	- a "loaded" flag
	- a load event (event queue; load multiple assets at once in order to streamline?)