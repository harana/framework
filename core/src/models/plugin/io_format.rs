use crate::error::Result;

pub trait IOFormat: Send + Sync {
    type Input;
    type Output;

    fn format_id(&self) -> &str;
    fn extensions(&self) -> &[&str];
    fn mime_type(&self) -> &str;
    fn read(&self, input: &Self::Input) -> Result<Self::Output>;
    fn write(&self, output: &Self::Output) -> Result<Self::Input>;
    fn validate(&self, input: &Self::Input) -> Result<()>;

    fn supports_extension(&self, ext: &str) -> bool {
        self.extensions().iter().any(|e| e.eq_ignore_ascii_case(ext))
    }
}
