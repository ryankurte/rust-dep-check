use std::collections::HashMap;

extern crate crates_index;
use crates_index::{Index, Crate, Version, Dependency};

extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Crates.io dependency analyser")
		.arg(Arg::with_name("dependency")
			.short("d")
			.long("depencency")
			.takes_value(true)
			.help("Dependency to filter on")
			.default_value("embedded-hal"))
		.get_matches();

    let dependency = matches.value_of("dependency").unwrap();
    println!("Checking for dependency: '{}'", dependency);


    println!("Fetching crate index");
    let index = Index::new("_index");
    if !index.exists() {
	index.retrieve().expect("Could not retrieve crates.io index");
    }

    // Find crates using the provided dependency
    let mut deps: Vec<Dependency> = Vec::new();
    let c: Vec<Version> = index.crates()
		.map(|c| c.latest_version().clone() )
		.filter(|ref c| {
			match c.dependencies().iter().find(|d| d.name() == dependency) {
				Some(d) => {
					deps.push(d.clone());
					true
				},
				None => false,
			}
		})
		.collect();

    println!("Found {} crates using '{}'", c.len(), dependency);

    // Generate map of dependency use
    let mut dep_map: HashMap<String, u32> = HashMap::new();
    let _: Vec<()> = deps.iter().map(|d| {
	let version = d.requirement().to_string();
    	*dep_map.entry(version).or_insert(0) += 1;
    }).collect();
    
    println!("Dependencies: {:?}", dep_map);

}
