from tts import generate_audio
from http.server import BaseHTTPRequestHandler, HTTPServer
import soundfile as sf
import io
import time

hostName = "localhost"
serverPort = 4003


class Server(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path == "/":
            self.write_home_response()
        elif self.path.startswith("/generate"):
            text = self.path.split("/generate?text=")[1]
            self.write_audio_response(text.replace("%20", " "))

    def write_audio_response(self, text):
        g_audio = generate_audio(text)  # NumPy array

        sample_rate = 24000  # Define sample rate or retrieve it from the audio generation

        # Convert the NumPy array to .wav format using soundfile
        wav_buffer = io.BytesIO()
        sf.write(wav_buffer, g_audio, sample_rate, format='WAV')

        # Retrieve the byte data from the buffer
        wav_bytes = wav_buffer.getvalue()

        self.send_response(200)
        self.send_header("Pragma", "public")
        self.send_header("Content-Disposition", 'attachment; filename="generated.wav"')
        self.send_header("Content-Type", "audio/x-wav")
        self.send_header("Content-Transfer-Encoding", "binary")
        self.send_header("Expires", "0")
        self.send_header("Cache-Control", "must-revalidate, post-check=0, pre-check=0")


        self.send_header("Cache-Control", "public")
        self.send_header("Content-Description", "wav file")
        self.send_header("Content-Length", str(len(wav_bytes)))  # Length of the byte data
        self.end_headers()

        self.wfile.write(wav_bytes)


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
