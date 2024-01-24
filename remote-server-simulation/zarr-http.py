import zarr
from http.server import SimpleHTTPRequestHandler
from socketserver import TCPServer

class ZarrHandler(SimpleHTTPRequestHandler):
    def do_GET(self):
        if self.path.endswith('.zarr'):
            # Serve Zarr file with proper headers
            self.send_response(200)
            self.send_header('Content-type', 'application/zarr')
            self.end_headers()

            # Load the Zarr file and serve
            zarr_file_path = self.path[1:]
            root = zarr.open_group(zarr_file_path, mode='r')
            root.write_to(self.wfile)
        else:
            # Serve other files with default handler
            super().do_GET()

# Specify the directory containing the Zarr file
zarr_directory = './n50-p10-t10.zarr'

# Specify the server address and port
server_address = ('', 8000)

# Set up the server
with TCPServer(server_address, ZarrHandler) as httpd:
    print(f"Serving Zarr files from {zarr_directory} at http://{server_address[0]}:{server_address[1]}/")
    httpd.zarr_directory = zarr_directory
    httpd.serve_forever()
