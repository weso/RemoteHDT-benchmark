import os
import zarr
from http.server import SimpleHTTPRequestHandler
from socketserver import TCPServer
import sys

class ZarrHandler(SimpleHTTPRequestHandler):
    def do_GET(self):
        if self.path.endswith('.zarr'):
            # Serve Zarr file with proper headers
            self.send_response(200)
            self.send_header('Content-type', 'application/zarr')
            self.end_headers()

            # Construct the correct file system path
            zarr_file_path = os.path.join(zarr_directory, self.path[1:])
            
            # Load the Zarr file and serve
            root = zarr.open_group(zarr_file_path, mode='r')
            root.write_to(self.wfile)
        else:
            # Serve other files with default handler
            super().do_GET()

# Specify the directory containing the Zarr file
#zarr_directory = '../zarr-files/n50-p10-t100.zarr'
zarr_directory ='../zarr-files/' + sys.argv[1]
# Specify the server address and port
server_address = ('', 8000)

# Set up the server
with TCPServer(server_address, ZarrHandler) as httpd:
    print(f"Serving Zarr file {sys.argv[1]} from {zarr_directory} at http://localhost:8000/{sys.argv[1]}")
    httpd.zarr_directory = zarr_directory
    httpd.serve_forever()
