from styletts2 import tts
from tts import init as init_tts
from server import run as run_server
import nltk

host = "localhost"
port = 5000

def main():
    init_tts()
    run_server()

if __name__ == "__main__":
    main()
