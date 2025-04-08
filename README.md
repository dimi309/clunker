Clunker
=======

This project is basically a program, written in Rust, which loads a model of a
goat from a .glb (gltf) file and renders it using the Vulkan API, also playing
a short sound read from an .ogg file. The goat is very basic and so are the 
shaders. This is by no means a complete game engine or library but rather just
an experiment that can also be built upon to produce a game or other graphical 
application that runs on Windows and Linux.

**I am not using ash or any other Vulkan-wrapping crate.** I am just accessing
my own [vulkan_helper](https://github.com/dimi309/vulkan_helper) C library with 
Rust (vulkan_helper was also used by my [small3d](https://github.com/dimi309/small3d) game development
library [once upon a time](https://github.com/dimi309/small3d/releases/tag/1.8015.last.vulkan)). 
The vulkan_helper and the Vulkan API bindings are created during the build with 
bindgen. It is because of the existence of this C - Rust interface that
I have called the project "Clunker".

![clunker](clunker-logo.png)


Prerequisites
-------------

- Rust (On both Windows and Linux I install it as suggested here:
  https://www.rust-lang.org/learn/get-started)
- A C compiler (Visual Studio on Windows, gcc / clang on others)
- CMake
- The [Vulkan SDK](https://vulkan.lunarg.com/) and the`VULKAN_SDK` environment 
  variable set to the path of the SDK
- Specifically, clang (even if you are using another compiler this one has to be 
  installed in parallel).

On Ubuntu / Debian for example clang can be installed like this:

	sudo apt-get install libclang-dev

Some more libraries might need to be installed for Linux, for example
for Ubuntu:

    sudo apt-get install livbulkan-dev
    sudo apt-get install libxcb-randr0-dev

On Windows you can download a prebuilt binary from here for example:

https://github.com/llvm/llvm-project/releases/tag/llvmorg-17.0.1

Try 

LLVM-17.0.1-win64.exe

... and set LIBCLANG_PATH environment variable to the installed binary
directory, for example `D:\llvm17\LLVM\bin`.
   
Setup
-----

You need to execute `prepare-vulkan-helper.bat`on Windows or 
`prepare-vulkan-helper.sh` on Linux to build and set up the vulkan_helper 
library before launching cargo. 

This repository contains the vulkan_helper repository as a submodule. Use 
the `--recursive` flag when cloning, otherwise the vulkan_helper subdirectory 
will remain empty on your drive. Alternatively the submodule can be retrieved 
after cloning using the following commands:
	
	git submodule init
	git submodule update

On Linux, the WINIT_UNIX_BACKEND environment variable might have to be set
to "x11". Otherwise winit may launch using wayland, making winit's xlib_window()
and xcb_connection() window functions return None. The values returned by these 
functions are needed for creating a Vulkan surface.

![snapshot](clunker.png)
