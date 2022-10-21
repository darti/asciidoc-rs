use std::{fs, path::Path};

use ctor::ctor;
use indoc::indoc;

use log::info;
use pandoc::{InputFormat, InputKind, OutputFormat, OutputKind, PandocOption};
use pathdiff::diff_paths;
use pretty_env_logger::env_logger::{Builder, Env};

use walkdir::WalkDir;

#[ctor]
fn init_logger() {
    Builder::from_env(Env::new().default_filter_or("info"))
        .is_test(true)
        .init();
}

#[ctor]
fn generate_tests() {
    let root = Path::new("tests").join("samples");
    let output_dir = Path::new("tests").canonicalize().unwrap().join("inputs");

    for entry in WalkDir::new(root.clone())
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let mut output_adoc = output_dir
            .clone()
            .join(diff_paths(entry.path(), root.clone()).unwrap());

        output_adoc.set_extension("adoc");

        fs::create_dir_all(output_adoc.parent().unwrap()).unwrap();

        let mut output_pandoc = output_adoc.clone();
        output_pandoc.set_extension("json");

        info!(
            "Generate test for {} to {} and {}",
            entry.file_name().to_string_lossy(),
            output_adoc.to_string_lossy(),
            output_pandoc.to_string_lossy()
        );

        let input = entry.into_path();

        {
            let mut pandoc = pandoc::new();

            pandoc
                .set_input(InputKind::Files(vec![input.clone()]))
                .set_input_format(InputFormat::MarkdownStrict, vec![])
                .add_option(PandocOption::Standalone)
                .add_option(PandocOption::ShiftHeadingLevelBy(-1))
                .set_output(OutputKind::File(output_adoc))
                .set_output_format(OutputFormat::Asciidoc, vec![]);

            pandoc.execute().unwrap();
        }

        {
            let mut pandoc = pandoc::new();

            pandoc
                .set_input(InputKind::Files(vec![input]))
                .set_input_format(InputFormat::MarkdownStrict, vec![])
                .add_option(PandocOption::Standalone)
                .add_option(PandocOption::ShiftHeadingLevelBy(-1))
                .set_output(OutputKind::File(output_pandoc))
                .set_output_format(OutputFormat::Json, vec![]);

            pandoc.execute().unwrap();
        }
    }
}

#[test]
fn hello_world() {
    let input_ref = indoc! {"
    = Hello, AsciiDoc!

    This is an interactive editor.
    Use it to try https://asciidoc.org[AsciiDoc].

    == Section Title

    * A list item
    * Another list item

    [,ruby]
    ----
    puts 'Hello, World!'
    ----
    "};

    let input = indoc! {"
    # Hello, Asciidoc!

    This is an interactive editor.
    Use it to try [AsciiDoc](https://asciidoc.org).

    ## Section Title

    * A list item
    * Another list item

    ```ruby
    puts 'Hello, World!'
    ```
    "};

    let output = "";

    let mut pandoc = pandoc::new();

    pandoc
        .set_input(InputKind::Pipe(input.to_owned()))
        .set_input_format(InputFormat::MarkdownStrict, vec![])
        .add_option(PandocOption::Standalone)
        .add_option(PandocOption::ShiftHeadingLevelBy(-1))
        .set_output(OutputKind::Pipe)
        .set_output_format(OutputFormat::Asciidoc, vec![]);

    match pandoc.execute().unwrap() {
        pandoc::PandocOutput::ToBuffer(s) => assert_eq!(output, s),
        _ => panic!("unhandled output"),
    }
}
