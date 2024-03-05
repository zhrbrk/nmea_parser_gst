use chrono::NaiveTime;

use nom::{
    character::complete::char,
    number::complete::float,
    combinator::opt,
    IResult,
};

use crate::{parse::NmeaSentence, sentences::utils::parse_hms, Error, SentenceType};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// GST - GPS Pseudorange Noise Statistics
//
//              1    2 3 4 5 6 7 8   9
//              |    | | | | | | |   |
// $ --GST,hhmmss.ss,x,x,x,x,x,x,x*hh<CR><LF>
// 
// Example: $GPGST,182141.000,15.5,15.3,7.2,21.8,0.9,0.5,0.8*54
//
// 1. UTC time of associated GGA fix
// 2. Total RMS standard deviation of ranges inputs to the navigation solution
// 3. Standard deviation (meters) of semi-major axis of error ellipse
// 4. Standard deviation (meters) of semi-minor axis of error ellipse
// 5. Orientation of semi-major axis of error ellipse (true north degrees)
// 6. Standard deviation (meters) of latitude error
// 7. Standard deviation (meters) of longitude error
// 8. Standard deviation (meters) of altitude error
// 9. Checksum
// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct GstData {
    #[cfg_attr(feature = "defmt-03", defmt(Debug2Format))]
    pub time: Option<NaiveTime>,
    pub rms_sd: Option<f32>,
    pub ellipse_semi_major_sd: Option<f32>,
    pub ellipse_semi_minor_sd: Option<f32>,
    pub err_ellipse_orientation: Option<f32>,
    pub lat_sd: Option<f32>,
    pub long_sd: Option<f32>,
    pub alt_sd: Option<f32>,
}
 
fn do_parse_gst(i: &str) -> IResult<&str, GstData> {
    let (i, time) = opt(parse_hms)(i)?;
    let (i,_) = char(',')(i)?;

    let (i, rms_sd) = opt(float)(i)?;
    let (i,_) = char(',')(i)?;

    let (i, ellipse_semi_major_sd) = opt(float)(i)?;
    let (i,_) = char(',')(i)?;

    let (i, ellipse_semi_minor_sd) = opt(float)(i)?;
    let (i,_) = char(',')(i)?;

    let (i, err_ellipse_orientation) = opt(float)(i)?;
    let (i,_) = char(',')(i)?;

    let (i, lat_sd) = opt(float)(i)?;
    let (i,_) = char(',')(i)?;

    let (i, long_sd) = opt(float)(i)?;
    let (i,_) = char(',')(i)?;

    let (i, alt_sd) = opt(float)(i)?;

    Ok((
        i,
        GstData {
            time,
            rms_sd,
            ellipse_semi_major_sd,
            ellipse_semi_minor_sd,
            err_ellipse_orientation,
            lat_sd,
            long_sd,
            alt_sd,
        },
    ))
}
pub fn parse_gst(sentence: NmeaSentence) -> Result<GstData, Error> {
    if sentence.message_id != SentenceType::GST {
        Err(Error::WrongSentenceHeader {
            expected: SentenceType::GST,
            found: sentence.message_id,
        })
    } else {
        Ok(do_parse_gst(sentence.data)?.1)
    }
}