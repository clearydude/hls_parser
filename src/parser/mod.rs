use nom::bytes::complete::{is_not, tag, take_till, take_until, take_while};

use nom::branch::alt;
use nom::character::complete::{anychar, char, line_ending, none_of, not_line_ending};
use nom::character::streaming::alpha1;
use nom::combinator::{not, rest, value};
use nom::multi::{many0, separated_list1};
use nom::sequence::{delimited, separated_pair};

mod attributes;
