use miette::{Diagnostic, SourceSpan};
use miette::{NamedSource, Result};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("oops!")]
#[diagnostic(
    code(oops::my::bad),
    url(docsrs),
    help("try doing it better next time?")
)]
struct MyBad {
    // The Source that we're gonna be printing snippets out of.
    // This can be a String if you don't have or care about file names.
    #[source_code]
    src: NamedSource<String>,
    // Snippets and highlights can be included in the diagnostic!
    #[label("This bit here")]
    bad_bit: SourceSpan,
}

fn this_fails() -> Result<()> {
    // You can use plain strings as a `Source`, or anything that implements
    // the one-method `Source` trait.
    let src = "source\n  text\n    here".to_string();
    println!("{}", src.len());

    Err(MyBad {
        src: NamedSource::new("bad_file.rs", src),
        bad_bit: (18, 4).into(),
    })?;

    Ok(())
}

fn main() -> Result<()> {
    this_fails()?;
    Ok(())
}
