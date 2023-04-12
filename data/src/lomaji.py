import itertools
import re
from typing import List
import unicodedata

ASCII_SUBS = [
    (r'\u207F', 'nn'),
    (r'\u0358', 'u'),
    (r'\u0301', '2'),
    (r'\u0300', '3'),
    (r'\u0302', '5'),
    (r'\u0304', '7'),
    (r'\u030D', '8'),
    (r'\u0306', '9'),
    (r'\u0324', 'r'),
]

TELEX_MAP = {
    '2': 's',
    '3': 'f',
    '5': 'l',
    '7': 'j',
    '8': 'j',
    '9': 'w'
}

KIP_SUBS = [
    (r'ch', 'ts'),
    (r'oa', 'ua'),
    (r'oe', 'ue'),
    (r'ou', 'oo'),
    (r'eng', 'ing'),
    (r'ek', 'ik'),
]


def poj_to_ascii(text):
    text = unicodedata.normalize('NFD', text)
    for sub in ASCII_SUBS:
        text = re.sub(sub[0], sub[1], text)
    text = re.sub(r'([A-Za-z]+)(\d)([A-Za-z]+)', r'\1\3\2', text).lower()
    return text


def poj_to_fhl_qstring(text):
    text = poj_to_ascii(text)
    if re.search(r' ', text) is not None:
        text = re.sub(r'\d| +', '', text)
    return text


def poj_to_fhl_reading(text):
    text = poj_to_ascii(text)
    for sub in KIP_SUBS:
        text = re.sub(sub[0], sub[1], text)
    text = re.sub(r' ', '-', text)
    return text


def poj_to_khiin(syllable, strip_tones):
    decomposed = unicodedata.normalize('NFD', syllable)
    for sub in ASCII_SUBS:
        decomposed = re.sub(sub[0], sub[1], decomposed)
    numeric = re.sub(r'([A-Za-z]+)(\d)([A-Za-z]+)',
                     r'\1\3\2', decomposed).lower()
    toneless = numeric
    telex = numeric

    match = re.search(r'\d$', numeric)
    if match is not None:
        toneless = numeric[:-1]
        telex = toneless + TELEX_MAP[numeric[-1]]

    if numeric == toneless:
        return [[toneless], [toneless]]

    if strip_tones:
        return [[numeric, toneless], [telex, toneless]]

    return [[numeric], [telex]]


def always_same(n, val):
    for i in range(n):
        yield val


def to_input_sequences(word: str) -> list[(str, str, int)]:
    syls = re.compile('[ -]').split(word)
    n_syls = len(syls)
    strip_tones = True  # len(syls) > 1
    syls = list(map(poj_to_khiin, syls, always_same(len(syls), strip_tones)))
    numeric_syls = [x[0] for x in syls]
    telex_syls = [x[1] for x in syls]
    numeric = [''.join(ea) for ea in list(itertools.product(*numeric_syls))]
    telex = [''.join(ea) for ea in list(itertools.product(*telex_syls))]
    syl_count = [n_syls] * len(numeric)
    return list(zip(numeric, telex, syl_count))


def has_hanji(string) -> bool:
    return any(ord(c) > 0x2e80 for c in string)


def normalize_loji(input, strip_tones=False) -> str:
    input = unicodedata.normalize('NFD', input)
    input = input.replace('-', ' ').replace('·', '').lower()

    if strip_tones is True:
        input = re.sub(r'[\u0300-\u030d]', '', input)

    return unicodedata.normalize('NFC', input)


def get_tone_position(syl) -> int:
    found = re.search(r"o[ae][a-z]", syl)
    if found is not None:
        return found.start() + 1
    for x in ['o', 'a', 'e', 'u', 'i', 'ṳ', 'n', 'm']:
        found = syl.find(x)
        if found > -1:
            return found
    return -1


def add_all_tones(syl) -> List[str]:
    ret = [syl]
    if re.search(r"[ptkh]ⁿ?$", syl):
        tones = ['\u030d']
    else:
        tones = ['\u0301', '\u0300', '\u0302', '\u0304', '\u0306']
    pos = get_tone_position(syl) + 1
    for tone in tones:
        syl_tone = syl[:pos] + tone + syl[pos:]
        syl_tone = unicodedata.normalize('NFC', syl_tone)
        ret.append(syl_tone)
    return ret
