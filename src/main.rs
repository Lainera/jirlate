use pulldown_cmark::{Event::*, Options, Parser, Tag};
use std::io::*;

fn main() -> Result<()> {
    let mut args = std::env::args();
    let source = if let Some(filename) = args.nth(1) {
        let mut file = std::fs::File::open(std::path::Path::new(&filename))
            .unwrap_or_else(|_| panic!("Failed to open file at:\n {}", filename));
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        contents
    } else {
        eprintln!("Please provide path to markdown file as first and only argument");
        std::process::exit(0);
    };

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&source, options);
    let mut dest = None;
    let mut list_depth: u8 = 0;
    let jira = parser.map(|event| match event {
        Start(Tag::Heading(level, ..)) => format!("{level}. "),
        Start(Tag::Strikethrough) | End(Tag::Strikethrough) => "-".to_string(),
        Start(Tag::Emphasis) | End(Tag::Emphasis) => "_".to_string(),
        Start(Tag::Strong) | End(Tag::Strong) => "*".to_string(),
        Start(Tag::Link(_, url, _)) => {
            // Store the url to be consumed by subsequent Text event
            dest = Some(url);
            String::new()
        }
        // Closing link shouldn't generate a newline
        End(Tag::Link(..)) => String::new(),
        Start(Tag::List(_)) => {
            list_depth = list_depth.saturating_add(1);
            String::new()
        }

        // If it is a sublist, newline comes from following item
        // If sublist is last item, newline comes from the closing tag of the last list
        End(Tag::List(_)) => {
            list_depth = list_depth.saturating_sub(1);
            if list_depth > 0 {
                String::new()
            } else {
                "\n".to_string()
            }
        }

        Start(Tag::BlockQuote) | End(Tag::BlockQuote) => "{quote}\n".to_string(),
        Start(Tag::Item) => {
            let mut list_indent = "*".repeat(list_depth as usize);
            list_indent.push(' ');
            format!("\n{list_indent}")
        }

        // List items get newlines before text, so there is no need
        // to add one after.
        // Last item of the list gets a newline from the End tag of the
        // List itself
        End(Tag::Item) => String::new(),
        Text(title) if dest.is_some() => {
            format!("[{}|{}]", title, dest.take().unwrap())
        }
        Text(text) => text.into_string(),
        Code(text) => format!("{{{{{}}}}}", text),
        Start(Tag::CodeBlock(_)) | End(Tag::CodeBlock(_)) => "{code}\n".to_string(),
        End(_) | SoftBreak => "\n".to_string(),
        _ => String::new(),
    });

    let mut out = BufWriter::new(stdout());
    for item in jira {
        write!(&mut out, "{}", item)?;
    }

    Ok(())
}
