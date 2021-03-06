#![feature(plugin)]
#![plugin(peg_syntax_ext)]

peg! parse(r#"

#[pub]
dec_byte -> u8
    = match_str:$([0-9]*<,3>) {?
        let val: u64 = match_str.parse().unwrap();

        // only let this rule match if the value is in range 0..255
        if val <= 255 {
            Ok(val as u8)
        } else {
            // the message explains what the rule expected and is used in the parse error
            Err("decimal byte")
        }
    }

tag -> &'input str
    = $([a-z]+)

#[pub]
xml
    = "<" open:tag ">" xml* "</" close:tag ">" {?
        if open == close {
            Ok(())
        } else {
            // TODO this has to be a `&'static str`, so we can't use a dynamic string
            Err("matching close tag")
        }
    }

"#);

#[test]
fn dec_byte() {
    assert_eq!(parse::dec_byte("0"), Ok(0));
    assert_eq!(parse::dec_byte("255"), Ok(255));
    assert_eq!(parse::dec_byte("1"), Ok(1));
    assert!(parse::dec_byte("256").is_err());
    assert!(parse::dec_byte("1234").is_err());
}

#[test]
fn xml() {
    assert!(parse::xml("<a></a>").is_ok());
    assert!(parse::xml("<a><b></b><c></c></a>").is_ok());
    assert!(parse::xml("<a><b><c></b></c></a>").is_err());
    assert!(parse::xml("<a><b></c><c></b></a>").is_err());
}
