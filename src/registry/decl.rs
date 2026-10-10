pub struct CopDecl {
    pub name: &'static str,
    pub category: Category,
    pub default_severity: Severity,
    pub since: &'static str,
    pub params: &'static [ParamSpec],
    pub build: fn(&CopParams, &Globals) -> Result<Box<dyn Cop>, ConfigError>,
}