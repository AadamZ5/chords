use clap::{Args, Parser, Subcommand, ValueEnum};
use note_lib::Note;

#[derive(Debug, Parser)]
#[command(
    about = "A collection of tools for calculating and manipulating musical notes and chords."
)]
struct Cli {
    #[command(subcommand)]
    command: NoteToolsCommand,
}

#[derive(Debug, Subcommand)]
enum NoteToolsCommand {
    #[command(about = "Add an interval to a given note")]
    AddInterval {
        #[arg(value_name = "NOTE")]
        note: String,
        #[arg(value_name = "INTERVAL")]
        interval: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        NoteToolsCommand::AddInterval { note, interval } => {
            let that_note = Note::try_from(note.as_str());
            match that_note {
                Ok(note) => {
                    println!("Parsed note: {}", note);
                }
                Err(e) => {
                    eprintln!("Error parsing note: {:?}", e);
                }
            }
        }
    }
}
