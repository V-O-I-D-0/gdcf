use create::parse_ty;
use itertools::Itertools;
use proc_macro::{Delimiter, Group, Ident, Spacing, TokenStream, TokenTree};
use std::iter::FromIterator;

pub(super) struct Table {
    model_name: TokenStream,
    table_name: Ident,
    field_mapping: Vec<(Option<Ident>, Ident)>,
}

impl Table {
    pub(super) fn parse(ts: TokenStream) -> Table {
        let mut iter = ts.into_iter();

        let (model_name, next) = parse_ty(&mut iter);

        match next {
            Some(TokenTree::Punct(punct)) =>
                match punct.spacing() {
                    Spacing::Joint if punct.as_char() == '=' => (),
                    Spacing::Joint => panic!("Expected '{}', got '{}'", '=', punct.as_char()),
                    _ => panic!("Expected joint '{}', got alone '{}'", '=', punct.as_char()),
                },
            Some(crap) => panic!("Expected '{}', got {:?}", '=', crap),
            None => panic!("Excepted '{}', got end-of-stream", '='),
        }

        alone_punct!(iter, '>');
        let table_name = ident!(iter);

        let body = any_group!(iter).stream();

        iter = body.into_iter();

        let mut fields = Vec::new();

        loop {
            let model = ident!(iter);

            match iter.next() {
                Some(TokenTree::Punct(ref punct)) if punct.as_char() == '=' => {},
                Some(TokenTree::Punct(ref punct)) if punct.as_char() == ',' => {
                    fields.push((None, model));
                    continue
                },
                Some(crap) => panic!("Expected ',' or '=', got {:?}", crap),
                None => break fields.push((None, model)),
            };

            alone_punct!(iter, '>');
            let table = ident!(iter);

            fields.push((Some(model), table));

            match iter.next() {
                Some(TokenTree::Punct(ref punct)) if punct.as_char() == ',' => continue,
                Some(crap) => panic!("Expected ',' or end-of-stream, got {:?}", crap),
                None => break,
            };
        }

        Table {
            model_name,
            table_name,
            field_mapping: fields,
        }
    }

    pub(super) fn generate(&self) -> TokenStream {
        let mut streams = Vec::<TokenStream>::new();

        streams.push("use core::table::{Field, Table};".parse().unwrap());
        streams.push(format!("pub const table_name: &str = \"{}\";", self.table_name).parse().unwrap());

        let mut field_string = String::new();

        for (_, field) in &self.field_mapping {
            streams.push(
                format!("pub static {}: Field = Field {{ table: table_name, name: \"{}\" }};", field, field)
                    .parse()
                    .unwrap(),
            );

            field_string = format!("{},{}", field_string, field);
        }

        field_string.remove(0);

        streams.push(
            format!(
                "pub static table: Table = Table {{ name: table_name, fields: &[{}] }};",
                field_string
            )
            .parse()
            .unwrap(),
        );

        TokenStream::from_iter(streams)
    }

    pub(super) fn insert(&self, backend: &str) -> TokenStream {
        let field_setter = self
            .field_mapping
            .iter()
            .filter(|(model, _)| model.is_some())
            .map(|(model, field)| format!("{}.set(&self.{})", field, model.as_ref().unwrap()))
            .join(",");

        format!(
            "use core::{{table::{{SetField, Table}}, query::insert::Insertable}};\
             impl Insertable<{}> for {} {{\
             fn table(&self) -> Table {{\
             table\
             }}\
             \
             fn values(&self) -> Vec<SetField<{}>> {{\
             vec![{}] \
             }}\
             }}",
            backend, self.model_name, backend, field_setter
        )
        .parse()
        .unwrap()
    }

    pub(super) fn query(&self, backend: &str) -> TokenStream {
        let field_getter = self.field_mapping
            .iter()
            .filter_map(|(model, _)| model.as_ref())
            .enumerate()
            .map(|(idx, model)|
                format!("{}: row.get(offset + {}).expect(\"In code generated by procedural macro. Did you define your model-to-table mapping correctly?\")?", model, idx))
            .join(",");

        format!(
            "use core::query::select::{{Queryable, Row}};\
             impl Queryable<{}> for {} {{\
             fn from_row(row: &Row<{}>, offset: isize) -> Result<Self, Error<{}>> {{\
             Ok({} {{\
             {}\
             }})\
             }}\
             }}",
            backend,
            self.model_name,
            backend,
            backend,
            self.type_name(),
            field_getter
        )
        .parse()
        .unwrap()
    }

    pub(super) fn gated_impl(&self, feature: &str, module: &str, backend: &str) -> TokenStream {
        stream! {
            format!("#[cfg(feature=\"{0}\")] mod {0}", feature).parse().unwrap(),
            TokenTree::Group(Group::new(Delimiter::Brace, stream! {
                format!("use core::backend::{}::{};", module, backend).parse().unwrap(),
                "use super::*;".parse().unwrap(),
                self.insert(backend),
                self.query(backend)
            })).into()
        }
    }

    pub(super) fn gated_insertable_impl(&self, feature: &str, module: &str, backend: &str) -> TokenStream {
        stream! {
            format!("#[cfg(feature=\"{0}\")] mod {0}", feature).parse().unwrap(),
            TokenTree::Group(Group::new(Delimiter::Brace, stream! {
                format!("use core::backend::{}::{};", module, backend).parse().unwrap(),
                "use super::*;".parse().unwrap(),
                self.insert(backend)
            })).into()
        }
    }

    fn type_name(&self) -> TokenTree {
        self.model_name.clone().into_iter().next().unwrap()
    }
}
