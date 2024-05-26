Clunker
=======

Boilerplate for creating cross-platform games and graphical applications running 
on Vulkan, using the Rust programming language and only a small set of 
dependencies. The project provides bindings for the Vulkan API, also taking care 
of some of the Vulkan setup busywork and ensuring cross-platform compatibility, 
as it **runs on Windows, Linux and MacOS**.

**I am not using ash or any other Vulkan-wrapping crate.** I am just accessing
my own [vulkan_helper](https://github.com/dimi309/vulkan_helper) C library with 
Rust (vulkan_helper was also used by my [small3d](https://github.com/dimi309/small3d) game development
library [once upon a time](https://github.com/dimi309/small3d/releases/tag/1.8015.last.vulkan)). 
The vulkan_helper and the Vulkan API bindings are created during the build with 
bindgen. It is because of the existence of this C - Rust interface that
I have called the project "Clunker", like an old customised car :) 

The project is basically a program, written in Rust, which loads a model of a
goat from a .glb (gltf) file and renders it using the Vulkan API. The goat
is very basic and so are the shaders. I just clone the codebase and
use it as a starting point for other projects.

**No additional features will be added, only bug fixes and adaptations,
ever-increasing the project's robustness and safety level.**

![clunker](clunker-logo.png)

Prerequisites
-------------

- Rust
- A C compiler (Visual Studio on Windows, gcc / clang on others)
- CMake
- The [Vulkan SDK](https://vulkan.lunarg.com/) and the`VULKAN_SDK` environment 
  variable set to the path of the SDK
- Specifically, clang (even if you are using another compiler this one has to be 
  installed in parallel).

On Ubuntu / Debian for example clang can be installed like this:

	sudo apt-get install libclang-dev

On Windows you can download a prebuilt binary from here for example:

https://github.com/llvm/llvm-project/releases/tag/llvmorg-17.0.1

Try 

LLVM-17.0.1-win64.exe

... and set LIBCLANG_PATH environment variable to the installed binary
directory, for example `D:\llvm17\LLVM\bin`.
   
Setup
-----

You need to execute `prepare-vulkan-helper.bat`on Windows or 
`prepare-vulkan-helper.sh` on Linux and MacOS to build and set up the 
vulkan_helper library before launching cargo. 

This repository contains the vulkan_helper repository as a submodule. Use 
the `--recursive` flag when cloning, otherwise the vulkan_helper subdirectory 
will remain empty on your drive. Alternatively the submodule can be retrieved 
after cloning using the following commands:
	
	git submodule init
	git submodule update

On Linux, the WINIT_UNIX_BACKEND environment variable has to be set to "x11". 
Otherwise winit may launch using wayland, making winit's xlib_window() and 
xcb_connection() window functions return None. The values returned by these 
functions are needed for creating a Vulkan surface.

![snapshot](clunker.png)
