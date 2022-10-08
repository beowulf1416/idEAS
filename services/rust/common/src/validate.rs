pub trait Validate {
    fn validate() -> Result<_, Vec<String>>;
}