use crate::ParseResult;

pub fn map<Parser, Mapper, Original, New>(parser: Parser, mapper: Mapper) -> ParseResult<New>
where
    Parser: Fn(&str) -> ParseResult<Original>,
    Mapper: Fn(Original) -> New,
{
    move |input| {
        parser(input).map(|(remaining, original)| (remaining, mapper(original)))
    }
}