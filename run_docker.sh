docker build . -f docker/Dockerfile -t bitmapflow
docker run -it bitmapflow

# Copy the .so file out of the container
docker run --rm --entrypoint cat bitmapflow /usr/src/bitmapflow/target/release/libbitmapflow_rust.so > libbitmapflow_rust.so