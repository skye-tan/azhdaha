#![allow(clippy::missing_docs_in_private_items)]

use std::{collections::HashMap, mem};

use anyhow::{Context, bail};
use itertools::Either;
use la_arena::Idx;
use log::trace;

use crate::hir::{
    resolver::{CompoundTypeData, FieldsData},
    *,
};

use super::{
    constants,
    resolver::{Resolver, Symbol, SymbolKind},
};

#[derive(Debug)]
pub struct Item {
    pub kind: ItemKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum ItemKind {
    Func(Box<FuncDef>),
    Decl(Vec<Symbol>),
    TyDef(Symbol),
    TaggedTypeSpecifier(Idx<resolver::CompoundTypeData>),
    Empty,
}

#[derive(Debug)]
pub struct FuncDef {
    pub label_resolver: Resolver<()>,

    pub arguments_symbols: HashMap<String, Symbol>,
    pub symbol: Symbol,
    pub body: Stmt,
    pub span: Span,
}

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

impl HirCtx<'_> {
    pub(crate) fn lower_to_item(&mut self, node: Node) -> anyhow::Result<Item> {
        trace!("[HIR/Item] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let kind = self.lower_to_item_kind(node)?;

        Ok(Item { kind, span })
    }

    pub(crate) fn lower_to_item_kind(&mut self, node: Node) -> anyhow::Result<ItemKind> {
        trace!("[HIR/ItemKind] Lowering '{}'", node.kind());

        Ok(match node.kind() {
            constants::FUNCTION_DEFINITION => {
                ItemKind::Func(Box::new(self.lower_to_func_def(node)?))
            }
            constants::DECLARATION => {
                let Either::Left(var_decl_list) = self.lower_to_var_decl_list(node)? else {
                    bail!("Invalid empty item declarations.");
                };

                let mut symbols = vec![];

                for var_decl in var_decl_list {
                    let symbol = self
                        .symbol_resolver
                        .insert_symbol(var_decl.ident.name.clone(), SymbolKind::Var(var_decl));

                    symbols.push(symbol);
                }

                ItemKind::Decl(symbols)
            }
            constants::TYPE_DEFINITION => {
                let var_decl = self.lower_to_var_decl(node)?;

                let symbol = self
                    .symbol_resolver
                    .insert_symbol(var_decl.ident.name, SymbolKind::TyDef(var_decl.ty));

                ItemKind::TyDef(symbol)
            }
            constants::STRUCT_SPECIFIER
            | constants::UNION_SPECIFIER
            | constants::ENUM_SPECIFIER => {
                let idx = self.lower_struct_or_union_or_enum(node)?;
                ItemKind::TaggedTypeSpecifier(idx)
            }
            constants::SEMICOLON => ItemKind::Empty,
            kind => {
                bail!("Cannot lower '{kind}' to 'ItemKind'.");
            }
        })
    }

    pub(crate) fn lower_fields_in_specifier(&mut self, body: Node<'_>) -> FieldsData {
        let mut result = FieldsData {
            by_index: vec![],
            by_name: HashMap::new(),
        };
        for node in body.children(&mut body.walk()) {
            if node.kind() == "{" || node.kind() == "}" {
                continue;
            }
            match self.lower_to_var_decl_list(node).unwrap() {
                Either::Left(fields) => {
                    for field in fields {
                        let new_index = result.by_index.len();
                        result.by_index.push(field.ty);
                        result.by_name.insert(field.ident.name, vec![new_index]);
                    }
                }
                Either::Right(unnamed) => {
                    let new_index = result.by_index.len();
                    result.by_index.push(unnamed.clone());
                    let inner_fields = match &unnamed.kind {
                        TyKind::Struct(idx) => {
                            let data = self.type_tag_resolver.get_data_by_res(idx);
                            let CompoundTypeData::Struct { fields } = data else {
                                panic!("Invalid struct {data:?}");
                            };
                            fields
                        }
                        TyKind::Union(idx) => {
                            let data = self.type_tag_resolver.get_data_by_res(idx);
                            let CompoundTypeData::Union { fields } = data else {
                                panic!("Invalid union {data:?}");
                            };
                            fields
                        }
                        _ => panic!(
                            "Type error: primitive unnamed field with type {} is invalid.",
                            unnamed,
                        ),
                    };
                    for inner_field in &inner_fields.by_name {
                        let mut indexes = vec![new_index];
                        indexes.extend_from_slice(inner_field.1);
                        result.by_name.insert(inner_field.0.clone(), indexes);
                    }
                }
            }
        }
        result
    }

    pub(crate) fn lower_to_func_def(&mut self, node: Node) -> anyhow::Result<FuncDef> {
        trace!("[HIR/FuncDef] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let saved_symbol_resolver = self.symbol_resolver.open_new_scope();

        let func_decl = self.lower_to_func_decl(node)?;

        assert!(self.return_ty.is_none());
        self.return_ty = Some(func_decl.sig.ret_ty.clone());

        _ = self.symbol_resolver.insert_symbol(
            func_decl.ident.name.clone(),
            SymbolKind::Func(func_decl.clone()),
        );

        let mut arguments_symbols = HashMap::new();

        for param in &func_decl.sig.params {
            if let Some(ident) = &param.ident {
                arguments_symbols.insert(
                    ident.name.clone(),
                    self.symbol_resolver
                        .insert_symbol(ident.name.clone(), SymbolKind::Param(param.clone())),
                );
            }
        }

        let body = self.lower_to_stmt(node.child(node.child_count() - 1).unwrap());

        self.symbol_resolver
            .restore_prev_scope(saved_symbol_resolver);
        let symbol = self.symbol_resolver.insert_symbol(
            func_decl.ident.name.clone(),
            SymbolKind::Func(func_decl.clone()),
        );

        let label_resolver = mem::take(&mut self.label_resolver);
        self.return_ty = None;

        // Restore resolvers and bail later to not break subsequent functions in case of failure.
        let body =
            body.with_context(|| format!("Fail to lower function {}", func_decl.ident.name))?;

        Ok(FuncDef {
            label_resolver,
            arguments_symbols,
            symbol,
            body,
            span,
        })
    }

    pub(crate) fn lower_to_block(&mut self, node: Node) -> anyhow::Result<Block> {
        trace!("[HIR/Block] Lowering '{}'", node.kind());

        let span = Span {
            lo: node.start_byte(),
            hi: node.end_byte(),
        };

        let saved_symbol_resolver = self.symbol_resolver.open_new_scope();

        let mut stmts = vec![];

        let mut cursor = node.walk();
        cursor.goto_first_child();
        cursor.goto_next_sibling();

        while cursor.node().kind() != "}" {
            let stmt = self.lower_to_stmt(cursor.node())?;

            stmts.push(stmt);

            cursor.goto_next_sibling();
        }

        self.symbol_resolver
            .restore_prev_scope(saved_symbol_resolver);

        Ok(Block { stmts, span })
    }
}
