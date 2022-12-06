
pub type EntityName = String;
pub type ProjectName = String;

pub enum ProjectKey {
    Name(EntityName, ProjectName),
}