// calculate points of all five-card hands possible in the deck

use cribbage::deck::{full_deck, part_deck};
use cribbage::hand::Hand;
use itertools::Itertools;

use clap::Parser;

/// calculate score of all five-card hands in a deck
#[derive(Parser,Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// maximum number of hands
    #[clap(short, long, value_parser)]
    max_hands : Option<u64>,

    /// output file to write data to in CSV format
    #[clap(short, long, value_parser)]
    output_path : String
}

fn main() {
    let args = Args::parse();

    let mut wtr = csv::Writer::from_path(args.output_path).expect("Writable file for output data.");
    wtr.write_record(&["hand0","hand1","hand2","hand3","cut","score"]).expect("able to write to csv");

    // maximum if arg not given is 52 * (51 choose 4)
    let tot_hands : u64 = if let Some(max_hands) = args.max_hands { max_hands } else { 12994800 };
    let progbar = indicatif::ProgressBar::new(tot_hands);
    for hand in full_deck().iter().cloned().combinations(4).map(|cards| Hand { hand : cards }) {
        for cut in part_deck(&hand.hand) {
            wtr.write_record(&[
                             hand.hand[0].to_string(),
                             hand.hand[1].to_string(),
                             hand.hand[2].to_string(),
                             hand.hand[3].to_string(),
                             cut.to_string(),
                             hand.score(&cut).to_string()
            ]).expect("able to write to csv");
            progbar.inc(1);
            if let Some(max_hands) = args.max_hands {
                if progbar.position() >= max_hands {
                    progbar.finish();
                    return;
                }
            }
        }
    }
    progbar.finish();
}
