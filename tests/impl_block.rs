//! The impl-block and function item kinds archive and content-address like every
//! other encoded form item: a portable-archive round-trip is identity, and a structural edit
//! to a body expression moves the content identity.

mod support;

use content_identity::PortableArchive;
use core_logos::{
    Block, Call, Callee, EncodedItem, Expression, Function, ImplBlock, ImplItem, MethodCall,
    Parameter, ReferenceExpression, ReferenceMutability, TupleFieldAccess, TypeReference,
    Visibility,
};
use name_table::NameTable;

/// Build the `Topic` inherent impl block as a stringless encoded form value:
///
/// ```ignore
/// impl Topic {
///     pub fn new(payload: impl Into<String>) -> Self { Self(payload.into()) }
///     pub fn payload(&self) -> &String { &self.0 }
///     pub fn into_payload(self) -> String { self.0 }
/// }
/// ```
fn topic_impl(names: &mut NameTable) -> EncodedItem {
    let payload = support::identifier(names, "payload");
    let self_path = support::path(names, &["Self"]);
    let into = support::identifier(names, "into");

    let new = Function {
        attributes: Vec::new(),
        visibility: Visibility::Public,
        name: support::identifier(names, "new"),
        generics: core_logos::Generics::none(),
        receiver: None,
        parameters: vec![Parameter {
            name: payload,
            type_reference: TypeReference::ImplTrait(core_logos::ImplTraitType {
                bounds: vec![TypeReference::Application(core_logos::TypeApplication {
                    head: support::path(names, &["Into"]),
                    arguments: vec![TypeReference::Path(support::path(names, &["String"]))],
                })],
            }),
        }],
        return_type: Some(TypeReference::Path(self_path.clone())),
        body: Block {
            statements: Vec::new(),
            // Self(payload.into())
            tail_expression: Expression::Call(Call {
                callee: Callee::Path(self_path),
                type_arguments: Vec::new(),
                arguments: vec![Expression::MethodCall(MethodCall {
                    receiver: Box::new(Expression::Path(core_logos::PathNode {
                        segments: vec![payload],
                    })),
                    method: into,
                    type_arguments: Vec::new(),
                    arguments: Vec::new(),
                })],
            }),
        },
    };

    let payload_getter = Function {
        attributes: Vec::new(),
        visibility: Visibility::Public,
        name: support::identifier(names, "payload"),
        generics: core_logos::Generics::none(),
        receiver: Some(core_logos::Receiver::Reference),
        parameters: Vec::new(),
        return_type: Some(TypeReference::Reference(core_logos::ReferenceType {
            lifetime: None,
            mutability: ReferenceMutability::Shared,
            referent: Box::new(TypeReference::Path(support::path(names, &["String"]))),
        })),
        body: Block {
            statements: Vec::new(),
            // &self.0
            tail_expression: Expression::Reference(ReferenceExpression {
                referent: Box::new(Expression::Field(TupleFieldAccess {
                    base: Box::new(Expression::Receiver),
                    index: 0,
                })),
            }),
        },
    };

    let into_payload = Function {
        attributes: Vec::new(),
        visibility: Visibility::Public,
        name: support::identifier(names, "into_payload"),
        generics: core_logos::Generics::none(),
        receiver: Some(core_logos::Receiver::Value),
        parameters: Vec::new(),
        return_type: Some(TypeReference::Path(support::path(names, &["String"]))),
        body: Block {
            statements: Vec::new(),
            // self.0
            tail_expression: Expression::Field(TupleFieldAccess {
                base: Box::new(Expression::Receiver),
                index: 0,
            }),
        },
    };

    EncodedItem::ImplBlock(ImplBlock {
        attributes: vec![core_logos::Attribute::ToolPath(support::path(
            names,
            &["rustfmt", "skip"],
        ))],
        generics: core_logos::Generics::none(),
        implemented_trait: None,
        self_type: TypeReference::Path(support::path(names, &["Topic"])),
        items: vec![
            ImplItem::Method(new),
            ImplItem::Method(payload_getter),
            ImplItem::Method(into_payload),
        ],
    })
}

#[test]
fn an_impl_block_round_trips_through_portable_archive() {
    let mut names = NameTable::new(name_table::IdentifierNamespace::Logos);
    let item = topic_impl(&mut names);

    let bytes = item.to_archive_bytes().expect("serialize");
    let restored = EncodedItem::from_archive_bytes(&bytes).expect("deserialize");

    assert_eq!(item, restored);
}

#[test]
fn an_impl_block_has_no_declared_name_but_carries_its_attributes() {
    let mut names = NameTable::new(name_table::IdentifierNamespace::Logos);
    let item = topic_impl(&mut names);

    assert_eq!(item.name(), None, "an impl block declares no name");
    assert_eq!(item.attributes().len(), 1, "the rustfmt::skip preamble");
}

#[test]
fn editing_a_body_expression_moves_the_impl_block_identity() {
    let mut names = NameTable::new(name_table::IdentifierNamespace::Logos);
    let item = topic_impl(&mut names);
    let before = item.content_identity().expect("hash");

    // Change `into_payload`'s body from `self.0` to `self.1` — a structural edit.
    let EncodedItem::ImplBlock(mut impl_block) = item else {
        panic!("impl block");
    };
    let ImplItem::Method(into_payload) = &mut impl_block.items[2] else {
        panic!("the third member is the into_payload method");
    };
    into_payload.body.tail_expression = Expression::Field(TupleFieldAccess {
        base: Box::new(Expression::Receiver),
        index: 1,
    });
    let after = EncodedItem::ImplBlock(impl_block)
        .content_identity()
        .expect("hash");

    assert_ne!(
        before.bytes(),
        after.bytes(),
        "a body edit moves the content identity",
    );
}

/// A free function is a `EncodedItem::Function`, not only an impl member.
#[test]
fn a_free_function_is_a_named_content_addressable_item() {
    let mut names = NameTable::new(name_table::IdentifierNamespace::Logos);
    let value = support::identifier(&mut names, "value");
    let name = support::identifier(&mut names, "identity");
    let string = support::path(&mut names, &["String"]);
    let function = Function {
        attributes: Vec::new(),
        visibility: Visibility::Public,
        name,
        generics: core_logos::Generics::none(),
        receiver: None,
        parameters: vec![Parameter {
            name: value,
            type_reference: TypeReference::Path(string.clone()),
        }],
        return_type: Some(TypeReference::Path(string)),
        body: Block {
            statements: Vec::new(),
            // value
            tail_expression: Expression::Path(core_logos::PathNode {
                segments: vec![value],
            }),
        },
    };
    let item = EncodedItem::Function(function);

    assert!(item.name().is_some(), "a function has a declared name");
    let bytes = item.to_archive_bytes().expect("serialize");
    let restored = EncodedItem::from_archive_bytes(&bytes).expect("deserialize");
    assert_eq!(item, restored);
}
