use pest::{error::Error as PestError, iterators::Pair, Parser as _};
use pest_derive::Parser as PestParser;
use serde::{
    ser::{Error, SerializeMap, SerializeSeq},
    Serialize,
};

pub mod value;

#[derive(PestParser)]
#[grammar = "grammar.pest"]
pub struct Parser;

pub struct LuaPestPair<'a>(Pair<'a, Rule>);

#[cfg(debug_assertions)]
fn print_pair(pair: &Pair<Rule>, depth: usize) {
    match pair.as_rule() {
        Rule::string => println!("{}{}", "\t".repeat(depth), pair.as_str()),
        Rule::COMMENT => println!(
            "{}comment: {}",
            "\t".repeat(depth),
            pair.as_str().replace('\n', ";")
        ),
        _ => println!("{}{:?}", "\t".repeat(depth), pair.as_rule()),
    }

    for pair in pair.clone().into_inner() {
        print_pair(&pair, depth + 1);
    }
}

impl<'config> LuaPestPair<'config> {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(src: &'config str) -> Result<Self, Box<PestError<Rule>>> {
        let result = Parser::parse(Rule::chunk, src);

        match result {
            Ok(mut pairs) => Ok(Self(pairs.next().unwrap())),
            Err(err) => Err(err.into()),
        }
    }

    pub fn from_pair(pair: Pair<'config, Rule>) -> Self {
        Self(pair)
    }

    #[cfg(debug_assertions)]
    pub fn print_pretty(&self) {
        print_pair(&self.0, 0)
    }
}

impl<'config> Serialize for LuaPestPair<'config> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let pair = &self.0;

        match pair.as_rule() {
            Rule::array => {
                let pair = self.0.clone();
                let inner = pair.into_inner();
                let len = Some(inner.len());

                let mut seq = serializer.serialize_seq(len)?;

                for field in inner {
                    let mut field_inner = field.into_inner();

                    let value = field_inner.next().unwrap();

                    seq.serialize_element(&LuaPestPair::from_pair(value))?;
                }

                seq.end()
            }
            Rule::dict => {
                let pair = self.0.clone();
                let inner = pair.into_inner();
                let len = Some(inner.len());

                let mut map = serializer.serialize_map(len)?;

                for field in inner {
                    let mut field_inner = field.into_inner();

                    let key = field_inner.next().unwrap();
                    let value = field_inner.next().unwrap();

                    map.serialize_entry(
                        &LuaPestPair::from_pair(key),
                        &LuaPestPair::from_pair(value),
                    )?;
                }

                map.end()
            }
            Rule::binary_expr => match pair.as_str() {
                "-" => {
                    //
                    serializer.serialize_none()
                }
                _ => Err(Error::custom(format!("Invalid binary expression: {pair}"))),
            },
            Rule::int => match pair.as_str().parse::<i32>() {
                Ok(value) => serializer.serialize_i32(value),
                Err(err) => Err(Error::custom(err)),
            },
            Rule::float => match pair.as_str().parse::<f64>() {
                Ok(value) => serializer.serialize_f64(value),
                Err(err) => Err(Error::custom(err)),
            },
            Rule::string => {
                let value = pair.as_str().trim_matches('\'').trim_matches('"');
                serializer.serialize_str(value)
            }
            Rule::boolean => serializer.serialize_bool(pair.as_str() == "true"),
            Rule::nil => serializer.serialize_none(),
            Rule::ident => serializer.serialize_str(pair.as_str()),
            _ => unimplemented!("{pair}"),
        }
    }
}
