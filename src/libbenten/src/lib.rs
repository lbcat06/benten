mod methods;

use std::path::PathBuf;
use methods::State;
use thiserror::Error;

pub struct BentenEngine {
    state: State,
    cfg: BentenConfig,
}

impl BentenEngine {
    pub fn new(mut cfg: BentenConfig) -> Self {
        //rid id of non visible characters such as "\n"
        cfg.id.retain(|c| !c.is_whitespace());
        let state = State::new(&cfg.id, &cfg.dir).unwrap();
            // .map_err(|_| panic!("layout `{}` not found", &cfg.id)).unwrap();

        BentenEngine { state, cfg }
    }

    pub fn on_key_press(&mut self, key_code: u16) -> BentenResponse {
        self.state.methods.get_mut(&self.state.current_method)
            .unwrap().on_key_press(key_code)
    }

    pub fn on_key_release(&mut self, key_code: u16) -> BentenResponse {
        self.state.methods.get_mut(&self.state.current_method)
            .unwrap().on_key_release(key_code)
    }

    pub fn set_layout(&mut self, name: &str) {
        self.state = State::new(name, &self.cfg.dir).unwrap();
    }

    pub fn reset(&mut self) {
        self.state.methods.get_mut(&self.state.current_method)
            .unwrap().reset()    }
}

#[derive(Debug, PartialEq)]
pub enum BentenResponse {
    Commit(String),
    Suggest(String),
    Undefined, //KeyCode is not defined
    Empty, //KeyCode found but didnt have anything to return, intentional (like special keys eg. Han key)
}

#[derive(Error, Debug)]
pub enum BentenError {
    #[error("`{0}`")]
    IoError(#[from] std::io::Error),
    #[error("`{0}`")]
    CsvParseError(#[from] csv::Error),
    #[error("`parsing error {0}`")]
    ZmeraldError(#[from] zmerald::de::Error),
    #[error("kb parse error")]
    KbParseError,
}

pub struct BentenConfig {
    pub id: String,
    pub dir: PathBuf
}

impl Default for BentenConfig {
    fn default() -> Self {
        BentenConfig {
            dir: xdg::BaseDirectories::with_prefix("benten").unwrap().get_config_home(),
            id: "layout id was not defined".to_string()
        }
    }
}