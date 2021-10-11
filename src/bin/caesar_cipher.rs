/*

How it works:

A monoalphabetic substitution used by Julius.

Replace characters in a word with the characters n places (mod alphabet length)
from that character in the alphabet.

*/

fn caesar(cipher: &str, shift: u8) -> String {
    cipher
        .chars()
        // |c| is a "closure" (like a JavaScript arrow function)
        .map(|c| {
            if c.is_ascii_alphabetic() {
                // Initialize to the starting letter of the lower or upper case aphabet
                // Characters are referenced by an array of numbers
                // i.e. lower case alphabet could be 0-25, upper case could be 26-52
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };

                /*
                1. convert c to integer
                2. add (shift - alphabet starting index)
                3. calculate new caracter index modulo 26 (alphabet length)
                4. Add new character to alphabet start index
                5. Convert integer back to a character
                */
                (first + (c as u8 + shift - first) % 26) as char
            } else {
                // If character is not ascii, just return it.
                c
            }
        })
        // collect() array of characters back into a string
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(caesar("", 13), "");
    }

    #[test]
    fn caesar_rot_13() {
        assert_eq!(caesar("rust", 13), "ehfg");
    }

    #[test]
    fn caesar_unicode() {
        assert_eq!(caesar("attack at dawn 攻", 5), "fyyfhp fy ifbs 攻");
    }
}
