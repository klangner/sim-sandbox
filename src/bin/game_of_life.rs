use std::{path::Path, fs, io::Write};

use anyhow::Result;
use image::{ImageBuffer, RgbImage};

use sim_sandbox::gol::sim;


struct LocalStorage {
    name: String,
    folder: String,
}

impl LocalStorage {
    // Repo data is store in folder with name created based on param file.
    fn init(param_file_name: &str) -> Result<Self> {
        let name = Path::new(&param_file_name).file_stem().unwrap().to_str().unwrap().to_owned();
        let folder = format!("output/game-of-life/{}", &name);
        fs::create_dir_all(Path::new(&folder))?;

        Ok(Self { name, folder })
        
    }
    
    // Save parameters to the output
    fn write_params(&self, params: &sim::Params) -> Result<()> {

        let out_str = toml::to_string_pretty(params)?;
        let file_name = format!("{}/{}.ini", &self.folder, &self.name);
        let mut output = fs::File::create(&file_name)?;
        write!(&mut output, "{}", &out_str)?;

        Ok(())
    }

    // Save current state as an image
    fn write_image(&self, system: &sim::System) -> Result<()> {
        let mut buffer: Vec<u8> = vec![0u8; 3*512*512]; // Generate the image data
        let path = format!("{}/{}.png", &self.folder, &self.name);
        image::save_buffer(&path, &buffer, 512, 512, image::ColorType::Rgb8)?;

        Ok(())
    }
}

fn main() -> Result<()> {
    let param_file = std::env::args().last().expect("No param file provided");
    let contents = fs::read_to_string(&param_file)?;
    let params: sim::Params = toml::from_str(&contents)?;

    let mut system = sim::System::init(params);
    let storage =  LocalStorage::init(&param_file)?;
    
    storage.write_params(&system.params)?;
    storage.write_image(&system)?;

    for _ts in 0..system.params.num_steps {
        system.next_step();
    }

    println!("Simulation finished!");
    Ok(())
}
