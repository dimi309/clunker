cd vulkan_helper\resources\shaders
glslangValidator -V fragmentShader.frag -o fragmentShader.spv
glslangValidator -V vertexShader.vert -o vertexShader.spv
cd ..\..
mkdir build
cd build
cmake .. -G"Visual Studio 17 2022"
cmake --build . --config Debug



