use super::*;

pub struct Build<'ast> {
    classes: &'ast Classes<'ast>,
    function: &'ast Function<'ast>,
}

impl<'ast> Build<'ast> {
    pub fn from(classes: &'ast Classes, function: &'ast Function) -> Self {
        Build { classes, function }
    }
}

impl<'ast> Update for Build<'ast> {
    fn update(&self, cfg: &mut CFG) -> Result<(), String> {
        for statement in self.function.get_statements() {
            ast::Update::update(statement, cfg, self.function, self.classes)?;
        }

        Ok(())
    }
}
