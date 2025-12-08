sudo ln -sfn /usr/local/cuda-13.0 /usr/local/cuda

# 写入环境（避免 session 丢失）
echo 'export CUDA_HOME=/usr/local/cuda' >> ~/.bashrc
echo 'export PATH=$CUDA_HOME/bin:$PATH' >> ~/.bashrc
# 关键：WSL 的 libcuda.so 在这里
echo 'export LD_LIBRARY_PATH=/usr/lib/wsl/lib:$CUDA_HOME/lib64:$LD_LIBRARY_PATH' >> ~/.bashrc
source ~/.bashrc

# 验证
which nvcc
nvcc --version
cat /usr/local/cuda/version.json 2>/dev/null || cat /usr/local/cuda/version.txt 2>/dev/null