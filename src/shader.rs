use std::io::Error;
use std::io::Read;
use std::path::PathBuf;
use std::time::SystemTime;
use std::{fs::File, io::ErrorKind};

use anyhow::{Context, Result};

pub trait Shader {
    fn get_text(&self) -> &str;
    fn set_text(&mut self, text: String);

    fn update(&mut self);
    fn check_changes(&mut self) -> Result<bool>;

    fn compile(&self) -> Result<()>;
}

pub struct TextShader {
    text: String,
}

impl TextShader {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

impl Shader for TextShader {
    fn get_text(&self) -> &str {
        &self.text
    }

    fn set_text(&mut self, text: String) {
        self.text = text;
    }

    fn check_changes(&mut self) -> Result<bool> {
        Ok(false)
    }
    fn update(&mut self) {}

    fn compile(&self) -> Result<()> {
        unimplemented!();
    }
}

pub struct FileShader {
    text_shader: TextShader,
    file_path: PathBuf,
    timestamp: SystemTime,
    live_reload: bool,
}

impl FileShader {
    pub fn new(file_path: PathBuf, live_reload: bool) -> Result<Self> {
        let mut file = File::open(&file_path).context(format!("Failed to open shader file"))?;
        let timestamp = file
            .metadata()
            .context("Failed to retrieve shader file metadata")?
            .modified()
            .context("Failed to retrieve shader file last modification time")?;

        let mut text = String::new();
        file.read_to_string(&mut text).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!(
                    "Failed to retrieve shader content: {:?} ({:?})",
                    &file_path, &e
                ),
            )
        })?;

        let text_shader = TextShader::new(text);

        Ok(Self {
            text_shader,
            file_path,
            timestamp,
            live_reload,
        })
    }
}

impl Shader for FileShader {
    fn get_text(&self) -> &str {
        self.text_shader.get_text()
    }

    fn set_text(&mut self, text: String) {
        self.text_shader.set_text(text);
    }

    fn check_changes(&mut self) -> Result<bool> {
        if !self.live_reload {
            return Ok(false);
        }

        if let Ok(metadata) = std::fs::metadata(&self.file_path) {
            let current_timestamp = metadata
                .modified()
                .context("Failed to retrieve shader file last modification time")?;
            if current_timestamp != self.timestamp {
                let mut file = File::open(&self.file_path).context("Failed to open shader file")?;
                let mut text = String::new();
                file.read_to_string(&mut text)
                    .context("Failed to retrieve shader content")?;

                self.text_shader.set_text(text);
                self.timestamp = current_timestamp;
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn update(&mut self) {}

    fn compile(&self) -> Result<()> {
        self.text_shader.compile()
    }
}

pub struct ShaderComposer {
    components: Vec<Box<dyn Shader>>,
    text: String,
}

impl Default for ShaderComposer {
    fn default() -> Self {
        Self {
            components: Vec::new(),
            text: String::new(),
        }
    }
}

impl ShaderComposer {
    pub fn insert(&mut self, index: usize, shader: Box<dyn Shader>) {
        let index = index.min(self.components.len());
        self.components.insert(index, shader);
    }

    pub fn push(&mut self, shader: Box<dyn Shader>) {
        self.components.push(shader);
        self.text.clear();
        for shader in self.components.iter() {
            self.text.push_str(shader.get_text());
            self.text.push('\n');
        }
    }
}

impl Shader for ShaderComposer {
    fn get_text(&self) -> &str {
        &self.text
    }

    fn set_text(&mut self, _text: String) {
        unimplemented!();
    }

    fn check_changes(&mut self) -> Result<bool> {
        let mut changed = false;

        for shader in self.components.iter_mut() {
            match shader.check_changes() {
                Ok(true) => {
                    if !changed {
                        changed = true;
                    }
                }
                Ok(false) => (),
                error => return error,
            }
        }

        if changed {
            self.text.clear();
            for shader in self.components.iter_mut() {
                self.text.push_str(shader.get_text());
                self.text.push('\n');
            }
        }

        Ok(changed)
    }

    fn update(&mut self) {
        for component in self.components.iter_mut() {
            component.update();
        }
    }

    fn compile(&self) -> Result<()> {
        unimplemented!();
    }
}
