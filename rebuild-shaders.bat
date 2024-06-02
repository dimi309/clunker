cd resources\shaders
glslangValidator -V fragmentShader.frag -o fragmentShader.spv
glslangValidator -V vertexShader.vert -o vertexShader.spv
cd ..\..

xcopy resources target\debug\resources /i /s /y
xcopy resources target\release\resources /i /s /y
