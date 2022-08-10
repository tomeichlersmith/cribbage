// calculate points of all five-card hands possible in the deck

use cribbage::deck;

use clap::Parser;

/// calculate score of all five-card hands in a deck
#[derive(Parser,Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// output file to write data to in CSV format
    #[clap(short, long, value_parser)]
    output_path : String
}

fn main() -> Result<(), csv::Error> {
    let args = Args::parse();

    // this is the computationally intensive part of this program
    // but is also the part that is well tested in the library and
    // so will not be the cause of an error here
    let lut = deck::unique_scores();

    // simply write it to CSV
    let mut wtr = csv::Writer::from_path(args.output_path)?;
    wtr.write_record(&["hand0","hand1","hand2","hand3","cut","score"])?;
    for (hand, score) in lut {
        wtr.write_record(&[
                         hand.hand[0].to_string(),
                         hand.hand[1].to_string(),
                         hand.hand[2].to_string(),
                         hand.hand[3].to_string(),
                         hand.cut.to_string(),
                         score.to_string()
        ])?;
    }

    Ok(())
}
