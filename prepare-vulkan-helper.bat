git submodule update --init
cd vulkan_helper\resources\shaders
glslangValidator -V fragmentShader.frag -o fragmentShader.spv
glslangValidator -V vertexShader.vert -o vertexShader.spv
cd ..\..
mkdir build
cd build
cmake .. -G"Visual Studio 17 2022"
cmake --build . --config Debug

xcopy include ..\..\include /i /s /y
xcopy lib ..\..\lib /i /s /y
cd ..\..

mkdir target\debug
xcopy resources target\debug\resources /i /s /y
mkdir target\release
xcopy resources target\release\resources /i /s /y

