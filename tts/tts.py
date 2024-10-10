from styletts2 import tts as style_tts
import nltk

tts = None


def init():
    global tts

    # https://pypi.org/project/styletts2/
    nltk.download("punkt_tab")

    # No paths provided means default checkpoints/configs will be downloaded/cached.
    tts = style_tts.StyleTTS2()


def generate_audio(text):
    global tts
    return tts.inference(text)
