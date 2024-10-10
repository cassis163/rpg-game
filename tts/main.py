from styletts2 import tts
import nltk


def main():
    # https://pypi.org/project/styletts2/

    nltk.download('punkt_tab')

    # No paths provided means default checkpoints/configs will be downloaded/cached.
    my_tts = tts.StyleTTS2()

    # Optionally create/write an output WAV file.
    out = my_tts.inference("Yo wassup", output_wav_file="test.wav")


if __name__ == "__main__":
    main()
