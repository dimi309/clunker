Clunker
=======

This is my first ever experiment with the Rust language. I have no idea where
it is going. For the moment it is basically a program, written in Rust, which 
draws a box using the Vulkan API. 

I am not using ash or any other Vulkan-wrapping crate. I am just accessing
my own [vulkan_helper](https://github.com/dimi309/vulkan_helper) C library with Rust
(vulkan_helper is also used by the [small3d](https://github.com/dimi309/small3d) game development
library).
The vulkan_helper and the Vulkan API bindings are created during the build with bindgen.

The code is clumsy and unsafe (I am new in crab-land) but it works. You need to
execute `prepare-vulkan-helper.bat` to build and set up the vulkan_helper
library before launching cargo. I've just prepared and tested the project on
Windows, at least for the time being.

Prerequisites
-------------

Rust, Visual Studio, Vulkan SDK (with the`VULKAN_SDK` environment 
variable set to the path of the SDK)

On Linux, clang needs to be installed, on Ubuntu / Debian for example
it is done like this:

   sudo apt-get install libclang-dev

![snapshot](clunker.png)


