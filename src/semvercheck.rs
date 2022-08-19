use git2::BranchType;
use semver::{BuildMetadata, Prerelease, Version, VersionReq};

fn main() {
    use git2::Repository;

    match Repository::open(std::env::current_dir().unwrap()) {
        Ok(repo) => {
            println!("Opened {:?}", repo.path());
            match repo.head() {
                Ok(head) => {
                    if head.is_branch() {
                        print!("'HEAD' refers to a local branch");
                        if let Some(name) = head.shorthand() {
                            println!(" named '{}' ('{}')", name, head.name().unwrap_or(""));
                        } else {
                            println!(" name unknown");
                        }
                    } else {
                        println!("'HEAD' is not a local branch");
                    }
                },
                _ => println!("Could not get Reference to HEAD"),
            }

            match repo.find_branch("origin/master", BranchType::Remote)
                .or(repo.find_branch("origin/main", BranchType::Remote)) {
                Ok(origin) => println!("Found '{}'", origin.name().unwrap().unwrap()),
                _ => println!("Could not find 'origin/master' or 'origin/main'"),
            }
        },
        Err(e) => panic!("failed to open: {}", e),
    };


    // See if it's a workspace project and if so repeat for all sub projects

    // get the version of the local branch from Cargo.toml

    // get the version of the remote branch from Cargo.toml

    // Determine what type of change in the version numbers has been made
    // if in verbose mode, print both out and the type of change made

    // For each code change, check against the rules and classify the change as being of type:
    // requiring major change
    // requiring minor change
    // requiring micro change
    // not requiring a change
    // If the change type is > than the version number change made, print it out with an explanation

    let req = VersionReq::parse(">=1.2.3, <1.8.0").unwrap();

    // Check whether this requirement matches version 1.2.3-alpha.1 (no)
    let version = Version {
        major: 1,
        minor: 2,
        patch: 3,
        pre: Prerelease::new("alpha.1").unwrap(),
        build: BuildMetadata::EMPTY,
    };
    assert!(!req.matches(&version));

    // Check whether it matches 1.3.0 (yes it does)
    let version = Version::parse("1.3.0").unwrap();
    assert!(req.matches(&version));
}
