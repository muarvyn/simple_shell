pub mod parser {
    use nom::{
        IResult,
        combinator::{map, opt, recognize, value, cut},
        branch::alt,
        bytes::complete::{tag, take_while_m_n, is_a, take_till, escaped},
        character::{
            is_alphabetic,
            is_digit,
            complete::{self, alpha1, alphanumeric1, multispace0, line_ending, newline, one_of, char},},
        sequence::{tuple, pair, delimited, preceded, terminated},
        multi::{many0, many0_count},
        Err,
        error::{Error, context, convert_error, ContextError, ErrorKind, ParseError, VerboseError},
    };


    #[derive(Debug, PartialEq)]
    pub enum Value {
        Str(String),
        Unsigned(u32),
    }

    #[derive(Debug, PartialEq)]
    pub enum Command {
        Empty,
        Assignment(String, Value),
        Quit,
    }

    fn parse_str<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
        escaped(alphanumeric1, '\\', one_of("\"n\\"))(i)
    }

    fn string<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
        i: &'a str,
      ) -> IResult<&'a str, &'a str, E> {
        context(
          "string",
          preceded(char('\"'), cut(terminated(parse_str, char('\"')))),
        )(i)
    }

    fn identifier(input: &str) -> IResult<&str, &str> {
        recognize(
        pair(
            alt((alpha1, tag("_"))),
            many0_count(alt((alphanumeric1, tag("_"))))
        )
        )(input)
    }

    fn assignment(line : &str) -> IResult<&str, (&str, Value)> {
        pair(preceded( multispace0, identifier),
            preceded(
                delimited(multispace0, tag("="), multispace0),
                cut(alt((
                    map(complete::u32, Value::Unsigned),
                    map(string, |s| Value::Str(String::from(s))),
                )))
            ))(line)
    }

    fn empty(line : &str) -> IResult<&str, Command> {
        if line.len() == 0 {
            return Ok((line, Command::Empty))
        }
        Err(Err::Error(Error::from_error_kind(line, ErrorKind::Fail)))
    }

    pub fn command(line : &str) -> IResult<&str, Command> {
        delimited(
            multispace0,
            alt((
                preceded( tag("let"),
                    map(assignment, |(a,b)| Command::Assignment(String::from(a),b))),
                map(tag("quit"), |_| Command::Quit),
                empty,
            )),
            multispace0
        )(line)
    }
} // mod parser


#[cfg(test)]
mod tests {
    use super::parser::*;
    use nom::{
        error::{Error, ErrorKind::Digit, ErrorKind::Char},
        Err,
    };

    #[test]
    fn test_basic() {
        assert_eq!(command("let x = 1"),
                   Ok(("", Command::Assignment(String::from("x"), Value::Unsigned(1)))));
        assert_eq!(command("\tquit \n"),
                   Ok(("", Command::Quit)));
        assert_eq!(command(" let\t_t_x22=0333 "),
                   Ok(("", Command::Assignment(String::from("_t_x22"), Value::Unsigned(333)))));
        let i = "let t=-3";
        assert_eq!(command(i),
                   Err(Err::Failure(Error{input: "-3", code: Char})));
        assert_eq!(command(" \t\n "),
                   Ok(("", Command::Empty)));

    }

    #[test]
    fn test_assignment() {
        assert_eq!(command("let str = \"test1\""),
        Ok(("", Command::Assignment(String::from("str"),
                                    Value::Str(String::from("test1"))))));
    }
}
