//! Rust Types.

use crate::sql_types::*;
use diesel::{
    deserialize::{self, FromSql},
    pg::Pg,
    serialize::{self, IsNull, Output, ToSql},
};
use postgis::ewkb::{AsEwkbGeometry, EwkbRead, EwkbWrite, GeometryT, Point};
use std::io::Write;

#[derive(Debug, Clone, FromSqlRow, AsExpression)]
#[sql_type = "Geometry"]
pub struct GeometryWrapper(pub GeometryT<Point>);

impl FromSql<Geometry, Pg> for GeometryWrapper {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let mut bytes = not_none!(bytes);
        Ok(GeometryWrapper(GeometryT::read_ewkb(&mut bytes)?))
    }
}

impl ToSql<Geometry, Pg> for GeometryWrapper {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        self.0.as_ewkb().write_ewkb(out)?;
        Ok(IsNull::No)
    }
}
