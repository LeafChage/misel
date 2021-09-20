extern crate clap;
extern crate md;

mod html;
use clap::{App, Arg, ArgMatches};
use std::fs::{read_to_string, File};
use std::io::{self, Error, ErrorKind, Write};

fn main() {
    if let Err(n) = command() {
        println!("{}", n);
    }
}

fn command() -> io::Result<()> {
    let matches = App::new("misel")
        .version("1.0")
        .author("LeafChage <https://github.com/LeafChage>")
        .about("Markdown to HTML.")
        .arg(
            Arg::with_name("input")
                .value_name("FILE")
                .help("input markdown file")
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("output html file")
                .default_value("./output.html")
                .required(true),
        )
        .arg(
            Arg::with_name("style")
                .long("style")
                .value_name("FILE(css)")
                .help("style file")
                .default_value("./resource/style.css"),
        )
        .arg(
            Arg::with_name("title")
                .long("title")
                .value_name("string")
                .help("html title"),
        )
        .get_matches();

    let input = fetch_string(&matches, "input")?;
    let output = fetch_string(&matches, "output")?;
    let style = fetch_string(&matches, "style")?;
    let title = if let Ok(title) = fetch_string(&matches, "title") {
        title
    } else {
        ""
    };

    let src = read_from(input)?;
    let style_src = read_from(style)?;
    if let Ok(tokens) = md::parser(&src) {
        write_to(output, html::generate(tokens, title, &style_src))?;
    } else {
        return Err(Error::from(ErrorKind::Other));
    }
    Ok(())
}

fn fetch_string<'a>(matches: &'a ArgMatches, name: &str) -> io::Result<&'a str> {
    if let Some(path) = matches.value_of(name) {
        Ok(path)
    } else {
        Err(Error::from(ErrorKind::InvalidInput))
    }
}

fn write_to(path: &str, data: String) -> io::Result<()> {
    println!("write to {}", path);
    let mut file = File::create(path)?;
    file.write_all(&data.as_bytes())?;
    file.flush()?;
    Ok(())
}

fn read_from(path: &str) -> io::Result<String> {
    println!("read from {}", path);
    read_to_string(path)
}
