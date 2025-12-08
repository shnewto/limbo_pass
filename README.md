# limbo pass

a little game, limbo on a mountain pass

![limbo pass screenshot](img/bevy-scene.png)

## run the game

build / run with the **release** flag so the overworld theme loads on time :)

```sh
cargo run --release
```

## run on the web (WASM)

First, install the WASM target and wasm-server-runner:

```sh
rustup target install wasm32-unknown-unknown
cargo install wasm-server-runner
```

Run from the project root:

```sh
cargo run --target wasm32-unknown-unknown --release
```

This will automatically start a local web server and open the game in your browser.

**Note**: The game is not available on mobile or touch screens. A message will be displayed instead.

## deploy to cloudflare pages

The build script will automatically install required tools if they're missing. To build for production:

```sh
./build.sh
```

This creates a `dist/` directory with all the files needed for deployment, including:
- Optimized WASM binary (under 25MB for Cloudflare Pages)
- JavaScript bindings
- Assets (audio, fonts, GLTF scenes)
- HTML and supporting files

```sh
# if not installed -> npm install -g wrangler
wrangler pages deploy dist
```

### Local Testing

To test the production build locally:

```sh
wrangler pages dev dist
```

### Debug Builds

For faster iteration during development, you can build in debug mode:

```sh
BUILD_MODE=debug ./build.sh
```

## gameplay

The game features a loading screen while assets load, followed by a menu with a "head to limbo pass" button to start playing.

### wander

- space bar
- ← ↑ ↓ →
- w a s d

### look

- hold ctrl and move the mouse to orbit the camera around the scene
- scroll to zoom
- secondary click / right click for a slow camera pan

### audio

- Click the "music" button in the top-right corner to toggle background music on/off

## about

- limbo pass was written with rust, bevy 0.17, and the amazing bevy plugins bevy_kira_audio, bevy_rapier3d, and smooth-bevy-cameras. the scene was modled in blender 3.2.0 alpha
- the overworld theme was made on an op-1, using the the deep space string synthesizer and tombola sequencer - then post processed with ffmpeg. if you just want to hear the theme, it's also listenable on [soundcloud](https://soundcloud.com/wanderball)
- the game is optimized for desktop browsers and is not available on mobile or touch devices

## why

mostly this little game was me figuring out how to use blender assets in bevy and then how to make the ghost's movement respect the surfaces of the terrain. i drew a lot of inspiration and owe a lot of lightbulb moments to the @sdfgeoff projects [blender_bevy_toolkit](https://github.com/sdfgeoff/blender_bevy_toolkit) and [blender_bevy_top_down_space_shooter](blender_bevy_top_down_space_shooter).

## blender

some details on implementation on the blender side. in case you're curious or looking for examples of how to do this sort of stuff yourself like i was at every step of the way.

- the scenes used for this project live in `blend/limbo_pass.blend` and are exported to `assets/gltf/limbo_pass.gltf`
- this project gets both scenes and the terrain mesh by name so make sure you keep track of those :)
- the ghost's origin is somewhere near the object's center of mass but I toggled it a bit. on the bevy / rapier3d side, the scene shares a transform with a sphere collider and because the ghost's shape is irregular it needed a bit of adjusting to match the collider's vertical area
- i ended up doing some... cursed looking things i probably didn't have to to get the terrain's vertices and indexes for the trimesh collider, if you can point me at a better solution I'd be glad for it :)
- i left this commented out in the main function, it was very helpful when developing colliders to see them rendered `.add_plugins(RapierDebugRenderPlugin::default())`

![ghost form scene](img/ghost-form-scene.png)

![terrain scene](img/terrain-scene.png)

![gltf export](img/gltf-export.png)
