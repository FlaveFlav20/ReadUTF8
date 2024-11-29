import random

def get_supposed_size(byte):
        if 0xC0 <= byte <= 0xDF:  # 110xxxxx: 2 bytes
            return 2
        elif 0xE0 <= byte <= 0xEF:  # 1110xxxx: 3 bytes
            return 3
        elif 0xF0 <= byte <= 0xF4:  # 11110xxx: 4 bytes
            return 4
        else:
            return 1

def generate_invalid_utf8_sequences():
    invalid_sequences = []

    overlong_sequences = [
        b"\xC0\xAF",
        b"\xE0\x80\xAF",
        b"\xF0\x80\x80\xAF",
        b"\xF8\x80\x80\x80\xAF",
        b"\xFC\x80\x80\x80\x80\xAF"
    ]
    invalid_sequences.extend(overlong_sequences)

    continuation_bytes = [bytes([b]) for b in range(0x80, 0xC0)]
    invalid_sequences.extend(continuation_bytes)

    invalid_sequences.append(b"\x80\x80")
    invalid_sequences.append(b"\x80\xBF")

    invalid_leading_bytes = [bytes([b]) for b in range(0xC0, 0xC2)]
    invalid_sequences.extend(invalid_leading_bytes)

    truncated_sequences = [
        b"\xC2",
        b"\xE0\xA0",
        b"\xF0\x90\x80",
    ]
    invalid_sequences.extend(truncated_sequences)

    non_characters = [
        b"\xEF\xBF\xBF",
        b"\xEF\xBF\xBE",
    ]
    invalid_sequences.extend(non_characters)

    out_of_range_bytes = [bytes([b]) for b in range(0xF5, 0x100)]
    invalid_sequences.extend(out_of_range_bytes)

    return invalid_sequences

invalid_sequences = generate_invalid_utf8_sequences()

from_file: str = "./tests_files/DDHC.txt"
target_file: str = "./tests_files/DDHC_custom_delims_corrupted.txt"
target_file_ref: str = "./tests_files/DDHC_custom_delims_corrupted_ref.txt"

f_from = open(from_file, "r")
f_target = open(target_file, "wb")
f_target_ref = open(target_file_ref, "w")

content: str = f_from.read()
res = bytearray()
res_ref: str = ""

delims: list = ["::", ":;", "|", "éè", "小六号", "毫"]
index: int = 0

corruption_probability = 0.3

for i in range(len(content) - 1):
    if content[i] == '\n':
        res += bytes(delims[index], 'utf-8')
        res_ref += '\n'
        index += 1
        index = index % len(delims)
    elif content[i + 1] == '\n':
        res += bytes(content[i], 'utf-8')
        res_ref += content[i]
    elif random.random() < corruption_probability:
        b = random.choice(invalid_sequences)
        res += b
        for i in range(0, get_supposed_size(b[0])):
            res_ref += '�'
    else:
        res += bytes(content[i], 'utf-8')
        res_ref += content[i]

f_target.write(res)
f_target_ref.write(res_ref)

f_target.close()
f_from.close()