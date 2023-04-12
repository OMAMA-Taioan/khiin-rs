import csv
from functools import cmp_to_key
import locale
from typing import List

from tables import Emoji, Frequency, Symbols, Conversion


def compare(x, y):
    return (x > y) - (x < y)


def freq_sort(left: Frequency, right: Frequency):
    cmp = -compare(left.freq, right.freq)
    return cmp if cmp != 0 else compare(left.chhan_id, right.chhan_id)


def conv_sort(left: Conversion, right: Conversion):
    cmp = compare(locale.strxfrm(left.input), locale.strxfrm(right.input))
    return cmp if cmp != 0 else -compare(left.weight, right.weight)


freq_sort_key = cmp_to_key(freq_sort)
conv_sort_key = cmp_to_key(conv_sort)
syls_sort_key = locale.strxfrm


def parse_freq_csv(csv_file, exclude_zeros=False) -> List[Frequency]:
    data: List[Frequency] = []
    with open(csv_file) as f:
        reader = csv.DictReader(f, skipinitialspace=True)
        for row in reader:
            data.append(Frequency(row))
    if exclude_zeros is True:
        data = [item for item in data if item.freq != 0]
    return sorted(data, key=freq_sort_key)


def parse_conv_csv(csv_file, sort_hanji_first) -> List[Conversion]:
    data: List[Conversion] = []
    with open(csv_file) as f:
        reader = csv.DictReader(f, skipinitialspace=True)
        for row in reader:
            data.append(Conversion(row, fixed_weights=sort_hanji_first))
    return sorted(data, key=conv_sort_key)


def parse_syls_txt(txt_file) -> List[str]:
    data = []
    if not txt_file:
        return data
    with open(txt_file) as f:
        data = [line.rstrip() for line in f]
    return sorted(list(set(data)), key=syls_sort_key)


def parse_symbols_tsv(tsv_file) -> List[Symbols]:
    dat = []
    with open(tsv_file, 'r') as f:
        rows = csv.DictReader(f, delimiter='\t')
        for row in rows:
            dat.append(Symbols(row))
    return dat


def parse_emoji_csv(csv_file) -> List[Emoji]:
    dat = []
    with open(csv_file, 'r') as f:
        rows = csv.DictReader(f)
        filter(lambda x: x['recent'] == 1, rows)
        for row in rows:
            dat.append(Emoji(row))
    return dat
