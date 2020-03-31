use glium::Frame;
use glium::backend::Facade;
use glium::texture::Texture2d;
use crate::glium_drawer::GliumDrawer;
use std::collections::HashMap;
use super::application_store::ApplicationStore;
use smithay::backend::graphics::gl::GLGraphicsBackend;
use linicon::{IconType, IconPath};
use image::ImageFormat;
use std::fs::File;
use std::io::BufReader;
use std::ffi::OsString;

pub struct ApplicationChooser {
  selected_panel_idx: u16,
  application_store: ApplicationStore,
  icon_textures: HashMap<OsString, Texture2d>,
}

impl ApplicationChooser {
  pub fn new()->Self {
    ApplicationChooser {
        selected_panel_idx: 0,
        application_store: ApplicationStore::new(),
        icon_textures: HashMap::new(),
    }
  }

  pub fn init<F: GLGraphicsBackend>(&mut self, drawer: &GliumDrawer<F>) {
    self.load_applications();
    self.load_icons(drawer);
  }

  fn load_applications(&mut self) {
    self.application_store.populate().unwrap();
    self.application_store.lookup_icons();
  }

  fn load_image_icon<F: GLGraphicsBackend>(
      drawer: &GliumDrawer<F>,
      icon_textures: &mut HashMap<OsString, Texture2d>, path: &IconPath, format: ImageFormat) {
    if let Ok(file) = File::open(&path.path) {
    if let Ok(image) = image::load(BufReader::new(file), format) {
        let rgba = image.to_rgba();
        let image_dimensions = rgba.dimensions();
        icon_textures.insert(path.path.clone().into_os_string(), drawer.texture_from_raw(rgba.into_raw(), image_dimensions));
    }
    }
  }

  fn load_icons<F: GLGraphicsBackend>(&mut self, drawer: &GliumDrawer<F>) {
    let applications = &self.application_store.applications;
    let icon_textures = &mut self.icon_textures;
    for app in applications.iter() {
        if let Some(icon) = app.icon_path.as_ref() { 
            match icon.icon_type {
                IconType::PNG => Self::load_image_icon(drawer, icon_textures, icon, ImageFormat::Png),
                ref icon_type => println!("{:?} icon type not supported yet!", icon_type)
            }
        }
    }
  }

  pub fn draw<F: GLGraphicsBackend>(&self, frame: &mut Frame, drawer: &GliumDrawer<F>) {
      //load icons as Texture2D
      let mut i = 0;
      let mut j = 0;
      for app in self.application_store.applications.iter() {
         if let Some(icon) = app.icon_path.as_ref() {
            if let Some(texture) = self.icon_textures.get(&icon.path.clone().into_os_string()) {
                drawer.render_texture(frame,  texture, 0usize,
                    false, (64, 64),
        (j*70, i*70),
         (1280, 800),
        None,
    ) ;
                
                j += 1;
                if j*70 > 1280 {
                 i+=1;
                 j=0;
                }
            }
         }
      }
  }
}
