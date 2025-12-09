sudo ln -sfn /usr/local/cuda-13.0 /usr/local/cuda

# write the env avoid session loss
echo 'export CUDA_HOME=/usr/local/cuda' >> ~/.bashrc
echo 'export PATH=$CUDA_HOME/bin:$PATH' >> ~/.bashrc
# here is the wsl libcuda.so
echo 'export LD_LIBRARY_PATH=/usr/lib/wsl/lib:$CUDA_HOME/lib64:$LD_LIBRARY_PATH' >> ~/.bashrc
source ~/.bashrc

# verify
which nvcc
nvcc --version
cat /usr/local/cuda/version.json 2>/dev/null || cat /usr/local/cuda/version.txt 2>/dev/null