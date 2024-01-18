# Use an official Python runtime as a parent image
FROM python:3.8-slim

# Set the working directory to /app
WORKDIR /app

# Copy all files inside the zarr-files directory into the container at /app/zarr-files/
COPY ./zarr-files/* /app/zarr-files/

# Make port 80 available to the world outside this container
EXPOSE 80

RUN ls -l /app/zarr-files/

# Run a simple HTTP server when the container launches
CMD ["python", "-m", "http.server", "80"]
