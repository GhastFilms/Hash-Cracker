#[allow(dead_code)]
pub mod alphabet {
    const LOWERCASE: [&'static str; 27] = ["","a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    const UPPERCASE: [&'static str; 26] = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
    const SPECIALS:  [&'static str; 33] = [" ", "!", "\"", "#", "$", "%", "&", "'", "(", ")", "*", "+", ",", "-", ".", "/", "[", "\\", "]", "^", "_", "`", ":", ";", "<", "=", ">", "?", "@", "{", "|", "}", "~"];
    const NUMBERS:   [&'static str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    
    pub fn get_all() -> Vec<&'static str> {
        let r = LOWERCASE.to_vec();
        //r = r.append(UPPERCASE.to_vec());
        //r = r.append(SPECIALS.to_vec()  );
        //r = r.append(NUMBERS.to_vec()   );
        r
    }    

    pub fn get_lowercase() -> Vec<&'static str> { LOWERCASE.to_vec() }
    pub fn get_uppercase() -> Vec<&'static str> { UPPERCASE.to_vec() }
    pub fn get_specials()  -> Vec<&'static str> { SPECIALS.to_vec()  }
    pub fn get_numbers()   -> Vec<&'static str> { NUMBERS.to_vec()   }
}
