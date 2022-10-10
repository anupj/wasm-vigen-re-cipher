// Size of the 'dictionary'
// (all non-control ASCII characters plus '\n' and '\r')
pub(crate) const SIZE: usize = 192;

// Tuple struct wrapper around an array of
// characters of size 192
#[derive(Clone, Copy)]
pub(crate) struct DictWrap(pub(crate) [char; SIZE]);

#[derive(Clone, Copy)]
pub(crate) enum ErrorCode {
    InvalidChar(char),
    InvalidIndex(usize),
}

// Creates and returns a new dictionary
// for the Vigenére matrix
impl DictWrap {
    pub(crate) fn new() -> DictWrap {
        // Every ASCII character that !is_control().
        let mut dict = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~ ¡¢£¤¥¦§¨©ª«¬­®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ"##.to_string();
        // Add carriage return to support in web textarea
        dict.push('\n');
        dict.push('\r');
        let mut dict_char_arr = [' '; SIZE];
        for (idx, ch) in dict.chars().enumerate() {
            dict_char_arr[idx] = ch;
        }
        DictWrap(dict_char_arr)
    }

    pub(crate) fn get_string(&self) -> String {
        let mut s = String::new();
        for ch in self.0 {
            s.push(ch);
        }
        s
    }
}

// Again using the `Newtype Pattern`, create
// a tuple struct wrapper around the 2D array
#[derive(Clone, Copy)]
pub(crate) struct VigMatrixWrap(pub(crate) [[char; SIZE]; SIZE]);

// Creates and returns a new Vigenére Matrix
impl VigMatrixWrap {
    pub(crate) fn new() -> VigMatrixWrap {
        let mut matrix: VigMatrixWrap = VigMatrixWrap([[' '; SIZE]; SIZE]);
        // Get the array of dictionary characters
        let binding = DictWrap::new().0;
        // Create a cyclical (i.e. never ending) iterator
        // cycle() repeats an interator endlessly
        let mut acc = binding.iter().cycle();

        for r in 0..matrix.0.len() {
            for c in 0..matrix.0.len() {
                matrix.0[r][c] = *acc.next().unwrap();
            }
            // this will start the next
            // loop at the next character
            // as the first item
            acc.next();
        }
        matrix
    }
}

// Completes the key if the key size is not
// the same as the message.
// In other words, extends the key String to
// be the same size as the message String.
fn complete_key(key: &str, msg_size: usize) -> String {
    // cycle() repeats an interator endlessly
    let mut key_chars = key.chars().cycle();
    let mut new_key = "".to_string();
    for _ in 0..msg_size {
        new_key.push(key_chars.next().unwrap());
    }
    new_key
}

// Encodes a message (msg) with a key(key)
// using a Vigenére matrix (vig_mat)
pub(crate) fn encode(msg: &str, key: &str, vig_mat: VigMatrixWrap) -> Result<String, ErrorCode> {
    // get size of message and key
    let msg_size = msg.chars().count();
    let key_size = key.chars().count();

    // initialisations
    let mut encrypted_msg = "".to_string();

    // if key has a different size, then complete it
    let mut key_e = key.to_string();
    if msg_size != key_size {
        key_e = complete_key(key, msg_size);
    }

    // convert to char vectors
    let key_chars: Vec<_> = key_e.chars().collect();
    let msg_chars: Vec<_> = msg.to_string().chars().collect();

    // encrypt message
    for i in 0..msg_size {
        encrypted_msg.push(vig_matcher(&vig_mat, msg_chars[i], key_chars[i])?);
    }

    Ok(encrypted_msg)
}

// Returns the matching character in the Vigenére matrix,
// depending on the header (msg_char) and column (key_char)
// characters provided
fn vig_matcher(matrix: &VigMatrixWrap, msg_char: char, key_char: char) -> Result<char, ErrorCode> {
    let index_col = index_finder(msg_char, &matrix)?;
    let index_row = index_finder(key_char, &matrix)?;

    Ok(matrix.0[index_row][index_col])
}

// Returns the index value of a char
// in the Vigenére matrix
fn index_finder(ch: char, matrix: &VigMatrixWrap) -> Result<usize, ErrorCode> {
    for (index, val) in matrix.0[0].iter().enumerate() {
        if ch == *val {
            return Ok(index);
        }
    }
    Err(ErrorCode::InvalidChar(ch))
}

// Decodes an encoded message (enc_msg) with
// a key (key) and a Vigenére Matrix (vig_mat)
pub(crate) fn decode(
    enc_msg: &str,
    key: &str,
    vig_mat: VigMatrixWrap,
) -> Result<String, ErrorCode> {
    // get size of message and key
    let msg_size = enc_msg.chars().count();
    todo!();
}
