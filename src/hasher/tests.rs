#[cfg(test)]
mod tests {
    use std::fs::File;
    use crate::hasher::*;
    #[test]
    fn hasher_check() {
        let f = match File::open("Cargo.toml") {
            Ok(s) => s,
            Err(e) => panic!("{}",e),
        };
        let mut h: Hasher = Hasher {
            output: f,
            hashes: HashSet::new(),
        };
        h.hashes.insert("909104CDB5B06AF2606ED4A197B07D09D5EF9A4AAD97780C2FE48053BCE2BE52".to_string());
        let s: String = "yeet".to_string();
        match h.check(&s) {
            Some(S) => {
                assert_eq!(S, format!("{}: {}\n", Hasher::hash(&s), s));
                assert!(h.hashes.is_empty());
            },
            None => panic!("e"),
        }
    }

    #[test]
    fn hasher_hash() {
        let hashes: [(String, String); 3] = [
            (String::from("909104CDB5B06AF2606ED4A197B07D09D5EF9A4AAD97780C2FE48053BCE2BE52"), String::from("yeet")),
            (String::from("8BC3CF7978B357A7E28B6F47EDE65C868CA07727676979A7A1964270CAD641AB"), String::from("tV%[C@k({r5&mcn6M!tuJ3gq")), 
            (String::from("A4F866B9B09C9CC784480F6E186CC6D49689B286426A0D075C3D4BC7FEA982B8"), String::from("`S@WgZ8b[GeSB72tV.kq2e\"&<Aqdzf@f")) 
        ];

        for x in 0..hashes.len() {
            assert_eq!(hashes[x].0, Hasher::hash(&hashes[x].1));
            assert_ne!(String::from("yote"), Hasher::hash(&hashes[x].1));
            assert_ne!(String::from(""), Hasher::hash(&hashes[x].1));
        }
    }

    #[test]
    fn hasher_to_hex() {
        let hex_strings: [(String, String); 3] = [
            (String::from("79656574"), String::from("yeet")),
            (String::from("7456255B43406B287B7235266D636E364D2174754A336771"), String::from("tV%[C@k({r5&mcn6M!tuJ3gq")), 
            (String::from("60534057675A38625B47655342373274562E6B71326522263C4171647A664066"), String::from("`S@WgZ8b[GeSB72tV.kq2e\"&<Aqdzf@f")) 
        ];
        for x in 0..hex_strings.len() {
            assert_eq!(hex_strings[x].0, Hasher::to_hex(hex_strings[x].1.as_bytes().to_vec()));
            assert_ne!(String::from("yote"), Hasher::to_hex(hex_strings[x].1.as_bytes().to_vec()));
            assert_ne!(String::from(""), Hasher::to_hex(hex_strings[x].1.as_bytes().to_vec()));
        }
    }
}