use codegen_schema::{ExpressionRef, Production};
use semver::Version;

pub trait ProductionChumskyExtensions {
    fn expression_for_version(&self, version: &Version) -> ExpressionRef;
}

impl ProductionChumskyExtensions for Production {
    fn expression_for_version(&self, version: &Version) -> ExpressionRef {
        self.versions
            .iter()
            .filter(|(v, _)| *v <= version)
            .last()
            .map(|(_, e)| e.clone())
            .expect(&format!(
                "Production {} has no content for version {}",
                self.name, version
            ))
    }
}
