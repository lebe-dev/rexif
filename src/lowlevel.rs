use super::rational::*;
use std::mem;

/// Read value from a stream of bytes
#[inline]
pub(crate) fn read_u16(le: bool, raw: &[u8]) -> u16 {
    if le {
        u16::from(raw[1]) * 256 + u16::from(raw[0])
    } else {
        u16::from(raw[0]) * 256 + u16::from(raw[1])
    }
}

/// Read value from a stream of bytes
#[inline]
pub(crate) fn read_i16(le: bool, raw: &[u8]) -> i16 {
    let mut u = i32::from(read_u16(le, raw));
    if u >= 0x8000 {
        u -= 0x10000;
    }
    u as i16
}

/// Read value from a stream of bytes
#[inline]
pub(crate) fn read_u32(le: bool, raw: &[u8]) -> u32 {
    if le {
        ((raw[3] as u32) << 24) + ((raw[2] as u32) << 16) + ((raw[1] as u32) << 8) + raw[0] as u32
    } else {
        raw[3] as u32 + ((raw[2] as u32) << 8) + ((raw[1] as u32) << 16) + ((raw[0] as u32) << 24)
    }
}

/// Read value from a stream of bytes
#[inline]
pub(crate) fn read_i32(le: bool, raw: &[u8]) -> i32 {
    let mut u = read_u32(le, raw) as i64;
    if u >= 0x8000_0000 {
        u -= 0x0001_0000_0000;
    }
    u as i32
}

/// Read value from a stream of bytes
#[inline]
pub(crate) fn read_f32(raw: &[u8]) -> f32 {
    let mut a = [0 as u8; 4];
    // idiot, but guarantees that transmute gets a 4-byte buffer
    for i in 0..4 {
        a[i] = raw[i];
    }
    // FIXME I am not sure that TIFF floating point can be cast this way for any given architecture
    // The ideal thing would be to read mantissa, exponent, etc. explicitly
    let f: f32 = unsafe { mem::transmute(a) };
    f
}

/// Read value from a stream of bytes
#[inline]
pub(crate) fn read_f64(raw: &[u8]) -> f64 {
    let mut a = [0 as u8; 8];
    for i in 0..8 {
        a[i] = raw[i];
    }
    // FIXME I am not sure that TIFF floating point can be cast this way for any given architecture
    // The ideal thing would be to read mantissa, exponent, etc. explicitly
    let f: f64 = unsafe { mem::transmute(a) };
    f
}

/// Read value from a stream of bytes
#[inline]
pub(crate) fn read_urational(le: bool, raw: &[u8]) -> URational {
    let n = read_u32(le, &raw[0..4]);
    let d = read_u32(le, &raw[4..8]);
    URational {
        numerator: n,
        denominator: d,
    }
}

/// Read value from a stream of bytes
#[inline]
pub(crate) fn read_irational(le: bool, raw: &[u8]) -> IRational {
    let n = read_i32(le, &raw[0..4]);
    let d = read_i32(le, &raw[4..8]);
    IRational {
        numerator: n,
        denominator: d,
    }
}

/// Read array from a stream of bytes. Caller must be sure of count and buffer size
pub(crate) fn read_i8_array(count: u32, raw: &[u8]) -> Vec<i8> {
    let mut a = Vec::<i8>::new();
    for i in 0..count {
        a.push(raw[i as usize] as i8);
    }
    a
}

/// Read array from a stream of bytes. Caller must be sure of count and buffer size
pub(crate) fn read_u16_array(le: bool, count: u32, raw: &[u8]) -> Vec<u16> {
    let mut a = Vec::<u16>::new();
    let mut offset = 0;
    for _ in 0..count {
        a.push(read_u16(le, &raw[offset..offset + 2]));
        offset += 2;
    }
    a
}

/// Read array from a stream of bytes. Caller must be sure of count and buffer size
pub(crate) fn read_i16_array(le: bool, count: u32, raw: &[u8]) -> Vec<i16> {
    let mut a = Vec::<i16>::new();
    let mut offset = 0;
    for _ in 0..count {
        a.push(read_i16(le, &raw[offset..offset + 2]));
        offset += 2;
    }
    a
}

/// Read array from a stream of bytes. Caller must be sure of count and buffer size
pub(crate) fn read_u32_array(le: bool, count: u32, raw: &[u8]) -> Vec<u32> {
    let mut a = Vec::<u32>::new();
    let mut offset = 0;
    for _ in 0..count {
        a.push(read_u32(le, &raw[offset..offset + 4]));
        offset += 4;
    }
    a
}

/// Read array from a stream of bytes. Caller must be sure of count and buffer size
pub(crate) fn read_i32_array(le: bool, count: u32, raw: &[u8]) -> Vec<i32> {
    let mut a = Vec::<i32>::new();
    let mut offset = 0;
    for _ in 0..count {
        a.push(read_i32(le, &raw[offset..offset + 4]));
        offset += 4;
    }
    a
}

/// Read array from a stream of bytes. Caller must be sure of count and buffer size
pub(crate) fn read_f32_array(count: u32, raw: &[u8]) -> Vec<f32> {
    let mut a = Vec::<f32>::new();
    let mut offset = 0;
    for _ in 0..count {
        a.push(read_f32(&raw[offset..offset + 4]));
        offset += 4;
    }
    a
}

/// Read array from a stream of bytes. Caller must be sure of count and buffer size
pub(crate) fn read_f64_array(count: u32, raw: &[u8]) -> Vec<f64> {
    let mut a = Vec::<f64>::new();
    let mut offset = 0;
    for _ in 0..count {
        a.push(read_f64(&raw[offset..offset + 8]));
        offset += 8;
    }
    a
}

/// Read array from a stream of bytes. Caller must be sure of count and buffer size
pub(crate) fn read_urational_array(le: bool, count: u32, raw: &[u8]) -> Vec<URational> {
    let mut a = Vec::<URational>::new();
    let mut offset = 0;
    for _ in 0..count {
        a.push(read_urational(le, &raw[offset..offset + 8]));
        offset += 8;
    }
    a
}

/// Read array from a stream of bytes. Caller must be sure of count and buffer size
pub(crate) fn read_irational_array(le: bool, count: u32, raw: &[u8]) -> Vec<IRational> {
    let mut a = Vec::<IRational>::new();
    let mut offset = 0;
    for _ in 0..count {
        a.push(read_irational(le, &raw[offset..offset + 8]));
        offset += 8;
    }
    a
}
