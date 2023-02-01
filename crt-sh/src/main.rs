use crt_sh_data::{self, Datasource};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    identity: String,
}

fn main() {
    let opt = Opt::from_args();
    let crt_sh_datasource = crt_sh_data::CrtShDatasource::new();
    let certificates = crt_sh_datasource.search_identity(&opt.identity).unwrap();
    dbg!(certificates);
}
