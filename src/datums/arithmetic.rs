// Copyright 2020 The VectorDB Authors.
//
// Code is licensed under Apache License, Version 2.0.

use super::Datum;
use crate::errors::{DatumError, Error};

arithmetic!(
    add,
    +,
    Datum::Int32,
    Datum::Int64,
    Datum::Float32,
    Datum::Float64
);

arithmetic!(
    sub,
    -,
    Datum::Int32,
    Datum::Int64,
    Datum::Float32,
    Datum::Float64
);

arithmetic!(
    mul,
    *,
    Datum::Int32,
    Datum::Int64,
    Datum::Float32,
    Datum::Float64
);

arithmetic!(
    div,
    /,
    Datum::Int32,
    Datum::Int64,
    Datum::Float32,
    Datum::Float64
);

#[test]
fn test_arithmetic() {
    use super::Datum;

    // Add.
    {
        let ref d1 = Datum::Int32(1);
        let ref d2 = Datum::Int32(2);
        let d3 = add(d1, d2);
        assert_eq!(Datum::Int32(3), d3.unwrap());
    }

    // Sub.
    {
        let ref d1 = Datum::Int32(1);
        let ref d2 = Datum::Int32(2);
        let d3 = sub(d1, d2);
        assert_eq!(Datum::Int32(-1), d3.unwrap());
    }

    // div.
    {
        let ref d1 = Datum::Int32(1);
        let ref d2 = Datum::Int32(2);
        let d3 = div(d1, d2);
        assert_eq!(Datum::Int32(0), d3.unwrap());
    }

    // mul.
    {
        let ref d1 = Datum::Int32(3);
        let ref d2 = Datum::Int32(2);
        let d3 = mul(d1, d2);
        assert_eq!(Datum::Int32(6), d3.unwrap());
    }
}
