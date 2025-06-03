# The Metro Game Engine

Metro is a framework/game engine for making 2D games for the web using Rust.

# Features

## Declarative rendering

Rendering in Metro is *declarative.* This makes it closer to a traditional game engine than many
other frameworks (such as Raylib or low-level APIs like Vulkan or SDL3GPU). In Metro,
after creating a Model (low-level mesh) and a Material (shader program), you may construct
a Mesh, which renders on-screen for as long as it is alive. Dropping the Mesh stops rendering it.

## Post-processing effects

A post-processing pass in Metro looks something like this:
1. render the entire game to a texture
2. render a screen mesh using that texture and the effect material to another texture
3. render that texture to the canvas

## Real-time 2D lighting

If a mesh

## High-Level input management

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