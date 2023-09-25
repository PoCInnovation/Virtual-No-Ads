use std::process;
use std::collections::HashSet;
use std::env;

pub mod block_ads;

fn main()
{
    let args: Vec<String> = env::args().collect();
    let mut blacklist_set: HashSet<String> = HashSet::new();

    if args.len() < 3 {
        eprintln!("USAGE: ./virtual-no-ads <interface> <domain_list>...");
        process::exit(84)
    }

    match block_ads::parse_adsfile(&args, &mut blacklist_set) {
        Ok(_) => {},
        Err(_) => process::exit(84),
    };

    block_ads::catch_packets(&args[1], blacklist_set);
}
