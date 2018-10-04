# rust-dep-check

A smol utility to check for dependencies on a package from crates.io and the feature flags they're using to help manage breaking changes and feature-flag based migration across the crates.io ecosystem.

## Usage

Run with `dep-check CRATE_NAME`, print help with `dep-check -h`.

### Help
```
Crates.io package dependency analyser
Ryan Kurte <ryan@kurte.nz>

USAGE:
    dep-check [FLAGS] [OPTIONS] <dependency>

FLAGS:
    -h, --help       Prints help information
    -r, --reindex    Force re-download of crates.io index
    -V, --version    Prints version information

OPTIONS:
        --index-dir <index-dir>    Directory for crates.io index [default: _index]

ARGS:
    <dependency>    Package to analyse
```

### Example Output
```
> dep-check embedded-hal

Loading crate index

Found 100 crates using 'embedded-hal'
Version requirements:
	^0.1	(0.1.3):	   1 / 100 (1.00 %)
	^0.1.0	(0.1.3):	   6 / 100 (6.00 %)
	^0.1.1	(0.1.3):	   1 / 100 (1.00 %)
	^0.1.2	(0.1.3):	   7 / 100 (7.00 %)
	^0.2	(0.2.1):	  19 / 100 (19.00 %)
	^0.2.0	(0.2.1):	  12 / 100 (12.00 %)
	^0.2.1	(0.2.1):	  45 / 100 (45.00 %)
	~0.2	(0.2.1):	   9 / 100 (9.00 %)
Features:
	unproven:	  35 / 100 (35.00 %)
Resolved versions and features:
	0.1.3:	  15 / 100 (15.00 %)
		unproven:	   3 / 15 (20.00 %)
	0.2.1:	  85 / 100 (85.00 %)
		unproven:	  32 / 85 (37.65 %)
```