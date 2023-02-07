use std::collections::HashSet;

use chrono;
use crt_sh_data::{self, Datasource};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    identity: String,

    /// Return only active certificates
    #[structopt(short, long)]
    active: bool,

    /// The certificate is not valid before this date: <2020-01-01>
    #[structopt(long)]
    not_before: Option<chrono::NaiveDate>,

    /// The certificate is not valid after this date: <2020-01-01>
    #[structopt(long)]
    not_after: Option<chrono::NaiveDate>,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let crt_sh_datasource = crt_sh_data::CrtShDatasource::new();
    let mut certificates = crt_sh_datasource
        .search_identity(&opt.identity)
        .await
        .unwrap();

    certificates = if opt.active {
        let now = chrono::offset::Local::now().naive_local();
        certificates
            .into_iter()
            .filter(|certificate| certificate.not_before <= now && certificate.not_after >= now)
            .collect()
    } else {
        certificates
    };

    certificates = if let Some(date) = opt.not_before {
        certificates
            .into_iter()
            .filter(|certificate| certificate.not_before.date() >= date)
            .collect()
    } else {
        certificates
    };

    certificates = if let Some(date) = opt.not_after {
        certificates
            .into_iter()
            .filter(|certificate| certificate.not_after.date() <= date)
            .collect()
    } else {
        certificates
    };

    let subdomains: HashSet<&str> = certificates
        .iter()
        .map(|certificate| {
            certificate
                .common_name
                .split('\n')
                .collect::<HashSet<&str>>()
        })
        .flatten()
        .collect();

    subdomains
        .iter()
        .for_each(|subdomain| println!("{}", subdomain));
}
