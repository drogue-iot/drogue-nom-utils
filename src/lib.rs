#![no_std]

mod num;

use nom::{
    IResult,
    character::streaming::digit1,
};
use num::{
    atoi_u8,
    atoi_usize,
};

pub fn parse_u8(input: &[u8]) -> IResult<&[u8], u8> {
    let (input, digits) = digit1(input)?;
    IResult::Ok((input, atoi_u8(digits).unwrap()))
}

pub fn parse_usize(input: &[u8]) -> IResult<&[u8], usize> {
    let (input, digits) = digit1(input)?;
    let num = atoi_usize(digits).unwrap();
    IResult::Ok((input, num))
}