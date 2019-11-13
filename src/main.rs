use structopt::StructOpt;
use wiki::args::Args;
use wiki::query_wiki;
use wiki::error::Error;

fn main() -> Result<(), Error> {
    let args = Args::from_args();

    let locale = args.locale.clone();

    let response = query_wiki(args)?;

    for search_result in response.query.search {
        println!("https://{}.wikipedia.org/wiki/{}\t-\t{}", locale, search_result.title.replace(' ', "_"), search_result.snippet);
    }

    Ok(())
}
