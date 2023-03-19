# SCOP


https://user-images.githubusercontent.com/55497343/226149531-847625d6-16f5-4cfc-96b3-d8403a4cd962.mp4

![img](https://github.com/kotabrog/SCOP/blob/main/img/42_with_taiyaki.png)
![img](https://github.com/kotabrog/SCOP/blob/main/img/teapot.png)

## Overview

Program to display a 3d model using opengl and rust, created as a 42 assignment.

## Requirement

- cargo 1.66.0
- OpenGL 3.2

## Usage

```
git clone .....
cd SCOP
make
./scop_display
```

To read and display .obj format files

```
./scop_display [file]
```

## Features

- Loading Files
    - Loading .obj files
        - Readable: v, f(only vertex index)
    - Loading textures in .bmp files (PASS must be written in the code)
        - Gray, BGR, BGRA
- 3d model display
    - Default color is some color in grayscale
    - Texture can be pasted by pressing F1 button
    - (With a little more refinement, a texture can be assigned to each button)
- Moving the 3d model
    - left-clicking with the mouse: vertical and horizontal movement
    - clicking the wheel with the mouse: rotation on any rotation axis
    - right-clicking with the mouse: z-axis rotation
    - rotating mouse wheel: z-axis movement

## Author

[twitter](https://twitter.com/Kotabrog)

## Licence

[MIT](https://github.com/kotabrog/SCOP/blob/main/LICENSE)
