use structopt::StructOpt;
use wiki::args::Args;
use wiki::error::Error;
use wiki::query_wiki;

fn main() -> Result<(), Error> {
    let args = Args::from_args();

    let locale = args.locale.clone();

    let response = query_wiki(args)?;

    for search_result in response.query.search {
        println!(
            "https://{}.wikipedia.org/wiki/{}\t-\t{}",
            locale,
            search_result.title.replace(' ', "_"),
            search_result.snippet
        );
    }

    Ok(())
}
