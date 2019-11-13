use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "wiki", about = "a tiny wikipedia client")]
pub struct Args {
    /// String to lookup for on wikipedia
    pub search: String,
    #[structopt(short, long, default_value = "en")]
    /// Country code to use when performing wiki search
    pub locale: String,
}
