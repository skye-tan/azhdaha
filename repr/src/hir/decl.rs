#![allow(clippy::missing_docs_in_private_items)]

use log::trace;

use crate::hir::*;

#[derive(Debug, Clone)]
pub struct LocalDecl {
    pub storage: Option<Storage>,
    pub ident: Ident,
    pub ty: Ty,
    pub init: Option<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FuncDecl {
    pub storage: Option<Storage>,
    pub ident: Ident,
    pub sig: FuncSig,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FuncSig {
    pub ret_ty: Ty,
    pub params: Vec<ParamDecl>,
    pub variadic_param: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ParamDecl {
    pub storage: Option<Storage>,
    pub ident: Option<Ident>,
    pub ty: Ty,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

impl HirCtx<'_> {
    pub(crate) fn lower_to_local_decl(&mut self, node: Node) -> anyhow::Result<LocalDecl> {
        trace!("[HIR/LocalDecl] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let mut storage = None;

        let mut cursor = node.walk();

        for child in node.children(&mut cursor) {
            if child.kind() == constants::STORAGE_CLASS_SPECIFIER {
                storage = Some(self.lower_to_storage(child.child(0).unwrap())?);
                break;
            }
        }

        let ty = self.lower_to_ty(node)?;

        let mut decl_node = node.child_by_field_name("declarator").unwrap();

        let init = if decl_node.kind() == constants::INIT_DECLARATOR {
            let init = self.lower_to_expr(decl_node.child(decl_node.child_count() - 1).unwrap())?;

            Some(init)
        } else {
            None
        };

        let ident = loop {
            if decl_node.kind() == constants::IDENTIFIER {
                break self.lower_to_ident(decl_node)?;
            } else {
                decl_node = decl_node.child_by_field_name("declarator").unwrap();
            }
        };

        Ok(LocalDecl {
            storage,
            ident,
            ty,
            init,
            span,
        })
    }

    pub(crate) fn lower_to_func_decl(&mut self, node: Node) -> anyhow::Result<FuncDecl> {
        trace!("[HIR/FuncDecl] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let mut storage = None;

        let mut cursor = node.walk();

        for child in node.children(&mut cursor) {
            if child.kind() == constants::STORAGE_CLASS_SPECIFIER {
                storage = Some(self.lower_to_storage(child.child(0).unwrap())?);
                break;
            }
        }

        let ty = self.lower_to_ty(node)?;

        let mut decl_node = node.child_by_field_name("declarator").unwrap();

        while decl_node.kind() != constants::FUNCTION_DECLARATOR {
            decl_node = decl_node.child_by_field_name("declarator").unwrap();
        }

        let ident = self.lower_to_ident(decl_node.child(0).unwrap())?;

        let sig = self.lower_to_func_sig(decl_node, ty)?;

        Ok(FuncDecl {
            storage,
            ident,
            sig,
            span,
        })
    }

    pub(crate) fn lower_to_func_sig(&mut self, node: Node, ret_ty: Ty) -> anyhow::Result<FuncSig> {
        trace!("[HIR/FuncSig] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let mut params = vec![];
        let mut variadic_param = false;

        let mut cursor = node.child_by_field_name("parameters").unwrap().walk();

        cursor.goto_first_child();
        cursor.goto_next_sibling();

        while cursor.node().kind() != ")" {
            if cursor.node().kind() == constants::VARIADIC_PARAMETER {
                variadic_param = true;
                break;
            } else {
                params.push(self.lower_to_param_decl(cursor.node())?);
            }

            cursor.goto_next_sibling();
            cursor.goto_next_sibling();
        }

        Ok(FuncSig {
            ret_ty,
            params,
            variadic_param,
            span,
        })
    }

    pub(crate) fn lower_to_param_decl(&mut self, node: Node) -> anyhow::Result<ParamDecl> {
        trace!("[HIR/ParamDecl] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let mut storage = None;

        let mut cursor = node.walk();

        for child in node.children(&mut cursor) {
            if child.kind() == constants::STORAGE_CLASS_SPECIFIER {
                storage = Some(self.lower_to_storage(child.child(0).unwrap())?);
                break;
            }
        }

        let ty = self.lower_to_ty(node)?;

        let mut ident = None;

        let mut decl_node = node;

        while let Some(node) = decl_node.child_by_field_name("declarator") {
            decl_node = node;

            if decl_node.kind() == constants::IDENTIFIER {
                ident = Some(self.lower_to_ident(decl_node)?);
                break;
            }
        }

        Ok(ParamDecl {
            storage,
            ident,
            ty,
            span,
        })
    }

    pub(crate) fn lower_to_ident(&mut self, node: Node) -> anyhow::Result<Ident> {
        trace!("[HIR/Ident] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        Ok(Ident {
            name: std::str::from_utf8(&self.source_code[node.start_byte()..node.end_byte()])?
                .to_string(),
            span,
        })
    }
}
