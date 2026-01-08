use crate::{models::project::Project, repository::repository::Repository};

pub struct ProjectService {
    repository: Repository<Project>,
}

impl ProjectService {
    pub fn new(repository: Repository<Project>) -> Self {
        ProjectService { repository }
    }

    pub fn get_project(&self, project_id: &str) -> Option<Project> {
        self.repository.get(project_id)
    }

    pub fn create_project(&self, project_id: String, project: Project) {
        self.repository.set(project_id, project);
    }
}