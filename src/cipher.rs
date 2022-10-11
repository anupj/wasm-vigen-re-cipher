// Size of the 'dictionary'
// (all non-control ASCII characters plus '\n' and '\r')
pub(crate) const SIZE: usize = 192;

// Tuple struct wrapper around an array of
// characters of size 192
#[derive(Clone, Copy)]
pub(crate) struct DictWrap(pub(crate) [char; SIZE]);

#[derive(Debug)]
pub(crate) enum ErrorCode {
    InvalidChar(char),
    InvalidIndex(usize),
}

// Creates and returns a new dictionary
// for the Vigenère matrix
impl DictWrap {
    pub(crate) fn new() -> DictWrap {
        // Every ASCII character that !is_control().
        let mut dict = r##"!"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~ ¡¢£¤¥¦§¨©ª«¬­®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ"##.to_string();
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

// Creates and returns a new Vigenère Matrix
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
// using a Vigenère Matrix (vig_mat)
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
    let key_chars: Vec<_> = key_e.to_string().chars().collect();
    let msg_chars: Vec<_> = msg.to_string().chars().collect();

    // encrypt message
    for i in 0..msg_size {
        encrypted_msg.push(vig_matcher(&vig_mat, msg_chars[i], key_chars[i])?);
    }

    Ok(encrypted_msg)
}

// Returns the matching character in the Vigenère Matrix,
// depending on the header (msg_char) and column (key_char)
// characters provided
fn vig_matcher(matrix: &VigMatrixWrap, msg_char: char, key_char: char) -> Result<char, ErrorCode> {
    let index_col = index_finder(msg_char, &matrix)?;
    let index_row = index_finder(key_char, &matrix)?;

    Ok(matrix.0[index_row][index_col])
}

// Returns the index value of a char
// in the Vigenère Matrix
fn index_finder(ch: char, matrix: &VigMatrixWrap) -> Result<usize, ErrorCode> {
    for (index, val) in matrix.0[0].iter().enumerate() {
        if ch == *val {
            return Ok(index);
        }
    }
    Err(ErrorCode::InvalidChar(ch))
}

// Decodes an encoded message (enc_msg) with
// a key (key) and a Vigenère Matrix (vig_mat)
pub(crate) fn decode(
    enc_msg: &str,
    key: &str,
    vig_mat: VigMatrixWrap,
) -> Result<String, ErrorCode> {
    // get size of message and key
    let msg_size = enc_msg.chars().count();
    let key_size = key.chars().count();

    // initialisations
    let mut decrypted_msg = "".to_string();

    // if key has a different size, then complete it
    let mut key_e = key.to_string();
    if msg_size != key_size {
        key_e = complete_key(key, msg_size);
    }

    // convert to char vectors
    let key_chars: Vec<_> = key_e.to_string().chars().collect();
    let msg_chars: Vec<_> = enc_msg.to_string().chars().collect();

    // decrypt message
    for letter in 0..msg_size {
        let mut msg_index = 0;
        let key_index = index_finder(key_chars[letter], &vig_mat)?;
        for c in 0..vig_mat.0.len() {
            if vig_mat.0[key_index][c] == msg_chars[letter] {
                msg_index = c;
            }
        }
        decrypted_msg.push(char_finder(msg_index, &vig_mat)?);
    }

    Ok(decrypted_msg)
}

// Decodes a message (msg) with a key (key)
// using a Vigenère Matrix (vig_mat).
// Returns the blank space char ' ' as '&nbsp;'
// so that consecutive blank spaces are
// rendered properly on the browser
pub(crate) fn decode_web(
    enc_msg: &str,
    key: &str,
    vig_mat: VigMatrixWrap,
) -> Result<String, ErrorCode> {
    let decoded = decode(enc_msg, key, vig_mat)?;
    let mut decoded_web = "".to_string();
    for ch in decoded.chars() {
        match ch {
            ' ' => decoded_web.push_str("&nbsp;"),
            '\n' | '\r' => decoded_web.push_str("<br>"),
            _ => decoded_web.push(ch),
        };
    }
    Ok(decoded_web)
}

// Returns the char value of
// an index in the Vigenère Matrix
fn char_finder(index: usize, mat: &VigMatrixWrap) -> Result<char, ErrorCode> {
    for (idx, val) in mat.0[0].iter().enumerate() {
        if index == idx {
            return Ok(*val);
        }
    }
    Err(ErrorCode::InvalidIndex(index))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_encode() {
        let vig_mat = VigMatrixWrap::new();
        let key = "°¡! RüST íS CóÓL ¡!°";
        let message = "Hello, World!";
        let encoded = encode(message, key, vig_mat).unwrap();
        assert_eq!(message, decode(&encoded, key, vig_mat).unwrap());

        let vig_mat = VigMatrixWrap::new();
        let key = "°¡! RüST íS CóÓL ¡!°";
        let message = "Anup Jadhav";
        let encoded = encode(message, key, vig_mat).unwrap();
        let decoded = decode(&encoded, key, vig_mat).unwrap();
        // println!("key      :##{}##:", key);
        // println!("message  :##{}##:", message);
        // println!("encoded  :##{}##:", encoded);
        // println!("decoded  :##{}##:", decoded);
        assert_eq!(message, decoded);

        let vig_mat = VigMatrixWrap::new();
        let key = "°¡! RüST íS CóÓL ¡!°";
        let message = "!!!!";
        let encoded = encode(message, key, vig_mat).unwrap();
        assert_eq!(message, decode(&encoded, key, vig_mat).unwrap());

        let vig_mat = VigMatrixWrap::new();
        let key = "°¡! RüST íS CóÓL ¡!°";
        let message = "WhátisApp+éars to   be the   problem here__°¿¿¿¿¿!!!!++++{{{{{{{}}}}}}}";
        let encoded = encode(message, key, vig_mat).unwrap();
        assert_eq!(message, decode(&encoded, key, vig_mat).unwrap());
    }

    #[test]
    fn test_complex() {
        let vig_mat = VigMatrixWrap::new();
        let key = "°¡! RüST íS CóÓL ¡!°";
        let message = r##"´+++´[[[    {{{'''''""""()*&^   
            $2374954904890~~~11939455    
            7+a+e{eíóúúááÉú}"}}}]]]"##;
        let encoded = encode(message, key, vig_mat).unwrap();
        assert_eq!(message, decode(&encoded, key, vig_mat).unwrap());
    }
}
