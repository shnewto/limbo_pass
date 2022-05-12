# limbo pass

a little game, limbo on a mountain pass

![limbo pass screenshot](img/bevy-scene.png)

## run the game with trunk (wasm)

make sure you have trunk https://trunkrs.dev/

then
```
trunk serve --release
```

and open http://127.0.0.1:8080 in your browser

## wander

- space bar
- ← ↑ ↓ →
- w a s d

## look

- hold ctrl and move the mouse to orbit the camera around the scene
- scroll to zoom
- secondary click / right click for a slow camera pan

## about

- limbo pass was written with rust, bevy, and the amazing bevy plugins bevy_kira_audio, bevy_rapier3d, and smooth-bevy-cameras. the scene was modled in blender 3.2.0 alpha
- the overworld theme was made on an op-1, using the the deep space string synthesizer and tombola sequencer - then post processed with ffmpeg. if you just want to hear the theme, it's also listenable on [soundcloud](https://soundcloud.com/wanderball)

## why

mostly this little game was me figuring out how to use blender assets in bevy and then how to make the ghost's movement respect the surfaces of the terrain. i drew a lot of inspiration and owe a lot of lightbulb moments to the @sdfgeoff projects [blender_bevy_toolkit](https://github.com/sdfgeoff/blender_bevy_toolkit) and [blender_bevy_top_down_space_shooter](blender_bevy_top_down_space_shooter).

## blender

some details on implementation on the blender side. in case you're curious or looking for examples of how to do this sort of stuff yourself like i was at every step of the way.

- the scenes used for this project live in `blend/limbo_pass.blend` and are exported to `limbo_pass/assets/gltf/limbo_pass.gltf`
- this project gets both scenes and the terrain mesh by name so make sure you keep track of those :)
- the ghost's origin is somewhere near the object's center of mass but I toggled it a bit. on the bevy / rapier3d side, the scene shares a transform with a sphere collider and because the ghost's shape is irregular it needed a bit of adjusting to match the collider's vertical area
- i ended up doing some... cursed looking things i probably didn't have to to get the terrain's vertices and indexes for the trimesh collider, if you can point me at a better solution I'd be glad for it :)
- i left this commented out in the main function, it was very helpful when developing colliders to see them rendered `.add_plugin(RapierDebugRenderPlugin::default())`

![ghost form scene](img/ghost-form-scene.png)

![terrain scene](img/terrain-scene.png)

![gltf export](img/gltf-export.png)
