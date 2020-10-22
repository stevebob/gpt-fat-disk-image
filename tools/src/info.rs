use mini_fat::FatInfo;
use mini_gpt::GptInfo;
use std::fmt;

struct Args {
    image_filename: String,
    debug: bool,
}

impl Args {
    fn parse() -> Self {
        (meap::let_map! {
            let {
                image_filename = opt_req("PATH", 'i').name("image").desc("path to disk image");
                debug = flag('d').name("debug").desc("print debugging info");
            } in {
                Self {
                    image_filename,
                    debug,
                }
            }
        })
        .with_help_default()
        .parse_env_or_exit()
    }
}

#[derive(Debug)]
struct DisplayInfo {
    gpt_info: GptInfo,
    fat_info: FatInfo,
}

impl fmt::Display for DisplayInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use mini_fat::FatType;
        write!(f, "FAT Type: ")?;
        match self.fat_info.fat_type() {
            FatType::Fat12 => writeln!(f, "FAT12")?,
            FatType::Fat16 => writeln!(f, "FAT16")?,
            FatType::Fat32 => writeln!(f, "FAT32")?,
        }
        writeln!(f, "Num Clusters: {}", self.fat_info.num_clusters())?;
        Ok(())
    }
}

fn main() {
    use std::fs::File;
    let Args {
        image_filename,
        debug,
    } = Args::parse();
    let mut image_file = File::open(image_filename).expect("unable to open file");
    let gpt_info = mini_gpt::gpt_info(&mut image_file).unwrap();
    let fat_info = mini_fat::fat_info(
        &mut image_file,
        gpt_info.first_partition_byte_range().unwrap(),
    )
    .unwrap();
    let display_info = DisplayInfo { gpt_info, fat_info };
    if debug {
        println!("{:#?}", display_info);
    } else {
        println!("{}", display_info);
    }
}