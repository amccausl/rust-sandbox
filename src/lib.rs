
/// Hepburn romanization for input string
/// @todo translate strange double-width roman characters
/// @todo benchmarking
/// @todo take an peekable iterator as input
/// @todo return an iterator with result
/// @todo unicode unwrap for range comparison optimizations
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
            Some('あ') => result.push('a'),
            Some('い') => result.push('i'),
            Some('う') => result.push('u'),
            Some('え') => result.push('e'),
            Some('お') => result.push('o'),

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
            Some('づ') => result.push_str("dzu"),
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
            Some('ふ') => result.push_str("fu"),
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
            Some('を') => result.push('o'),

            Some('ん') => result.push('n'),

            // Katakana
            Some('ア') => result.push('a'),
            Some('イ') => result.push('i'),
            Some('ウ') => result.push('u'),
            Some('エ') => result.push('e'),
            Some('オ') => result.push('o'),

            Some('カ') => result.push_str("ka"),
            Some('キ') => match it.peek() {
                Some(&'ャ') => {
                    it.next();
                    result.push_str("kya");
                },
                Some(&'ュ') => {
                    it.next();
                    result.push_str("kyu");
                },
                Some(&'ョ') => {
                    it.next();
                    result.push_str("kyo");
                },
                _ => result.push_str("ki"),
            },
            Some('ク') => result.push_str("ku"),
            Some('ケ') => result.push_str("ke"),
            Some('コ') => result.push_str("ko"),

            Some('ガ') => result.push_str("ga"),
            Some('ギ') => match it.peek() {
                Some(&'ャ') => {
                    it.next();
                    result.push_str("gya");
                },
                Some(&'ュ') => {
                    it.next();
                    result.push_str("gyu");
                },
                Some(&'ョ') => {
                    it.next();
                    result.push_str("gyo");
                },
                _ => result.push_str("gi"),
            },
            Some('グ') => result.push_str("gu"),
            Some('ゲ') => result.push_str("ge"),
            Some('ゴ') => result.push_str("go"),

            Some('サ') => result.push_str("sa"),
            Some('シ') => match it.peek() {
                Some(&'ャ') => {
                    it.next();
                    result.push_str("sha");
                },
                Some(&'ュ') => {
                    it.next();
                    result.push_str("shu");
                },
                Some(&'ョ') => {
                    it.next();
                    result.push_str("sho");
                },
                _ => result.push_str("shi"),
            },
            Some('ス') => result.push_str("su"),
            Some('セ') => result.push_str("se"),
            Some('ソ') => result.push_str("so"),

            Some('ザ') => result.push_str("za"),
            Some('ジ') => match it.peek() {
                Some(&'ャ') => {
                    it.next();
                    result.push_str("ja");
                },
                Some(&'ュ') => {
                    it.next();
                    result.push_str("ju");
                },
                Some(&'ョ') => {
                    it.next();
                    result.push_str("jo");
                },
                _ => result.push_str("zi"),
            },
            Some('ズ') => result.push_str("zu"),
            Some('ゼ') => result.push_str("ze"),
            Some('ゾ') => result.push_str("zo"),

            Some('タ') => result.push_str("ta"),
            Some('チ') => match it.peek() {
                Some(&'ャ') => {
                    it.next();
                    result.push_str("cha");
                },
                Some(&'ュ') => {
                    it.next();
                    result.push_str("chu");
                },
                Some(&'ョ') => {
                    it.next();
                    result.push_str("cho");
                },
                _ => result.push_str("chi"),
            },
            Some('ツ') => result.push_str("tsu"),
            Some('テ') => result.push_str("te"),
            Some('ト') => result.push_str("to"),

            Some('ダ') => result.push_str("da"),
            Some('ヂ') => match it.peek() {
                Some(&'ャ') => {
                    it.next();
                    result.push_str("ja");
                },
                Some(&'ュ') => {
                    it.next();
                    result.push_str("ju");
                },
                Some(&'ョ') => {
                    it.next();
                    result.push_str("jo");
                },
                _ => result.push_str("ji"),
            },
            Some('ヅ') => result.push_str("zu"),
            Some('デ') => result.push_str("de"),
            Some('ド') => result.push_str("do"),

            Some('ナ') => result.push_str("na"),
            Some('ニ') => match it.peek() {
                Some(&'ャ') => {
                    it.next();
                    result.push_str("nya");
                },
                Some(&'ュ') => {
                    it.next();
                    result.push_str("nyu");
                },
                Some(&'ョ') => {
                    it.next();
                    result.push_str("nyo");
                },
                _ => result.push_str("ni"),
            },
            Some('ヌ') => result.push_str("nu"),
            Some('ネ') => result.push_str("ne"),
            Some('ノ') => result.push_str("no"),

            Some('ハ') => result.push_str("ha"),
            Some('ヒ') => match it.peek() {
                Some(&'ャ') => {
                    it.next();
                    result.push_str("hya");
                },
                Some(&'ュ') => {
                    it.next();
                    result.push_str("hyu");
                },
                Some(&'ョ') => {
                    it.next();
                    result.push_str("hyo");
                },
                _ => result.push_str("hi"),
            },
            Some('フ') => result.push_str("fu"),
            Some('ヘ') => result.push_str("he"),
            Some('ホ') => result.push_str("ho"),

            Some('バ') => result.push_str("ba"),
            Some('ビ') => match it.peek() {
                Some(&'ャ') => {
                    it.next();
                    result.push_str("bya");
                },
                Some(&'ュ') => {
                    it.next();
                    result.push_str("byu");
                },
                Some(&'ョ') => {
                    it.next();
                    result.push_str("byo");
                },
                _ => result.push_str("bi"),
            },
            Some('ブ') => result.push_str("bu"),
            Some('ベ') => result.push_str("be"),
            Some('ボ') => result.push_str("bo"),

            Some('パ') => result.push_str("pa"),
            Some('ピ') => match it.peek() {
                Some(&'ャ') => {
                    it.next();
                    result.push_str("pya");
                },
                Some(&'ュ') => {
                    it.next();
                    result.push_str("pyu");
                },
                Some(&'ョ') => {
                    it.next();
                    result.push_str("pyo");
                },
                _ => result.push_str("pi"),
            },
            Some('プ') => result.push_str("pu"),
            Some('ペ') => result.push_str("pe"),
            Some('ポ') => result.push_str("po"),

            Some('マ') => result.push_str("ma"),
            Some('ミ') => match it.peek() {
                Some(&'ャ') => {
                    it.next();
                    result.push_str("mya");
                },
                Some(&'ュ') => {
                    it.next();
                    result.push_str("myu");
                },
                Some(&'ョ') => {
                    it.next();
                    result.push_str("myo");
                },
                _ => result.push_str("mi"),
            },
            Some('ム') => result.push_str("mu"),
            Some('メ') => result.push_str("me"),
            Some('モ') => result.push_str("mo"),

            Some('ヤ') => result.push_str("ya"),
            Some('ユ') => result.push_str("yu"),
            Some('ヨ') => result.push_str("yo"),

            Some('ラ') => result.push_str("ra"),
            Some('リ') => match it.peek() {
                Some(&'ャ') => {
                    it.next();
                    result.push_str("rya");
                },
                Some(&'ュ') => {
                    it.next();
                    result.push_str("ryu");
                },
                Some(&'ョ') => {
                    it.next();
                    result.push_str("ryo");
                },
                _ => result.push_str("ri"),
            },
            Some('ル') => result.push_str("ru"),
            Some('レ') => result.push_str("re"),
            Some('ロ') => result.push_str("ro"),

            Some('ワ') => result.push_str("wa"),
            Some('ヰ') => result.push_str("wi"),
            Some('ヱ') => result.push_str("we"),
            Some('ヲ') => result.push_str("wo"),

            Some('ン') => result.push('n'),

            // Copy other characters unchanged
            Some( value ) => result.push( value ),
            None => break,
        }

        // Detect long vowel indicator from loan words
        // @todo should only run when last character was replaced
        match it.peek() {
            Some( &value ) => {
                if value == 'ー' || value == '一' {
                    match result.pop() {
                        Some( 'a' ) => {
                            it.next();
                            result.push( 'ā' );
                        },
                        Some( 'e' ) => {
                            it.next();
                            result.push( 'ē' );
                        },
                        Some( 'i' ) => {
                            it.next();
                            result.push( 'ī' );
                        },
                        Some( 'o' ) => {
                            it.next();
                            result.push( 'ō' );
                        },
                        Some( 'u' ) => {
                            it.next();
                            result.push( 'ū' );
                        },
                        // This shouldn't happen, but we just add the character back for return
                        Some( value ) => {
                            result.push( value );
                        },
                        None => {},
                    };
                }
            },
            None => {},
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_handles_monographs() {
        assert_eq!( "ru", romanize( "る" ) );
    }

    #[test]
    fn it_handles_digraphs() {
        assert_eq!( "rya", romanize( "りゃ" ) );
    }

    #[test]
    fn it_handles_voiced_mark() {
        assert_eq!( "gu", romanize( "ぐ" ) );
    }

    #[test]
    fn it_handles_semi_voiced_mark() {
        assert_eq!( "pa", romanize( "ぱ" ) );
    }

    #[test]
    fn it_handles_long_vowel() {
        assert_eq!( "mā", romanize( "マー" ) )
    }

    #[test]
    fn it_romanizes_chinese_loan() {
        assert_eq!( "mājan", romanize( "マージャン" ) );
        assert_eq!( "chāhan", romanize( "チャーハン" ) );
        assert_eq!( "chāshū", romanize( "チャーシュー" ) );
        assert_eq!( "shūmai", romanize( "シューマイ" ) );
    }
}
