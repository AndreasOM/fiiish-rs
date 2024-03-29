# TODO


## In Progress


- [ ] Fix formatting! ;)

## TODO

- [ ] Create released automatically via github actions
	- [ ] Ensure build_number.txt is not too old 

- [ ] Use build.rs to write out version to version.txt
- [ ] Only package needed versions of music for platform
- [ ] Add UI sounds
- [ ] Allow disabling sounds
- [ ] Load sounds via soundbank configuration

- [ ] Make build script more robust
- [ ] Add check to ensure version matches tag
- [ ] Create release builds via github actions (cheat mode)
- [ ] Automate upload to itch.io (macOS, cheat mode, off stream)
- [ ] Automate creating a linux release
- [ ] Automate creating a windows release

- [ ] Play music on Windows
- [ ] Play music on Linux

- [ ] Remove warnings
- [ ] Cleanup: Remove unnecessary `use ...`

- [ ] replace `#[derive(Debug)]` with `#[derivative(Debug)]`

- [ ] External: Update OMT to newer regex crate

- [ ] Remove unnecessary crates. Candidates:
	- [ ] regex


- [ ] Change UiElement handling from Box <...> to Rc< RefCell< ... > >
- [ ] Improve next zone selection
- [ ] Add skip to next zone button
- [ ] Fix Windows build (cheat mode, keyword: shader-crusher)
- [ ] Expand EntityConfiguration to allow multiple AnimatedTextures

- [ ] Create macros for DebugRender
- [ ] Cleanup Pickups (remove Coins)
- [ ] Split renderer context from window
- [ ] Extend Vector2 with more functionality
- [ ] Add basic ResourceManager
- [ ] Refactor WindowUpdateContext
- [ ] Avoid duplication in texture manager
- [ ] Write better formatter for Matrix?? and Vector?
- [ ] Fix texture edges in atlas bleeding over
- [ ] Allow to specify font for UiLabel

- [ ] Use deadness for fish death animation?!
- [ ] Improve coin dropping on death

- [ ] Investigate why Filesystem_Memory exists is not implemented

## Future


- [ ] Experiment with using messages to communicate with UiElements
- [ ] Implement block wise reading for FilesystemStreams
- [ ] Implement, and use Serializer
- [ ] Sort Materials on creation instead of every frame
- [ ] Only transfer minimal per Vertex data to GPU (e.g. remove TexCoords where not needed)
- [ ] Remember window size, and position
- [ ] Retain info of open FilesystemStreams in Filesystems to allow cleanup, e.g. via Rc or Arc
- [ ] Add Read, BufRead, and Seek trait to FilesystemStream
- [ ] Add Angle helpers
- [ ] Fix linkage between TextureAtlas sub Texture via hwid
- [ ] Fix font converter to include correct line height, and baseline

## Out of scope (for now)

- [ ] Select active Material via configuration
- [ ] Multitextureing with tex coords for channel 1, or following is currently not supported
- [ ] Reimplement all Matrix & Vector math
- [ ] Remove glutin dependency

- [ ] Create iOS version

## DONE

- [x] Use oml-audio for audio
- [x] Automate creating a release for macOS
- [x] Add drop mode for sounds
- [x] Add sound effects on macOS
- [x] Stub sound effects for Windows & linux
- [x] Play music on macOS
- [x] Stub music on Windows & Linux
- [x] Store music (& audio) setting
- [x] Update crates to newer versions
- [x] Bump OMT to allow bumping regex crate

## Season 02
### S02E01 - 3.0h
- [x] Switch to using oml-game

## Released

### v0.6.3-alpha

- [x] Add fadeout for entities
- [x] Switch to 2021 edition of Rust

### v0.6.1-alpha

- [x] Add icon to .app bundle for macOS
- [x] Create .app bundle for macOS
- [x] Finalize ResultDialog
- [x] Create release helper scripts

### v0.6.0-alpha

- [x] Add savegame
	- [x] Fix savegame location
- [x] Implement UI system
- [x] Add basic multiline text
- [x] Add version number to settings dialog
- [x] Finalize UiLabel rendering
- [x] Hook up counters
- [x] Add font rendering
	- [x] Fix font layout
	- [x] Fix positioning of glyphs inside text
	- [x] Fix calculation of text layout height
- [x] Add result dialog
- [x] Add UiSpacer
- [x] Add in-game counters
- [x] Stub UiLabel
- [x] Improve UI debug rendering to include bounding rectangles
- [x] Move event result handling into UiElements
- [x] Add UiButton
- [x] Add Settings Button
- [x] Add Settings Dialog
- [x] Add Music, and Sound Button to Settings Dialog
- [x] Add collision with obstacles 
- [x] Add pause button
- [x] Add UI
- [x] Use shapes for collision detection
- [x] Visualise collision shapes
- [x] Add cache for collision shapes
- [x] Add pause to game (use keyboard 'P')
- [x] Load shapes
- [x] Move fish via mouse when paused for debugging
- [x] Add collision between fish & obstacle bounding circles
- [x] Add circles to the debug renderer
- [x] Add color to debug renderer
- [x] Add debug renderner, lines, and frames
- [x] Add Zone progression
- [x] Add Zone management
- [x] Make Player a real entity (and rename player to fiiish)
- [x] Improve loading speed for assets from disk (~15s -> ~0.5s)
- [x] Synchronise Background with game state
- [x] Link pakfiles in if they exist at build time
- [x] Load data from _linked in_ packfile
- [x] Load data from pakfile (.omar)
- [x] Make Coins collectable, and respawn current zones Pickups via 'r' key
- [x] Improve entity configuration
- [x] Fix movement bug
- [x] Throw some rocks into the water, and add some coins, and seaweed to the mix
- [x] Add background
	- [x] Make background fullscreen
	- [x] Fix texture mapping for background
	- [x] Use correct shader for background
	- [x] Move background
	- [x] Add support for multiple texture channels
	- [x] Add second texture for background
- [x] Add multi texture support
- [x] Reflect channel 1, and following textures in Material key
- [x] Add render layers
- [x] Add some coins
- [x] Add TextureAtlas
- [x] Extract demo code out of FiiishApp
- [x] Add AnimatedTexture (a bit hacky)
- [x] Allow quads to be rotated
- [x] Add fish rotation, and use direction for movement
- [x] Limit player/fish angle at boundaries
- [x] Add game logic wrapper
- [x] Add basic Fiiish movement
- [x] Add support for (M)VP uniform
- [x] Add very basic Matrix struct
- [x] Setup correct MVP for orthographic projection
- [x] Add Filesystem_Layered that allows fallbacks for loading
- [x] Add support for runtime modifiable textures
- [x] Add pixel editor, just because we can
- [x] Add initial uniform handling
- [x] Render quad with texture
- [x] Improve shader problem debugging by adding shader source to failure report
- [x] Add texture coordinates to vertices
- [x] Extract render Effect
- [x] Allow material switching for renderer
- [x] Add MaterialBuilder
- [x] Render triangle with customer shader from file
- [x] Add filesystem abstraction
- [x] Encapsulate shader and program
- [x] Move quad renderer into renderer
- [x] Accept mouse input (buttons & position)
- [x] Create container for Vector2
- [x] Render untextured triangle
- [x] Allow setting of window title
- [x] Create wrapper to acces OpenGL functions, and use them in renderer
- [x] Add time step to update context
- [x] Accept keyboard input (exit via ESC & SPACE)
- [x] Open window on MacOS, including OpenGL context
- [x] Create gitignore to reduce noise
- [x] Create initial project

