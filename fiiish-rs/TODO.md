# TODO


## In Progress

- [ ] Add TextureAtlas
	- [ ] Fix sub texture tex coords (via texture mtx)

## TODO

- [ ] Split renderer context from window
- [ ] Extend Vector2 with more functionality

- [ ] Add basic ResourceManager

- [ ] Refactor WindowUpdateContext
- [ ] Avoid duplication in texture manager

- [ ] Write better formatter for Matrix?? and Vector?

## Future

- [ ] Limit player/fish angle at boundaries

- [ ] Only transfer minimal per Vertex data to GPU (e.g. remove TexCoords where not needed)

- [ ] Remember window size, and position
- [ ] Retain info of open FilesystemStreams in Filesystems to allow cleanup, e.g. via Rc or Arc

- [ ] Add Read, BufRead, and Seek trait to FilesystemStream
- [ ] Add Angle helpers

- [ ] Fix linkage between TextureAtlas sub Texture via hwid

## Out of scope (for now)

- [ ] Add multi texture support

## DONE

- [x] Extract demo code out of FiiishApp
- [x] Add AnimatedTexture (a bit hacky)
- [x] Allow quads to be rotated
- [x] Add fish rotation, and use direction for movement
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

## Released


### Version 0.0.1 - Build xxxx
