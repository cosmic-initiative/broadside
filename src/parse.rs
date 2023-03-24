use std::path::PathBuf;
use ariadne::{Label, Report, ReportKind};
use chumsky::error::Simple;
use chumsky::Parser;
use chumsky::prelude::{end, filter, just};
use semver::Version;
use crate::model::{CannonBall, CannonFile};

fn path_segment() -> impl Parser<char, String, Error=Simple<char>> {
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

fn file_path() -> impl Parser<char,PathBuf,Error=Simple<char>> {
    just("/").then(path_segment()).repeated().map( |segments|{
        let mut rtn = String::new();
        for (_,segment) in segments {
            rtn.push_str("/");
            rtn.push_str(segment.as_str() );
        }
        rtn.into()
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

pub fn cannon_file() -> impl Parser<char,CannonFile, Error=Simple<char>> {
   cannonball().then(file_path()).map( |(ball, path)| {
       CannonFile::new( ball, path )
   })
}

pub fn cannonfile_complete() -> impl Parser<char, CannonFile, Error=Simple<char>> {
    cannon_file().then_ignore(end())
}


pub fn report_parse<'a>(errs: Vec<Simple<char>>) -> Report<'a> {
    let mut builder = Report::build(ReportKind::Error, (), 0);
    for err in errs {
        builder = builder.with_label(Label::new( err.span()).with_message(err.to_string()) );
    }
    builder.finish()
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