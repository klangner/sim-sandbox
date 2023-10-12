use std::{fs, path::Path};
use std::io::Write;
use anyhow::Result;
use sim_sandbox::evolution::{Params, Universe, Tile};


struct LocalStorage {
    name: String,
    folder: String,
}

impl LocalStorage {
    // Repo data is store in folder with name created based on param file.
    fn init(param_file_name: &str) -> Result<Self> {
        let name = Path::new(&param_file_name).file_stem().unwrap().to_str().unwrap().to_owned();
        let folder = format!("output/evolution/{}", &name);
        fs::create_dir_all(Path::new(&folder))?;

        Ok(Self { name, folder })

    }

    // Save parameters to the output
    fn write_params(&self, params: &Params) -> Result<()> {

        let out_str = toml::to_string_pretty(params)?;
        let file_name = format!("{}/{}.ini", &self.folder, &self.name);
        let mut output = fs::File::create(&file_name)?;
        write!(&mut output, "{}", &out_str)?;

        Ok(())
    }

    // Save current state as an image
    fn write_image(&self, universe: &Universe, step: usize) -> Result<()> {
        let mut buffer: Vec<u8> = vec![255; 3*universe.width()*universe.height()]; // Generate the image data

        for row in 0..universe.height() {
            for col in 0..universe.width() {
                if let Tile::Occupied(a) = universe.cell_at(row, col) {
                    let r =  a.genes as u8;
                    let g =  (a.genes >> 8) as u8;
                    let b =  (a.genes >> 16) as u8;
                    let idx = 3 * (row * universe.width() + col) as usize;
                    buffer[idx] = r;
                    buffer[idx+1] = g;
                    buffer[idx+2] = b;
                }
            }
        }
        let path = format!("{}/{:04}.png", &self.folder, step);
        image::save_buffer(&path, &buffer, universe.width() as u32, universe.height() as u32, 
            image::ColorType::Rgb8)?;

        Ok(())
    }
}
 

fn main() -> Result<()> {
    let param_file = std::env::args().last().expect("No param file provided");
    let contents = fs::read_to_string(&param_file)?;
    let params: Params = toml::from_str(&contents)?;
    let storage = LocalStorage::init(&param_file)?;
    storage.write_params(&params)?;
    let mut universe = Universe::random(params.clone());

    for t in 0..params.steps_per_generation {
        universe.tick();
        storage.write_image(&universe, t+1)?;
    }

    println!("Simulation finished!");

    Ok(())
}