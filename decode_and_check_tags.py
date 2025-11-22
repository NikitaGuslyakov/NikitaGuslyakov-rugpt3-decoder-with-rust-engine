# -*- coding: utf-8 -*-

import rust_hf_decoder
import re

TOKENIZER_DIR = r"C:\Experiments\project\tokenizer_rugpt3large"
INPUT_IDS     = r"C:\Experiments\project\corpus\AA_tagged\wiki_00_ids.txt"
OUTPUT_TEXT   = r"C:\Experiments\project\corpus\AA_tagged\wiki_00_decoded.txt"

MAX_TOKENS = 20000  # 0 = декодировать все ID

SPECIAL_TAGS = [
    "<|intellect|>",
    "<|emotion|>",
    "<|movement|>",
    "<|instinct|>",
    "<|negativeemotion|>"
]

def analyze_tags(text: str) -> None:
    """Анализирует целостность специальных тегов."""
    for tag in SPECIAL_TAGS:
        matches = re.findall(re.escape(tag), text)
        if matches:
            print(f"🟢 Тег '{tag}' присутствует и целый.")
        else:
            print(f"🔴 Тег '{tag}' отсутствует или разрезан.")

def main():
    print("Запускаю Rust-декодер...")
    print(f"Токенайзер: {TOKENIZER_DIR}")
    print(f"Вход (ID): {INPUT_IDS}")
    print(f"Выход:     {OUTPUT_TEXT}")
    print(f"MAX_TOKENS: {MAX_TOKENS}\n")

    rust_hf_decoder.decode_file(
        TOKENIZER_DIR,
        INPUT_IDS,
        OUTPUT_TEXT,
        MAX_TOKENS,
    )

    # Читаем расшифрованный текст и проверяем теги
    decoded_text = open(OUTPUT_TEXT, encoding="utf-8").read()
    analyze_tags(decoded_text)

    print("\nГотово.")
    print(f"Расшифрованный текст: {OUTPUT_TEXT}")

if __name__ == "__main__":
    main()