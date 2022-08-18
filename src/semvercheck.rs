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
                        println!("'HEAD' is a local branch");
                        if let Some(name) = head.shorthand() {
                            println!("Shorthand name is '{}'", name);
                        }
                    }
                },
                _ => println!("Could not get Reference to HEAD"),
            }

            if repo.find_branch("master", BranchType::Remote).is_ok() {
                println!("Remote 'master' exists");
            } else if repo.find_branch("main", BranchType::Remote).is_ok() {
                println!("Remote 'main' exists");
            } else {
                println!("Could not find remote 'master' or 'main' branch");
            }
        },
        Err(e) => panic!("failed to open: {}", e),
    };

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
