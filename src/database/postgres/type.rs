use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum PostgreSQLType {
    Text,
    Char(i32),
    VarChar(i32),
    Bit(i32),
    VarBit,
    UUID,
    Xml,
    Inet,
    Boolean,
    Integer,
    SmallInt,
    Int,
    BigInt,
    Oid,
    DoublePrecision,
    Real,
    Decimal(i32, i32),
    Money,
    Timestamp(i32),
    TimestampTz(i32),
    Json,
    JsonB,
    ByteA,
}
