extern crate freedesktop_entry_parser;
extern crate dirs;

use freedesktop_entry_parser::{parse_entry, SectionBytes, AttrBytes};

use std::io;
use std::fs::{self, read};
use std::path::Path;
use std::ffi::OsStr;
use std::process::Command;

const SYSTEM_SEARCH_PATH: &str = "/usr/share/applications";
const USER_SEARCH_PATH: &str = ".local/share/applications";

#[derive(Debug)]
pub struct Application {
  pub name: String,
  pub exec: String,
  pub icon: Option<String>
}

impl Application {
  pub fn new(name: String, exec: String, icon: Option<String>) -> Self {
    Application {name, exec, icon}
  }

  pub fn launch(&self) {
    let mut parts = self.exec.split(" ").into_iter();
    let cmd = parts.next().unwrap();
    let args: Vec<_> = parts.collect();
    Command::new(cmd)
        .args(args)
        .spawn()
        .expect("command failed to start");
    ()
  }
}

#[derive(Debug)]
pub struct ApplicationStore {
  pub applications: Vec<Application>
}

impl ApplicationStore {
  pub fn new() -> Self {
    ApplicationStore {
      applications: vec![]
    }
  }

  pub fn pretty_print(&self) {
    println!("Application store:");
    for app in self.applications.iter() {
      println!("  * {}", app.name);
      println!("    - exec: \"{}\"", app.exec);
      let icon_str =  if app.icon == None {
          "None"
      } else {
          &app.icon.as_ref().unwrap()
      };
      println!("    - icon: {}", icon_str);
    }
  }

  pub fn populate(&mut self) -> io::Result<()> {
    let home = dirs::home_dir();
    if home.is_some() {
        self.populate_from_dir(home.unwrap().join(USER_SEARCH_PATH))?;
    };
    self.populate_from_dir(SYSTEM_SEARCH_PATH)
  }

  fn populate_from_dir<S: AsRef<OsStr>>(&mut self, dir_str: S) -> io::Result<()> {
    let dir = Path::new(&dir_str);
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension() == Some(OsStr::new("desktop")) {
                let data = read(path)?;
                let desktop_entries = parse_entry(&data);
                for desktop_entry in desktop_entries {
                  self.add_entry(desktop_entry.unwrap())
                }
            }
        }
    }
    Ok(())
  }

  fn add_entry(&mut self, section: SectionBytes) {
    if section.title == b"Desktop Entry" {
        let mut app_name = None;
        let mut app_exec = None;
        let mut app_icon = None;

        for AttrBytes {name, value}  in section.attrs {
          if name == b"Name" {
            app_name = Some(value)
          }
          if name == b"Exec" {
            app_exec = Some(value)
          }
          if name == b"Icon" {
            app_icon = Some(value)
          }
        }

        match (app_name, app_exec, app_icon) {
          (Some(name), Some(exec), icon) =>
              self.applications.push(
                  Application::new(
                      std::str::from_utf8(name).unwrap().to_string(),
                      std::str::from_utf8(exec).unwrap().to_string(),
                      icon
                        .map(|path| std::str::from_utf8(path).unwrap().to_string())
                  )
              ),
          _ => (),
        };
    }
  }
}
