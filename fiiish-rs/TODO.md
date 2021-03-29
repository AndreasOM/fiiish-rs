# TODO


## In Progress

- [ ] Render quad with texture

## TODO



- [ ] Split renderer context from window
- [ ] Extend Vector2 with more functionality

- [ ] Add basic ResourceManager

## Future

- [ ] Only transfer minimal per Vertex data to GPU (e.g. remove TexCoords where not needed)

- [ ] Remember window size, and position
- [ ] Retain info of open FilesystemStreams in Filesystems to allow cleanup, e.g. via Rc or Arc

## Out of scope (for now)

- [ ] Add multi texture support

## DONE

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
