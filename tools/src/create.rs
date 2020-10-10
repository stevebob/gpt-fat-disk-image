use std::fs::File;
use std::io;
use std::process;

struct Args {
    path_pairs: Vec<mini_fat::PathPair>,
    output: Box<dyn io::Write>,
}

impl Args {
    fn parse() -> Self {
        (meap::let_map! {
            let {
                local_filesystem_paths = opt_multi::<String, _>("PATH", 'l')
                    .name("local")
                    .desc("paths to local files to include in image (corresponds to -d)");
                disk_image_paths = opt_multi("PATH", 'd')
                    .name("disk")
                    .desc("paths in disk image where files will be stored (corresponds to -l)");
                output = opt_opt::<String, _>("PATH", 'o').name("output").desc("output file path (omit for stdout)");
            } in {{
                if local_filesystem_paths.len() != disk_image_paths.len() {
                    eprintln!("Error: -l and -d must be passed the same number of times.");
                    process::exit(1);
                }
                let path_pairs = local_filesystem_paths
                    .into_iter()
                    .zip(disk_image_paths.into_iter())
                    .map(|(in_local_filesystem, in_disk_image)| mini_fat::PathPair {
                        in_local_filesystem: File::open(in_local_filesystem).unwrap(),
                        in_disk_image,
                    })
                    .collect();
                Self {
                    path_pairs,
                    output: if let Some(path) = output {
                        Box::new(File::create(path).unwrap())
                    } else {
                        Box::new(io::stdout())
                    },
                }
            }}
        })
        .with_help_default()
        .parse_env_or_exit()
    }
}

fn main() {
    let Args {
        path_pairs,
        mut output,
    } = Args::parse();
    let partition_size = mini_fat::partition_size(&path_pairs).unwrap();
    mini_gpt::write_header(&mut output, partition_size).unwrap();
}
