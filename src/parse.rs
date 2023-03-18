use chumsky::error::Simple;
use chumsky::Parser;
use chumsky::prelude::{end, filter, just};
use semver::Version;
use crate::model::CannonBall;

fn path_segment() -> impl Parser<char, String, Error=Simple<char>> {
    filter( |c:&char| c.is_alphanumeric()
        || *c == '_'
        || *c == '-'
    ).repeated().map( |path| {
        let mut string = String::new();

        for c in path{
            string.push(c);
        }

        string

    } )
}

fn semver_filter() -> impl Parser<char, String, Error=Simple<char>> {
    filter( |c:&char| c.is_alphanumeric()
        || *c == '_'
        || *c == '-'
        || *c == '.'
    ).repeated().map( |path| {
        let mut string = String::new();

        for c in path{
            string.push(c);
        }

        string

    } )
}

pub fn semver() -> impl Parser<char, Version, Error=Simple<char>> {
   semver_filter().try_map(|version, span| {
       Version::parse( version.as_str() ).map_err(|e| Simple::custom(span, e.to_string() ))
   })
}


pub fn cannonball() -> impl Parser<char, CannonBall, Error=Simple<char>> {
    path_segment().then_ignore(just("/")).then(path_segment()).then_ignore(just("/")).then(semver()).map(|((account,series),version)|{
        CannonBall::new( account, series, version )
    })
}


pub fn cannonball_complete() -> impl Parser<char, CannonBall, Error=Simple<char>> {
    cannonball().then_ignore(end())
}


#[cfg(test)]
pub mod test {
    use crate::parse::{cannonball, semver};
    use chumsky::Parser;
    use semver::Version;

    #[test]
    pub fn test_semver() {
        let version = semver().parse("3.1.0").unwrap();

        assert_eq!(version.major, 3);
        assert_eq!(version.minor, 1);
        assert_eq!(version.patch, 0);
    }

    #[test]
    pub fn test_cannonball() {
        let cannon_ball = cannonball().parse("uberscott/ball/1.3.5").unwrap();
        assert_eq!(cannon_ball.version, Version::parse("1.3.5").unwrap());
        assert_eq!(cannon_ball.account.as_str(), "uberscott");
        assert_eq!(cannon_ball.series.as_str(), "ball");
    }
}