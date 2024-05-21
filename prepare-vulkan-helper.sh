git submodule update --init
cd vulkan_helper/resources/shaders
glslangValidator -V fragmentShader.frag -o fragmentShader.spv
glslangValidator -V vertexShader.vert -o vertexShader.spv
cd ../..
mkdir build
cd build
cmake .. 
cmake --build .

cp -rf include ../../
cp -rf lib ../../
cd ../..

cd resources/shaders
glslangValidator -V fragmentShader.frag -o fragmentShader.spv
glslangValidator -V vertexShader.vert -o vertexShader.spv
cd ../..

mkdir target
mkdir target/debug
cp -rf resources target/debug/resources
mkdir target/release
cp -rf resources target/release/resources
cd ..
