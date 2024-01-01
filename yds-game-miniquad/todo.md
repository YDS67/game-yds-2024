# TODO for this version

- Optimize text rendering. (Make a marker variable for highlighted text so the shader doesn't need to check the fragment position each time. Input the marker to Vertex shader).
- Make a GUI struct and module, add methods for button and slider.
- Introduce different textures to walls, floors and ceiling (changes to mesh module, use map array values as input).
- Put a single quad mesh generation in a separate function to optimize mesh module.