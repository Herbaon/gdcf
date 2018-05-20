use core::backend::pg::Pg;
use core::query::QueryPart;
use core::types::{BigInteger, Boolean, Double, Float, Integer, Text, SmallInteger, Unsigned, UtcTimestamp};

simple_query_part!(Pg, Text, "TEXT");
simple_query_part!(Pg, SmallInteger, "SMALLINT");
simple_query_part!(Pg, Integer, "INT");
simple_query_part!(Pg, BigInteger, "BIGINT");
simple_query_part!(Pg, Boolean, "BOOL");
simple_query_part!(Pg, Float, "FLOAT(4)");
simple_query_part!(Pg, Double, "REAL");
simple_query_part!(Pg, Unsigned<SmallInteger>, "SMALLINT");
simple_query_part!(Pg, Unsigned<Integer>, "INTEGER");
simple_query_part!(Pg, Unsigned<BigInteger>, "BIGINT");
simple_query_part!(Pg, UtcTimestamp, "TIMESTAMP WITHOUT TIME ZONE");