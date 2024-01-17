# Use a base image (you can choose a suitable base image for your needs)
FROM alpine:latest

# Set the working directory inside the container
WORKDIR /app

# Create a directory to store the files
RUN mkdir /app/zarr-files

# Copy all files from the host machine's zarr-files folder into the container
COPY ./zarr-files /app/zarr-files

# Display the content of the copied folder (optional, for verification)
RUN ls -l /app/zarr-files

# Define the default command (you can customize this based on your needs)
CMD ["echo", "Container is ready!"]
