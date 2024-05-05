
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Model{
    #[default]
    FlatHat,
    Pyramid
}

impl Model{
    pub const ALL: [Model;2]=[
        Model::FlatHat,
        Model::Pyramid,
    ];
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Model::FlatHat => "Flat hat",
                Model::Pyramid => "Pyramid"})
            }
        }