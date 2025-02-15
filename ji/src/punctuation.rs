// # --------

// input (照 standard keyboard) = '

// menu, 漢字 submode == [「,  」]
// menu, 羅字 submode == [‘, ’, ']

// # --------

// input = "

// menu, 漢字 submode == [『, 』, 々, 〱]
// menu, 羅字 submode == [“, ”, "]

// # --------

// input = <

// menu, 漢字 submode == [〈, 《]
// menu, 羅字 submode == [<, «]

// # --------

// input = >

// menu, 漢字 submode == [〉, 》]
// menu, 羅字 submode == [>, »]

// # --------

// input = ;

// direct output, 漢字 submode == ・[U+30FB]
// direct output, 羅字 submode == ;

// # --------

// input = :

// menu, 漢字 submode == [：, ⋯⋯]

// direct output, 羅字 submode == :

// # -------

// input = +

// menu, 漢字 submode == [+, ＋, ⁺]
// menu, 羅字 submode == [+, ⁺]

// # -------

// input = =

// menu, 漢字 submode == [=, ＝, 〓]
// direct output, 羅字 submode == =

// # -------

// input = _

// menu, 漢字 submode == [_, —, ＿, ⁻]
// menu, 羅字 submode == [_, —, ⁻]

// # -------

// input = [

// menu, 漢字 submode == [〔, 【, 〖]
// direct output, 羅字 submode == [

// # -------

// input = ]

// menu, 漢字 submode == [〕, 】, 〗]
// direct output, 羅字 submode == ]

const APOSTROPHE_HANJI_CHARS: [char; 3] = ['「', '」', '\''];
// const APOSTROPHE_LOMAJI_CHARS: [char; 3] = ['‘', '’', '\''];

const QUOTE_HANJI_CHARS: [char; 5] = ['『', '』', '"', '々', '〱'];
// const QUOTE_LOMAJI_CHARS: [char; 3] = ['"', '"', '"'];

const LESS_THAN_HANJI_CHARS: [char; 2] = ['〈', '《'];
const LESS_THAN_LOMAJI_CHARS: [char; 2] = ['<', '«'];

const GREATER_THAN_HANJI_CHARS: [char; 2] = ['〉', '》'];
const GREATER_THAN_LOMAJI_CHARS: [char; 2] = ['>', '»'];

// const COLON_HANJI_CHARS: [char; 2] = ['：', '⋯'];
// const COLON_LOMAJI_CHAR: char = ':';

const PLUS_HANJI_CHARS: [char; 3] = ['+', '＋', '⁺'];
const PLUS_LOMAJI_CHARS: [char; 2] = ['+', '⁺'];

const EQUALS_HANJI_CHARS: [char; 3] = ['=', '＝', '〓'];
// const EQUALS_LOMAJI_CHAR: char = '=';

const UNDERSCORE_HANJI_CHARS: [char; 4] = ['_', '—', '＿', '⁻'];
const UNDERSCORE_LOMAJI_CHARS: [char; 3] = ['_', '—', '⁻'];

const LEFT_BRACKET_HANJI_CHARS: [char; 3] = ['〔', '【', '〖'];
// const LEFT_BRACKET_LOMAJI_CHAR: char = '[';

const RIGHT_BRACKET_HANJI_CHARS: [char; 3] = ['〕', '】', '〗'];
// const RIGHT_BRACKET_LOMAJI_CHAR: char = ']';

pub fn get_lomaji_chars(key: char) -> Option<Vec<char>> {
    match key {
        // '\'' => Some(APOSTROPHE_LOMAJI_CHARS.to_vec()),
        // '"' => Some(QUOTE_LOMAJI_CHARS.to_vec()),
        '<' => Some(LESS_THAN_LOMAJI_CHARS.to_vec()),
        '>' => Some(GREATER_THAN_LOMAJI_CHARS.to_vec()),
        '+' => Some(PLUS_LOMAJI_CHARS.to_vec()),
        '_' => Some(UNDERSCORE_LOMAJI_CHARS.to_vec()),
        _ => None,
    }
}

pub fn get_hanji_chars(key: char) -> Option<Vec<char>> {
    match key {
        '\'' => Some(APOSTROPHE_HANJI_CHARS.to_vec()),
        '"' => Some(QUOTE_HANJI_CHARS.to_vec()),
        '<' => Some(LESS_THAN_HANJI_CHARS.to_vec()),
        '>' => Some(GREATER_THAN_HANJI_CHARS.to_vec()),
        '+' => Some(PLUS_HANJI_CHARS.to_vec()),
        '=' => Some(EQUALS_HANJI_CHARS.to_vec()),
        '_' => Some(UNDERSCORE_HANJI_CHARS.to_vec()),
        '[' => Some(LEFT_BRACKET_HANJI_CHARS.to_vec()),
        ']' => Some(RIGHT_BRACKET_HANJI_CHARS.to_vec()),
        _ => None,
    }
}