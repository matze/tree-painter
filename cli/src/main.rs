use anyhow::{anyhow, Result};
use clap::Parser;
use std::fs::read_to_string;
use std::path::PathBuf;
use tree_painter::{Lang, Renderer, Theme};

#[derive(Parser)]
struct Args {
    /// Path to a helix theme
    #[clap(long)]
    theme: PathBuf,

    /// Path to a source file
    #[clap(long)]
    source: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let lang = Lang::from(&args.source)
        .ok_or_else(|| anyhow!("Cannot determine language from file extension"))?;

    let source = read_to_string(args.source)?;
    let theme = Theme::from_helix(&read_to_string(args.theme)?)?;
    let mut renderer = Renderer::new(theme);

    print!(
        r#"
    <!DOCTYPE html>
      <title>tree-painter highlighting</title>
      <style>
        {}
      </style>
    </head>
    <body class="tsc-bg">
      <pre>
        <table>
          <tbody>"#,
        renderer.css()
    );

    for line in renderer.render(&lang, source.as_bytes())? {
        print!(r#"<tr><td class="tsc-line">{line}</td></tr>"#);
    }

    print!(
        r#"
          </tbody>
        </table>
      </pre>
    </body>"#
    );

    Ok(())
}
