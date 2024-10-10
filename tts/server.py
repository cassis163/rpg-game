from tts import generate_audio
from http.server import BaseHTTPRequestHandler, HTTPServer
import time

hostName = "localhost"
serverPort = 4003


class Server(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path == "/":
            self.write_home_response()
        elif self.path.startswith("/generate"):
            text = self.path.split("/generate?text=")[1]
            self.write_audio_response(text)
    
    def write_audio_response(self, text):
        self.send_response(200)
        self.send_header("Content-Disposition", f'attachment; filename="generated.wav"')
        self.send_header("Content-Type", "application/octet-stream")
        self.end_headers()
        self.wfile.write(generate_audio(text))

    def write_home_response(self):
        self.send_response(200)
        self.send_header("Content-type", "text/html")
        self.end_headers()
        self.wfile.write(
            bytes("<html><head><p>TTS-server is running.</p></head>", "utf-8")
        )

def run():
    webServer = HTTPServer((hostName, serverPort), Server)
    print("Server started http://%s:%s" % (hostName, serverPort))

    try:
        webServer.serve_forever()
    except KeyboardInterrupt:
        pass

    webServer.server_close()
    print("Server stopped.")
