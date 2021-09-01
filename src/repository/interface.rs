use crate::model::commit::Commit;
use crate::model::commits::Commits;
use crate::model::delta::Delta;
use crate::repository::errors::Error;

pub(crate) trait Repository {
    fn commits_in_current_branch(&self) -> Result<Commits, Error>;
    fn compare_with_parent(&self, _: &Commit) -> Result<Delta, Error>;
}
