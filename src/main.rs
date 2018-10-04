use std::collections::HashMap;

extern crate crates_index;
use crates_index::{Index, Version, Dependency};

extern crate clap;
use clap::{Arg, App};

extern crate semver;
use semver::{VersionReq};
use semver::Version as VersionNo;

fn main() {
    let matches = App::new("Crates.io dependency analyser")
		.arg(Arg::with_name("dependency")
			.short("d")
			.long("depencency")
			.help("Dependency to filter on")
			.default_value("embedded-hal")
			.takes_value(true))
		.arg(Arg::with_name("reindex")
			.short("r")
			.long("reindex")
			.help("Force re-download of crates.io index"))
		.arg(Arg::with_name("index-dir")
			.long("index-dir")
			.takes_value(true)
			.help("Directory with crates.io index")
			.default_value("_index"))
		.get_matches();

    let dependency = matches.value_of("dependency").unwrap();
	let reindex = matches.is_present("reindex");
	let index_dir = matches.value_of("index-dir").unwrap();

    println!("Loading crate index");
    let index = Index::new(index_dir);
    if !index.exists() || reindex {
		index.retrieve().expect("Could not retrieve crates.io index");
    }

	// Find crate for checking and load available versions
	let check_crate = index.crates().find(|c| c.name() == dependency)
		.expect("Dependency not found");
	let mut check_versions: Vec<VersionNo> = check_crate.versions().iter().map(|v| VersionNo::parse(v.version()).unwrap() ).collect();
	check_versions.sort();
	check_versions.reverse();

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

	let total_deps = c.len();

    // Generate maps of version requirements and resolutions
    let mut requirement_map: HashMap<VersionReq, u32> = HashMap::new();
	let mut resolved_map: HashMap<VersionNo, u32> = HashMap::new();
    let _: Vec<()> = deps.iter().map(|d| {
		let requirement = VersionReq::parse(&d.requirement().to_string()).unwrap();
    	*requirement_map.entry(requirement.clone()).or_insert(0) += 1;

		let resolved = check_versions.iter().find(|v| requirement.matches(v) ).unwrap();
		*resolved_map.entry(resolved.clone()).or_insert(0) += 1;
    }).collect();

	let mut requirement_list: Vec<(VersionReq, u32)> = requirement_map.iter().map(|(r, n)| (r.clone(), n.clone())).collect();
	requirement_list.sort_by(|a, b| a.0.cmp(&b.0) );

	let mut resolved_list: Vec<(VersionNo, u32)> = resolved_map.iter().map(|(f, n)| (f.clone(), n.clone())).collect();
	resolved_list.sort_by(|a, b| a.0.cmp(&b.0) );

	// Generate map of feature flags
	let mut feature_map: HashMap<String, u32> = HashMap::new();
    let _: Vec<()> = deps.iter().map(|d| {
		for flag in d.features() {
			*feature_map.entry(flag.clone()).or_insert(0) += 1;
		}
    }).collect();

	let mut feature_list: Vec<(String, u32)> = feature_map.iter().map(|(f, n)| (f.clone(), n.clone())).collect();
	feature_list.sort_by(|a, b| a.0.cmp(&b.0) );

	// Show outputs
	println!("");

	println!("Found {} crates using '{}'", total_deps, dependency);

	println!("Version requirements:");
	for (r, n) in requirement_list {
		let resolved = check_versions.iter().find(|v| r.matches(v) ).unwrap();
		println!("\t{}\t({}):\t{:4} / {} ({:.2} %)", r, resolved, n, total_deps, 
				n as f64 / total_deps as f64 * 100.0);
	}

	println!("Resolved version:");
	for (r, n) in resolved_list {
		println!("\t{}:\t{:4} / {} ({:.2} %)", r, n, total_deps, 
				n as f64 / total_deps as f64 * 100.0);
	}

	println!("Features:");
	for (f, n) in feature_list {
		println!("\t{}:\t{:4} / {} ({:.2} %)", f, n, total_deps, 
				n as f64 / total_deps as f64 * 100.0);
	}

	println!("");
}
