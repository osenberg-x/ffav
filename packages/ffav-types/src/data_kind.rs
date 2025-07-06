#[derive(Debug)]
pub enum DataKind {
	Local(String),
	Network(String),
	Memory(String),
	Unknown,
}