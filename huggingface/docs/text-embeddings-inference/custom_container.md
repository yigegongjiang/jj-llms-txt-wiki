# Build a custom container for TEI

You can build our own CPU or CUDA TEI container using Docker. To build a CPU container, run the following command in the
directory containing your custom Dockerfile:

```shell
docker build .
```

To build a CUDA container, it is essential to determine the compute capability (compute cap) of the GPU that will be
used at runtime. This information is crucial for the proper configuration of the CUDA containers. The following are
the examples of runtime compute capabilities for various GPU types:

- Turing (T4, RTX 2000 series, ...) - `runtime_compute_cap=75`
- Ampere 8.0 (A100, ...) - `runtime_compute_cap=80`
- Ampere 8.6 (A10, ...) - `runtime_compute_cap=86`
- Ada Lovelace (RTX 4000 series, ...) - `runtime_compute_cap=89`
- Hopper (H100) - `runtime_compute_cap=90`
- Blackwell 10.0 (B200, GB200, ...) - `runtime_compute_cap=100`
- Blackwell 12.0 (GeForce RTX 50X0, ...) - `runtime_compute_cap=120`

Once you have determined the compute capability is determined, set it as the `runtime_compute_cap` variable and build
the container using `Dockerfile-cuda`:

```shell
# Get submodule dependencies
git submodule update --init

runtime_compute_cap=80

docker build . -f Dockerfile-cuda --build-arg CUDA_COMPUTE_CAP=$runtime_compute_cap
```
