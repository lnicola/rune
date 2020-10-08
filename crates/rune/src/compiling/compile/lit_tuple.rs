use crate::compiling::compile::prelude::*;

/// Compile a literal tuple.
impl Compile2 for ast::LitTuple {
    fn compile2(&self, c: &mut Compiler<'_>, needs: Needs) -> CompileResult<()> {
        let span = self.span();
        log::trace!("LitTuple => {:?}", c.source.source(span));

        for (expr, _) in &self.items {
            expr.compile2(c, Needs::Value)?;
            c.scopes.decl_anon(expr.span())?;
        }

        c.asm.push(
            Inst::Tuple {
                count: self.items.len(),
            },
            span,
        );

        c.scopes.undecl_anon(span, self.items.len())?;

        if !needs.value() {
            c.warnings.not_used(c.source_id, span, c.context());
            c.asm.push(Inst::Pop, span);
            return Ok(());
        }

        Ok(())
    }
}
