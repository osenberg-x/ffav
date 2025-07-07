#[derive(Debug, Clone)]
pub enum DataKind {
	Local(String),
	Network(String),
	Memory(String),
	Unknown,
}