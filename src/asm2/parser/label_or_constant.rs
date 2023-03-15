use super::*;


#[derive(Debug)]
pub struct AstLabel
{
    pub decl_span: diagn::Span,
    pub hierarchy_level: usize,
    pub name: String,
    
    pub item_ref: Option<util::ItemRef::<asm2::Symbol>>,
}


#[derive(Debug)]
pub struct AstConstant
{
    pub decl_span: diagn::Span,
    pub hierarchy_level: usize,
    pub name: String,
    pub expr: expr::Expr,
    
    pub item_ref: Option<util::ItemRef::<asm2::Symbol>>,
}


pub fn parse(
    report: &mut diagn::Report,
    walker: &mut syntax::TokenWalker)
    -> Result<AstAny, ()>
{
    let mut decl_span = diagn::Span::new_dummy();
    let mut hierarchy_level = 0;
    
    while let Some(tk_dot) = walker.maybe_expect(syntax::TokenKind::Dot)
    {
        hierarchy_level += 1;
        decl_span = decl_span.join(&tk_dot.span);
    }

    let tk_name = walker.expect(report, syntax::TokenKind::Identifier)?;
    let name = tk_name.excerpt.clone().unwrap();
    decl_span = decl_span.join(&tk_name.span);


    if walker.maybe_expect(syntax::TokenKind::Equal).is_some()
    {
        let expr = expr::parse(report, walker)?;
        walker.expect_linebreak(report)?;
        
        Ok(AstAny::Constant(AstConstant {
            decl_span,
            hierarchy_level,
            name,
            expr,

            item_ref: None,
        }))
    }
    else
    {
        let tk_colon = walker.expect(report, syntax::TokenKind::Colon)?;
        decl_span = decl_span.join(&tk_colon.span);
        
        Ok(AstAny::Label(AstLabel {
            decl_span,
            hierarchy_level,
            name,

            item_ref: None,
        }))
    }
}