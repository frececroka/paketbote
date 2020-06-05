use std::fmt;
use std::io::Write;
use std::str::FromStr;

use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Text;
use fehler::throws;
use serde::Serialize;

use crate::error::Error;

#[derive(Debug, Clone, Copy, Serialize, FromSqlRow, AsExpression)]
#[sql_type = "Text"]
pub enum Compression {
    Zstd, Gzip, Lzma
}

impl FromStr for Compression {
    type Err = Error;
    #[throws]
    fn from_str(string: &str) -> Self {
        match string {
            "xz" => Compression::Lzma,
            "gz" => Compression::Gzip,
            "zst" => Compression::Zstd,
            _ => Err(format!("Unknown compression format {}", string))?
        }
    }
}

impl fmt::Display for Compression {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self {
            Compression::Lzma => "xz",
            Compression::Gzip => "gz",
            Compression::Zstd => "zst",
        };
        write!(fmt, "{}", repr)
    }
}

impl<DB> FromSql<Text, DB> for Compression
    where DB: Backend, String: FromSql<Text, DB>,
{
    #[throws(Box<dyn std::error::Error + Send + Sync>)]
    fn from_sql(bytes: Option<&DB::RawValue>) -> Self {
        String::from_sql(bytes)?.parse()?
    }
}

impl<DB> ToSql<Text, DB> for Compression
    where DB: Backend, String: ToSql<Text, DB>,
{
    #[throws(Box<dyn std::error::Error + Send + Sync>)]
    fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> IsNull {
        self.to_string().to_sql(out)?
    }
}

