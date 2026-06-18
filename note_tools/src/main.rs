use clap::{Parser, Subcommand};
use note_lib::{Interval, Note};

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

    #[command(about = "Subtract an interval from a given note")]
    SubInterval {
        #[arg(value_name = "NOTE")]
        note: String,
        #[arg(value_name = "INTERVAL")]
        interval: String,
    },

    /// What keys contain this chord, and at what degree?
    LocateChord {
        #[arg(value_name = "CHORD")]
        chord: String,
        #[arg(value_name = "MODE")]
        mode: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        NoteToolsCommand::AddInterval { note, interval } => {
            let note = Note::try_from(note.as_str())?;
            let interval = Interval::try_from(interval.as_str())?;

            let result = note + interval;
            println!("{}", result);
        }
        NoteToolsCommand::SubInterval { note, interval } => {
            let note = Note::try_from(note.as_str())?;
            let interval = Interval::try_from(interval.as_str())?;

            let result = note - interval;
            println!("{}", result);
        }
        NoteToolsCommand::LocateChord { chord, mode } => {
            println!("Locating chord: {}, in mode: {:?}", chord, mode);
        }
    }
    Ok(())
}
