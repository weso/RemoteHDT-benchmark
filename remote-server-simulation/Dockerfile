# Use the official Nginx image as the base image
FROM nginx:latest


# Copy the contents of the 'zarr-files' directory to the Nginx default directory
COPY zarr-files /usr/share/nginx/html/

# Configure Nginx to enable directory listing
RUN sed -i 's/index  index.html/index  index.html index.htm autoindex on/' /etc/nginx/nginx.conf

# Expose port 80
EXPOSE 80
