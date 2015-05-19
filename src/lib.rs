
/// Hepburn romanization for input string
/// @todo return an iterator with result
/// @todo add katakana
/// @todo unicode unwrap for range comparison, byte detection for digraphs
/// @todo benchmarking
pub fn romanize( input: &str ) -> String {
    /*
    for b in input.as_bytes() {
        print!("{:x}, ", b);
    }
    */

    let mut result = String::new();
    let mut it = input.chars().peekable();
    loop {
        //println!("{} = {:x}, {}", value, value as u32, 0x305f == value as u32);

        match it.next() {
            // Hiragana
            Some('あ') => result.push_str("a"),
            Some('い') => result.push_str("i"),
            Some('う') => result.push_str("u"),
            Some('え') => result.push_str("e"),
            Some('お') => result.push_str("o"),

            Some('か') => result.push_str("ka"),
            Some('き') => match it.peek() {
                Some(&'ゃ') => {
                    it.next();
                    result.push_str("kya");
                },
                Some(&'ゅ') => {
                    it.next();
                    result.push_str("kyu");
                },
                Some(&'ょ') => {
                    it.next();
                    result.push_str("kyo");
                },
                _ => result.push_str("ki"),
            },
            Some('く') => result.push_str("ku"),
            Some('け') => result.push_str("ke"),
            Some('こ') => result.push_str("ko"),

            Some('が') => result.push_str("ga"),
            Some('ぎ') => match it.peek() {
                Some(&'ゃ') => {
                    it.next();
                    result.push_str("gya");
                },
                Some(&'ゅ') => {
                    it.next();
                    result.push_str("gyu");
                },
                Some(&'ょ') => {
                    it.next();
                    result.push_str("gyo");
                },
                _ => result.push_str("gi"),
            },
            Some('ぐ') => result.push_str("gu"),
            Some('げ') => result.push_str("ge"),
            Some('ご') => result.push_str("go"),

            Some('さ') => result.push_str("sa"),
            Some('し') => match it.peek() {
                Some(&'ゃ') => {
                    it.next();
                    result.push_str("sha");
                },
                Some(&'ゅ') => {
                    it.next();
                    result.push_str("shu");
                },
                Some(&'ょ') => {
                    it.next();
                    result.push_str("sho");
                },
                _ => result.push_str("shi"),
            },
            Some('す') => result.push_str("su"),
            Some('せ') => result.push_str("se"),
            Some('そ') => result.push_str("so"),

            Some('ざ') => result.push_str("za"),
            Some('じ') => match it.peek() {
                Some(&'ゃ') => {
                    it.next();
                    result.push_str("ja");
                },
                Some(&'ゅ') => {
                    it.next();
                    result.push_str("ju");
                },
                Some(&'ょ') => {
                    it.next();
                    result.push_str("jo");
                },
                _ => result.push_str("ji"),
            },
            Some('ず') => result.push_str("zu"),
            Some('ぜ') => result.push_str("ze"),
            Some('ぞ') => result.push_str("zo"),

            Some('た') => result.push_str("ta"),
            Some('ち') => match it.peek() {
                Some(&'ゃ') => {
                    it.next();
                    result.push_str("cha");
                },
                Some(&'ゅ') => {
                    it.next();
                    result.push_str("chu");
                },
                Some(&'ょ') => {
                    it.next();
                    result.push_str("cho");
                },
                _ => result.push_str("chi"),
            },
            Some('つ') => result.push_str("tsu"),
            Some('て') => result.push_str("te"),
            Some('と') => result.push_str("to"),

            Some('だ') => result.push_str("da"),
            Some('ぢ') => result.push_str("ji"),
            Some('づ') => result.push_str("zu"),
            Some('で') => result.push_str("de"),
            Some('ど') => result.push_str("do"),

            Some('な') => result.push_str("na"),
            Some('に') => match it.peek() {
                Some(&'ゃ') => {
                    it.next();
                    result.push_str("nya");
                },
                Some(&'ゅ') => {
                    it.next();
                    result.push_str("nyu");
                },
                Some(&'ょ') => {
                    it.next();
                    result.push_str("nyo");
                },
                _ => result.push_str("ni"),
            },
            Some('ぬ') => result.push_str("nu"),
            Some('ね') => result.push_str("ne"),
            Some('の') => result.push_str("no"),

            Some('は') => result.push_str("ha"),
            Some('ひ') => match it.peek() {
                Some(&'ゃ') => {
                    it.next();
                    result.push_str("hya");
                },
                Some(&'ゅ') => {
                    it.next();
                    result.push_str("hyu");
                },
                Some(&'ょ') => {
                    it.next();
                    result.push_str("hyo");
                },
                _ => result.push_str("hi"),
            },
            Some('ふ') => result.push_str("hu"),
            Some('へ') => result.push_str("he"),
            Some('ほ') => result.push_str("ho"),

            Some('ば') => result.push_str("ba"),
            Some('び') => match it.peek() {
                Some(&'ゃ') => {
                    it.next();
                    result.push_str("pya");
                },
                Some(&'ゅ') => {
                    it.next();
                    result.push_str("pyu");
                },
                Some(&'ょ') => {
                    it.next();
                    result.push_str("pyo");
                },
                _ => result.push_str("pi"),
            },
            Some('ぶ') => result.push_str("bu"),
            Some('べ') => result.push_str("be"),
            Some('ぼ') => result.push_str("bo"),

            Some('ぱ') => result.push_str("pa"),
            Some('ぴ') => match it.peek() {
                Some(&'ゃ') => {
                    it.next();
                    result.push_str("pya");
                },
                Some(&'ゅ') => {
                    it.next();
                    result.push_str("pyu");
                },
                Some(&'ょ') => {
                    it.next();
                    result.push_str("pyo");
                },
                _ => result.push_str("pi"),
            },
            Some('ぷ') => result.push_str("pu"),
            Some('ぺ') => result.push_str("pe"),
            Some('ぽ') => result.push_str("po"),

            Some('ま') => result.push_str("ma"),
            Some('み') => match it.peek() {
                Some(&'ゃ') => {
                    it.next();
                    result.push_str("mya");
                },
                Some(&'ゅ') => {
                    it.next();
                    result.push_str("myu");
                },
                Some(&'ょ') => {
                    it.next();
                    result.push_str("myo");
                },
                _ => result.push_str("mi"),
            },
            Some('む') => result.push_str("mu"),
            Some('め') => result.push_str("me"),
            Some('も') => result.push_str("mo"),

            Some('や') => result.push_str("ya"),
            Some('ゆ') => result.push_str("yu"),
            Some('よ') => result.push_str("yo"),

            Some('ら') => result.push_str("ra"),
            Some('り') => match it.peek() {
                Some(&'ゃ') => {
                    it.next();
                    result.push_str("rya");
                },
                Some(&'ゅ') => {
                    it.next();
                    result.push_str("ryu");
                },
                Some(&'ょ') => {
                    it.next();
                    result.push_str("ryo");
                },
                _ => result.push_str("ri"),
            },
            Some('る') => result.push_str("ru"),
            Some('れ') => result.push_str("re"),
            Some('ろ') => result.push_str("ro"),

            Some('わ') => result.push_str("wa"),
            Some('を') => result.push_str("wo"),

            Some('ん') => result.push('n'),

            // Copy other characters unchanged
            Some( value ) => result.push( value ),
            None => break,
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_handles_single_phonemes() {
        assert_eq!( "ru", romanize( "る" ) );
    }

    #[test]
    fn it_handles_digraphs() {
        assert_eq!( "kurya", romanize( "くりゃ" ) );
    }

    #[test]
    fn it_handles_voiced_mark() {
        assert_eq!( "gu", romanize( "ぐ" ) );
    }

    #[test]
    fn it_handles_semi_voiced_mark() {
        assert_eq!( "pa", romanize( "ぱ" ) );
    }
}
