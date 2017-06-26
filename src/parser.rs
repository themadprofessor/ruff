use nom::{be_u32, be_u16, IResult};

use super::Pixel;
use super::Farbfeld;
use super::error::*;

named!(pub parse_pixel<Pixel>, do_parse!(
    red: be_u16 >>
    green: be_u16 >>
    blue: be_u16 >>
    alpha: be_u16 >>
    (Pixel::new(red, green, blue, alpha))
));

named!(pub parse_farb<Farbfeld>, do_parse!(
    tag!("farbfeld") >>
    width: be_u32 >>
    height: be_u32 >>
    pixels: many0!(flat_map!(take!(8), parse_pixel)) >>
    res: expr_res!(Farbfeld::new(width, height, pixels)) >>
    (res)
));

pub fn i_to_res<I, O>(res: IResult<I, O, u32>) -> Result<O> {
    match res {
        IResult::Incomplete(need) => Err(Error::from(ErrorKind::NotEnoughDataError(need))),
        IResult::Done(_, farb) => Ok(farb),
        IResult::Error(err) => Err(Error::from(ErrorKind::NomError(err)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pixel() {
        let data = [0_u8, 1_u8, 0_u8, 2_u8, 0_u8, 3_u8, 0_u8, 4_u8];
        assert_eq!(Pixel::new(1_u16, 2_u16, 3_u16, 4_u16), i_to_res(parse_pixel(&data)).unwrap());
    }

    #[test]
    fn test_parse_farb() {
        let mut data = "farbfeld".as_bytes().to_vec();
        data.extend([0,0,0,1, 0,0,0,1,    0,1, 0,1, 0,1, 0,1].iter());
        let correct = Farbfeld::new(1_u32, 1_u32, Vec::from([Pixel::new(1_u16, 1_u16, 1_u16, 1_u16)].to_vec())).unwrap();
        let test = i_to_res(parse_farb(&data)).unwrap();

        assert_eq!(correct.width(), test.width());
        assert_eq!(correct.height(), test.height());
        assert_eq!(correct.pixels(), test.pixels());
    }
}