use weedle::{
    *,
    types::*,
    interface::*,
};

type WeedleErr<'a> = Err<CompleteStr<'a>, u32>;

#[derive(Debug)]
pub struct Error<'a>(Option<WeedleErr<'a>>);

impl<'a> From<WeedleErr<'a>> for Error<'a> {
    fn from(e: WeedleErr<'a>) -> Self {
        Self(Some(e))
    }
}

fn flatten<'a>(union: &'a MayBeNull<UnionType<'a>>) -> Vec<&'a NonAnyType<'a>> {
    let mut result = vec![];
    for member in &union.type_.body.list {
        match member {
            UnionMemberType::Single(s) => {
                result.push(s);
            },
            UnionMemberType::Union(u) => {
                result.append(&mut flatten(&u));
            },
        }
    }
    result
}

fn contains_sequence(ty: &Type) -> bool {
    fn non_any_contains_sequence(ty: &NonAnyType) -> bool {
        match ty {
            NonAnyType::Sequence(..) => true,
            _ => false,
        }
    }

    match ty {
        Type::Single(SingleType::Any(..)) => {
            false
        },
        Type::Single(SingleType::NonAny(t)) => {
            non_any_contains_sequence(&t)
        },
        Type::Union(u) => {
            flatten(u).into_iter().any(non_any_contains_sequence)
        },
    }
}

pub fn validate<'a>(webidl: &'a str) -> Result<(), Error<'a>> {
    let result = parse(webidl)?;
    println!("{:#?}", result);
    for def in result {
        if let Definition::Interface(iface) = def {
            for member in iface.members.body {
                if let InterfaceMember::Attribute(attr) = member {
                    if contains_sequence(&attr.type_.type_) {
                        return Err(Error(None));
                    }
                }
            }
        }
    }
    Ok(())
}
