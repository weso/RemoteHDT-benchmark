# Use the official Nginx image as the base image
FROM nginx:latest

# Set the working directory inside the container
WORKDIR /usr/share/nginx/zarr

# Copy all .zarr files from the local 'zarr-files' folder to the container's working directory
COPY ./zarr-files/ .

# Copy a custom Nginx configuration file to serve .zarr files
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Expose port 80 for HTTP traffic
EXPOSE 80
