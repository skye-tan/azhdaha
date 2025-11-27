#![allow(clippy::missing_docs_in_private_items)]

use azhdaha_errors::{Context, bail};
use itertools::Either;
use log::trace;

use crate::hir::*;

#[derive(Debug)]
pub struct VarDecl {
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
    pub(crate) fn lower_to_var_decl_list(
        &mut self,
        node: Node,
    ) -> azhdaha_errors::Result<Either<Vec<VarDecl>, Ty>> {
        trace!("[HIR/LocalDeclList] Lowering '{}'", node.kind());

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

        let mut decls = vec![];

        let mut cursor = node.walk();

        for decl_node in node.children_by_field_name("declarator", &mut cursor) {
            let mut ty = self.lower_to_ty(node, Some(decl_node))?;

            let ident = {
                let mut decl_node = decl_node;
                loop {
                    match decl_node.kind() {
                        constants::IDENTIFIER
                        | constants::TYPE_IDENTIFIER
                        | constants::FIELD_IDENTIFIER => {
                            break self.lower_to_ident(decl_node)?;
                        }
                        _ => {
                            decl_node = decl_node
                                .child_by_field_name("declarator")
                                .context(span, "Cannot find declarator.")?;
                        }
                    }
                }
            };

            let init = if decl_node.kind() == constants::INIT_DECLARATOR {
                let mut init = self
                    .lower_to_expr_with_expected_type(
                        decl_node.child(decl_node.child_count() - 1).unwrap(),
                        ty.clone(),
                    )
                    .with_context(span, || {
                        format!("Fail to lower initializer of {}", ident.name)
                    })?;

                if ty.kind.is_array() {
                    let mut temp = &init;
                    while let ExprKind::Cast(inner) = &temp.kind {
                        temp = inner;
                    }
                    if let ExprKind::Lit(lit) = &temp.kind
                        && let LitKind::Str(string) = &lit.kind
                    {
                        let list = initializer_list_from_string(string, ty.clone(), span);
                        init = Expr {
                            kind: ExprKind::InitializerList(Box::new(
                                self.lower_to_initializer_tree(&ty.kind, list, span),
                            )),
                            ty: Ty {
                                kind: TyKind::InitializerList,
                                is_linear: false,
                                quals: vec![],
                                span,
                            },
                            span,
                        };
                        init = Expr {
                            kind: ExprKind::Cast(Box::new(init)),
                            ty: ty.clone(),
                            span,
                        };
                    }
                }

                if let ExprKind::Cast(in_cast) = &mut init.kind
                    && let ExprKind::InitializerList(items) = &mut in_cast.kind
                    && let TyKind::Array { kind: _, size } = &mut ty.kind
                    && size.is_none()
                {
                    *size = Some(items.children().len());
                    init.ty = ty.clone();
                }

                Some(init)
            } else {
                None
            };

            decls.push(VarDecl {
                storage: storage.clone(),
                ident,
                ty,
                init,
                span,
            });
        }

        if decls.is_empty() {
            return Ok(Either::Right(self.lower_to_ty(node, None)?));
        }

        Ok(Either::Left(decls))
    }

    pub(crate) fn lower_to_var_decl(&mut self, node: Node) -> azhdaha_errors::Result<VarDecl> {
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

        let mut decl_node = node.child_by_field_name("declarator").unwrap();

        let ty = self.lower_to_ty(node, Some(decl_node))?;

        let init = if decl_node.kind() == constants::INIT_DECLARATOR {
            let init = self.lower_to_expr(decl_node.child(decl_node.child_count() - 1).unwrap())?;

            Some(init)
        } else {
            None
        };

        let ident = loop {
            match decl_node.kind() {
                constants::IDENTIFIER | constants::TYPE_IDENTIFIER => {
                    break self.lower_to_ident(decl_node)?;
                }
                _ => {
                    decl_node = decl_node
                        .child_by_field_name("declarator")
                        .context(span, "Cannot find declarator.")?;
                }
            }
        };

        Ok(VarDecl {
            storage,
            ident,
            ty,
            init,
            span,
        })
    }

    pub(crate) fn lower_to_func_decl(&mut self, node: Node) -> azhdaha_errors::Result<FuncDecl> {
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

        let mut decl_node = node.child_by_field_name("declarator").unwrap();

        let ty = self.lower_to_ty(node, Some(decl_node))?;

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

    pub(crate) fn lower_to_func_sig(
        &mut self,
        node: Node,
        ret_ty: Ty,
    ) -> azhdaha_errors::Result<FuncSig> {
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

        if params.is_empty() {
            variadic_param = true;
        }

        if params.len() == 1 && params[0].ty.kind.is_void() {
            params = vec![];
        }

        Ok(FuncSig {
            ret_ty,
            params,
            variadic_param,
            span,
        })
    }

    pub(crate) fn lower_to_param_decl(&mut self, node: Node) -> azhdaha_errors::Result<ParamDecl> {
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

        let mut decl_node = node.child_by_field_name("declarator");

        let mut ty = self.lower_to_ty(node, decl_node)?;

        let mut ident = None;

        while let Some(node) = decl_node {
            if node.kind() == constants::IDENTIFIER {
                ident = Some(self.lower_to_ident(node)?);
                break;
            } else {
                decl_node = node.child_by_field_name("declarator");
            }
        }

        // Function arguments are always decayed to pointer in C.
        if let TyKind::Array { kind, size: _ } = ty.kind {
            ty.kind = TyKind::Ptr {
                kind,
                quals: vec![],
            };
        }

        Ok(ParamDecl {
            storage,
            ident,
            ty,
            span,
        })
    }

    pub(crate) fn lower_to_ident(&self, node: Node) -> azhdaha_errors::Result<Ident> {
        trace!("[HIR/Ident] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let Ok(name) = std::str::from_utf8(&self.source_code[node.start_byte()..node.end_byte()])
        else {
            bail!(span, "Invalid utf8 in identifier name");
        };

        Ok(Ident {
            name: name.to_string(),
            span,
        })
    }
}
