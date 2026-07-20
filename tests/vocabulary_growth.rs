//! The layout-3 vocabulary growth — const items, inline modules, heterogeneous impl
//! members (methods, associated types, associated consts), integer and array
//! literals, slice and lifetime types — archives and content-addresses like every
//! other encoded form value: a portable-archive round-trip is identity, and a structural edit
//! to a literal value moves the content identity.

mod support;

use content_identity::PortableArchive;
use core_logos::{
    ArrayExpression, AssociatedType, Attribute, Const, EncodedItem, Expression, IntegerLiteral,
    IntegerRepresentation, Module, ReferenceExpression, ReferenceMutability, ReferenceType,
    SliceType, TypeReference, Visibility,
};
use name_table::NameTable;

/// The `short_header` const module: `pub mod short_header { pub const INPUT_RECORD:
/// u64 = 0x0000000000000000; }` as a stringless encoded form value.
fn short_header_module(names: &mut NameTable) -> EncodedItem {
    let input_record = EncodedItem::Const(Const {
        visibility: Visibility::Public,
        attributes: Vec::new(),
        name: support::identifier(names, "INPUT_RECORD"),
        type_reference: TypeReference::Path(support::path(names, &["u64"])),
        value: Expression::IntegerLiteral(IntegerLiteral {
            value: 0,
            representation: IntegerRepresentation::Hexadecimal { minimum_digits: 16 },
        }),
    });
    EncodedItem::Module(Module {
        visibility: Visibility::Public,
        attributes: vec![Attribute::ToolPath(support::path(
            names,
            &["rustfmt", "skip"],
        ))],
        name: support::identifier(names, "short_header"),
        items: vec![input_record],
    })
}

/// The `HEADS` associated const `const HEADS: &'static [&'static str] = &["Record"];`
/// as a stringless encoded form value, exercising slice/lifetime types and an array literal.
fn heads_const(names: &mut NameTable) -> Const {
    let static_lifetime = support::identifier(names, "static");
    let str_reference = TypeReference::Reference(ReferenceType {
        lifetime: Some(static_lifetime),
        mutability: ReferenceMutability::Shared,
        referent: Box::new(TypeReference::Path(support::path(names, &["str"]))),
    });
    let slice_reference = TypeReference::Reference(ReferenceType {
        lifetime: Some(static_lifetime),
        mutability: ReferenceMutability::Shared,
        referent: Box::new(TypeReference::Slice(SliceType {
            element: Box::new(str_reference),
        })),
    });
    Const {
        visibility: Visibility::Private,
        attributes: Vec::new(),
        name: support::identifier(names, "HEADS"),
        type_reference: slice_reference,
        value: Expression::Reference(ReferenceExpression {
            referent: Box::new(Expression::Array(ArrayExpression {
                elements: vec![Expression::StringLiteral("Record".to_string())],
            })),
        }),
    }
}

#[test]
fn a_const_module_round_trips_through_portable_archive() {
    let mut names = NameTable::new(name_table::IdentifierNamespace::Logos);
    let item = short_header_module(&mut names);

    let bytes = item.to_archive_bytes().expect("serialize");
    let restored = EncodedItem::from_archive_bytes(&bytes).expect("deserialize");

    assert_eq!(item, restored);
    assert!(item.name().is_some(), "a module declares a name");
}

#[test]
fn an_associated_const_round_trips_through_portable_archive() {
    let mut names = NameTable::new(name_table::IdentifierNamespace::Logos);
    let item = EncodedItem::Const(heads_const(&mut names));

    let bytes = item.to_archive_bytes().expect("serialize");
    let restored = EncodedItem::from_archive_bytes(&bytes).expect("deserialize");

    assert_eq!(item, restored);
}

#[test]
fn editing_a_literal_value_moves_the_const_identity() {
    let mut names = NameTable::new(name_table::IdentifierNamespace::Logos);
    let type_reference = TypeReference::Path(support::path(&mut names, &["u64"]));
    let name = support::identifier(&mut names, "HEADER");
    let base = Const {
        visibility: Visibility::Public,
        attributes: Vec::new(),
        name,
        type_reference: type_reference.clone(),
        value: Expression::IntegerLiteral(IntegerLiteral {
            value: 0x0001_0000_0000_0000,
            representation: IntegerRepresentation::Hexadecimal { minimum_digits: 16 },
        }),
    };
    let before = EncodedItem::Const(base.clone())
        .content_identity()
        .expect("hash");

    // Same name and type, a different literal value — a structural edit.
    let edited = Const {
        value: Expression::IntegerLiteral(IntegerLiteral {
            value: 0x0100_0000_0000_0000,
            representation: IntegerRepresentation::Hexadecimal { minimum_digits: 16 },
        }),
        ..base
    };
    let after = EncodedItem::Const(edited).content_identity().expect("hash");

    assert_ne!(
        before.bytes(),
        after.bytes(),
        "a literal-value edit moves the content identity",
    );
}

/// The representation descriptor is part of the value: the same numeric value under
/// decimal and hexadecimal forms are distinct encoded form values with distinct identities.
#[test]
fn the_integer_representation_is_part_of_the_identity() {
    let mut names = NameTable::new(name_table::IdentifierNamespace::Logos);
    let type_reference = TypeReference::Path(support::path(&mut names, &["u64"]));
    let name = support::identifier(&mut names, "HEADER");
    let make = |representation| {
        EncodedItem::Const(Const {
            visibility: Visibility::Public,
            attributes: Vec::new(),
            name,
            type_reference: type_reference.clone(),
            value: Expression::IntegerLiteral(IntegerLiteral {
                value: 8,
                representation,
            }),
        })
        .content_identity()
        .expect("hash")
    };

    let decimal = make(IntegerRepresentation::Decimal);
    let hexadecimal = make(IntegerRepresentation::Hexadecimal { minimum_digits: 16 });
    assert_ne!(
        decimal.bytes(),
        hexadecimal.bytes(),
        "the surface representation is hashed data, not a projection-time choice",
    );
}

/// An impl block carries a heterogeneous ordered member set: an associated type, an
/// associated const, and a method preserve their source order through the archive.
#[test]
fn an_impl_block_carries_heterogeneous_members_in_order() {
    use core_logos::{Block, Function, Generics, ImplBlock, ImplItem, PathNode, Receiver};

    let mut names = NameTable::new(name_table::IdentifierNamespace::Logos);
    let associated_type = ImplItem::AssociatedType(AssociatedType {
        name: support::identifier(&mut names, "Err"),
        value: TypeReference::Path(support::path(&mut names, &["NotaDecodeError"])),
    });
    let associated_const = ImplItem::AssociatedConst(heads_const(&mut names));
    let method = ImplItem::Method(Function {
        attributes: Vec::new(),
        visibility: Visibility::Public,
        name: support::identifier(&mut names, "route"),
        generics: Generics::none(),
        receiver: Some(Receiver::Reference),
        parameters: Vec::new(),
        return_type: None,
        body: Block {
            statements: Vec::new(),
            tail_expression: Expression::Path(PathNode {
                segments: vec![support::identifier(&mut names, "route")],
            }),
        },
    });
    let item = EncodedItem::ImplBlock(ImplBlock {
        attributes: Vec::new(),
        generics: Generics::none(),
        implemented_trait: None,
        self_type: TypeReference::Path(support::path(&mut names, &["Input"])),
        items: vec![associated_type, associated_const, method],
    });

    let bytes = item.to_archive_bytes().expect("serialize");
    let restored = EncodedItem::from_archive_bytes(&bytes).expect("deserialize");
    assert_eq!(item, restored);

    let EncodedItem::ImplBlock(restored_block) = restored else {
        panic!("impl block");
    };
    assert!(matches!(
        restored_block.items[0],
        ImplItem::AssociatedType(_)
    ));
    assert!(matches!(
        restored_block.items[1],
        ImplItem::AssociatedConst(_)
    ));
    assert!(matches!(restored_block.items[2], ImplItem::Method(_)));
}
