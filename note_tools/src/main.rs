use clap::{Parser, Subcommand};
use note_lib::{AbstractNote, Interval, Note};

mod explore_root;

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

    #[command(about = "Add intervals together to determine the resulting interval")]
    AddIntervals {
        #[arg(value_name = "INTERVAL")]
        interval1: String,
        #[arg(value_name = "INTERVAL")]
        interval2: String,
    },

    #[command(about = "Subtract intervals to determine the resulting interval")]
    SubIntervals {
        #[arg(value_name = "INTERVAL")]
        interval1: String,
        #[arg(value_name = "INTERVAL")]
        interval2: String,
    },

    /// What keys contain this chord, and at what degree?
    LocateChord {
        #[arg(value_name = "CHORD")]
        chord: String,
        #[arg(value_name = "MODE")]
        mode: Option<String>,
    },

    #[command(about = "Explore what keys contain this root at a given scale degree")]
    ExploreRoot {
        #[arg(
            value_name = "ROOT",
            help = "The abstract root note to explore (no octave specified), e.g. C#"
        )]
        root: String,
    },

    #[command(about = "Find enharmonics for a given note")]
    Enharmonic {
        #[arg(
            value_name = "NOTE",
            help = "The note to find enharmonics for, e.g. C#4"
        )]
        note: String,
        #[arg(
            long,
            short,
            help = "Whether to include double sharps and double flats in the enharmonic results"
        )]
        doubles: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match execute_cli(cli) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    Ok(())
}

fn execute_cli(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
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
        NoteToolsCommand::AddIntervals {
            interval1,
            interval2,
        } => {
            let interval1 = Interval::try_from(interval1.as_str())?;
            let interval2 = Interval::try_from(interval2.as_str())?;

            let result = interval1 + interval2;
            println!("{}", result);
        }
        NoteToolsCommand::SubIntervals {
            interval1,
            interval2,
        } => {
            let interval1 = Interval::try_from(interval1.as_str())?;
            let interval2 = Interval::try_from(interval2.as_str())?;

            let result = interval1 - interval2;
            println!("{}", result);
        }
        NoteToolsCommand::LocateChord { chord, mode } => {
            println!("Locating chord: {}, in mode: {:?}", chord, mode);
        }
        NoteToolsCommand::ExploreRoot { root } => {
            println!("Exploring root: {}", root);
            let root = AbstractNote::try_from(root.as_str())?;
            println!("Parsed root: {}", root);
            explore_root::explore_root(root);
        }
        NoteToolsCommand::Enharmonic { note, doubles } => {
            let note = Note::try_from(note.as_str())?;
            println!("Enharmonics for {}:", note);
            if doubles {
                for enharmonic in note.get_enharmonics_extended() {
                    println!("{}", enharmonic);
                }
            } else {
                for enharmonic in note.get_enharmonics() {
                    println!("{}", enharmonic);
                }
            }
        }
    }
    Ok(())
}
