use nom::bytes::complete::{is_not, tag, take_till, take_until, take_while};

use nom::branch::alt;
use nom::character::complete::{anychar, char, line_ending, none_of, not_line_ending};
use nom::character::streaming::alpha1;
use nom::combinator::{not, rest, value};
use nom::multi::{many0, separated_list1};
use nom::sequence::{delimited, separated_pair};

mod attributes;

//fn parse_attributes(attrs_str: &str) -> nom::IResult<&str, Vec<(&str, &str)>> {
//    separated_list1(char(','), parse_attribute)(attrs_str)
//}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn parses_attribute_list_into_key_value_map() {
//        let attr_str = r#"BANDWIDTH=2312764,AVERAGE-BANDWIDTH=1919803,CODECS="ec-3,hvc1.2.4.L63.90",RESOLUTION=640x360,FRAME-RATE=23.97,VIDEO-RANGE=PQ,AUDIO="atmos",CLOSED-CAPTIONS=NONE"#;
//        let parsed = parse_attributes(attr_str);
//
//        let expected = Ok(("", vec![("BANDWIDTH", "2312764"), ("AVERAGE-BANDWIDTH", "1919803"), ("CODECS", "\"ec-3, hvc1.2.4.L63.90\""), ("RESOLUTION", "640x360"), ("FRAME-RATE", "23.97"), ("VIDEO-RANGE", "PQ"), ("AUDIO", "\"atmos\""), ("CLOSED-CAPTIONS","NONE")]));
//
//        assert_eq!(parsed, expected)
//    }
//}
