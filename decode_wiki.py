# -*- coding: utf-8 -*-

import rust_hf_decoder

TOKENIZER_DIR = r"C:\Experiments\project\tokenizer_rugpt3large"
INPUT_IDS     = r"C:\Experiments\project\corpus\AA_tagged\wiki_00_ids.txt"
OUTPUT_TEXT   = r"C:\Experiments\project\corpus\AA_tagged\wiki_00_decoded.txt"

MAX_TOKENS = 20000  # 0 = декодировать все ID

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

    print("\nГотово.")
    print(f"Раскодированный текст: {OUTPUT_TEXT}")

if __name__ == "__main__":
    main()

