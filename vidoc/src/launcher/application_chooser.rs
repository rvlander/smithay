use glium::Frame;
use glium::texture::Texture2d;
use crate::glium_drawer::GliumDrawer;
use std::collections::HashMap;
use super::application_store::ApplicationStore;
use smithay::backend::graphics::gl::GLGraphicsBackend;

pub struct ApplicationChooser {
  selected_panel_idx: u16,
  application_store: ApplicationStore,
  icon_textures: HashMap<String, Texture2d>,
}

impl ApplicationChooser {
  pub fn new()->Self {
    ApplicationChooser {
        selected_panel_idx: 0,
        application_store: ApplicationStore::new(),
        icon_textures: HashMap::new(),
    }
  }

  pub fn init(&mut self) {
    self.load_applications();
    self.load_icons();
  }

  fn load_applications(&mut self) {
    self.application_store.populate().unwrap();
    self.application_store.lookup_icons();
  }

  fn load_icons(&mut self) {
  }

  pub fn draw<F: GLGraphicsBackend>(&self, frame: &Frame, glium_drawer: &GliumDrawer<F>) {
      //load icons as Texture2D

  }
}
