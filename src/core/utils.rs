/// 値が負か？ (⇔ 15bit目が1か？)
pub fn is_negative(value: u16) -> bool {
    value >> 15 == 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_negative() {
        assert_eq!(is_negative(0x8000), true);
        assert_eq!(is_negative(-123i16 as u16), true);
        assert_eq!(is_negative(32768), true);
        assert_eq!(is_negative(32767), false);
    }
}
