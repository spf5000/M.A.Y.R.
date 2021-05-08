use crate::ParseResult;

pub fn pair_matcher<Parser1, Parser2, Result1, Result2>(first_parser: Parser1, second_parser: Parser2)
    -> impl Fn(&str) -> ParseResult<(Result1, Result2)>
where
  Parser1: Fn(&str) -> Result<(&str, Result1), &str>,
  Parser2: Fn(&str) -> Result<(&str, Result2), &str>
{
    move |input| {
        let (remaining_input, first_result) = first_parser(input)?;
        let (remaining_input, second_result) = second_parser(remaining_input).or_else(|_| Err(input))?;
        return Ok((remaining_input, (first_result, second_result)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matchers::literal_matcher::match_literal;
    use rstest::rstest;

    #[rstest(input, expected,
    case("Hello World! It's a beautiful day!", Ok(("", ((), ())))),
    case("Hello World! It's a beautiful day! I think I'll go outside", Ok((" I think I'll go outside", ((), ())))),
    case("It's a beautiful day! Hello World!", Err("It's a beautiful day! Hello World!")),
    case("Hello World! I think I'll go outside!", Err("Hello World! I think I'll go outside!")),
    case("Hello World!", Err("Hello World!")),
    case(" It's a beautiful day!", Err(" It's a beautiful day!"))
    )]
    fn pair_matcher_tests(input: &str, expected: Result<(&str, ((), ())), &str>) {
        let hello_world_matcher = match_literal("Hello World!");
        let beautiful_day_matcher = match_literal(" It's a beautiful day!");
        let pair_matcher = pair_matcher(hello_world_matcher, beautiful_day_matcher);

        assert_eq!(expected, pair_matcher(input));
    }

    // macro_rules! pair_matcher_tests {
    //     ($($name:ident: $value:expr,)*) => {
    //     $(
    //     #[test]
    //     fn $name() {
    //         let (input, expected) = $value;
    //
    //         let hello_world_matcher = match_literal("Hello World!");
    //         let beautiful_day_matcher = match_literal(" It's a beautiful day!");
    //         let pair_matcher = pair_matcher(hello_world_matcher, beautiful_day_matcher);
    //
    //         assert_eq!(expected, pair_matcher(input));
    //     }
    //     )*
    //     }
    // }

    // pair_matcher_tests! {
    //     matches_pair_no_extra_input_test: (),
    //     matches_pair_with_extra_input_test: (),
    //     matches_pair_out_of_order_test: (),
    //     first_matches_second_does_not_test: (),
    //     only_first_matches_no_remaining_input_test: (),
    //     only_second_matches_no_remaining_input_test: (),
    // }
}