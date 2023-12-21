---
marp: true
theme: custom-theme
math: mathjax
paginate: false
footer: ""
backgroundColor: "#282828"
---

<!-- 
_class: "lead"
_footer: " "
-->

## My Game 2024

### December 2023 notes

---

# What kind of game do I want?
### Genre

Fantasy RPG, survival?

### Rendering
- First person pseudo 3D
- Huge 2D map (open world? procedural? some mix of everything)
- Pixel art (2D sprites/textures for everything)
- Can switch between 2D and 3D at any time
- Engine-agnostic (optimized for all GPU's, possibility of CPU software rendering)
- Customizable/moddable?

---

# January goals

### Create the engine

- Map/level editor. Load maps from image files? Pixel color = texture.
- Render map (raycasting, but with full quad GPU rendering, not stripes)
- Walls rendered as quads, floor/ceiling as quads too (more customizeable)
- Switch between fully 2D top-down / fully 3D view
- Sky texture. Day/night cycle. 2D lighting?
- Render animated sprites

Use **SFML**, Raylib?

---

# Raycasting walls

- Launch 2D rays from camera
- First launch to the centers of closest tiles, then further and further
- If no collisions, draw wall/ceiling quads
- If collision, draw wall quad
- Quads are drawn by projecting their corners to the camera

