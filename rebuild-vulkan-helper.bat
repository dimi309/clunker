git submodule update --init
cd vulkan_helper\build
cmake --build . --config Debug

xcopy include ..\..\include /i /s /y
xcopy lib ..\..\lib /i /s /y
cd ..\..
