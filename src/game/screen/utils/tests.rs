use super::*;

#[test]
#[should_panic(expected = "Not enough digits")]
fn not_enough_digits() {
    number_to_string_leading_ascii(0, 42, false);
}

#[test]
#[should_panic(expected = "Too many digits")]
fn too_many_digits() {
    number_to_string_leading_ascii(10, 42, false);
}

#[test]
#[should_panic(expected = "Number too large")]
fn number_too_large_1_digit() {
    number_to_string_leading_ascii(1, 36, false);
}

#[test]
#[should_panic(expected = "Number too large")]
fn number_too_large_2_digits() {
    number_to_string_leading_ascii(2, 360, false);
}

#[test]
#[should_panic(expected = "Number too large")]
fn number_too_large_3_digits() {
    number_to_string_leading_ascii(3, 3600, false);
}

#[test]
#[should_panic(expected = "Number too large")]
fn number_too_large_4_digits() {
    number_to_string_leading_ascii(4, 36000, false);
}

#[test]
#[should_panic(expected = "Number too large")]
fn number_too_large_5_digits() {
    number_to_string_leading_ascii(5, 360000, false);
}

#[test]
#[should_panic(expected = "Number too large")]
fn number_too_large_6_digits() {
    number_to_string_leading_ascii(6, 3600000, false);
}

#[test]
#[should_panic(expected = "Number too large")]
fn number_too_large_7_digits() {
    number_to_string_leading_ascii(7, 36000000, false);
}

#[test]
#[should_panic(expected = "Number too large")]
fn number_too_large_8_digits() {
    number_to_string_leading_ascii(8, 360000000, false);
}

#[test]
#[should_panic(expected = "Number too large")]
fn number_too_large_9_digits() {
    number_to_string_leading_ascii(9, 3600000000, false);
}

#[test]
fn normal_numbers() {
    for i in 0..10 {
        assert_eq!(number_to_string_leading_ascii(1, i, false), i.to_string());
    }

    for i in (0..100).step_by(5) {
        assert_eq!(number_to_string_leading_ascii(2, i, false), format!("{:2}", i));
    }
    assert_eq!(number_to_string_leading_ascii(2, 9, false), " 9");
    assert_eq!(number_to_string_leading_ascii(2, 99, false), "99");

    for i in (0..1000).step_by(50) {
        assert_eq!(number_to_string_leading_ascii(3, i, false), format!("{:3}", i));
    }
    assert_eq!(number_to_string_leading_ascii(3, 9, false), "  9");
    assert_eq!(number_to_string_leading_ascii(3, 99, false), " 99");
    assert_eq!(number_to_string_leading_ascii(3, 999, false), "999");

    for i in (0..10000).step_by(500) {
        assert_eq!(number_to_string_leading_ascii(4, i, false), format!("{:4}", i));
    }
    assert_eq!(number_to_string_leading_ascii(4, 9, false), "   9");
    assert_eq!(number_to_string_leading_ascii(4, 99, false), "  99");
    assert_eq!(number_to_string_leading_ascii(4, 999, false), " 999");
    assert_eq!(number_to_string_leading_ascii(4, 9999, false), "9999");
}

#[test]
fn normal_numbers_leading_zeros() {
    for i in 0..10 {
        assert_eq!(number_to_string_leading_ascii(1, i, true), i.to_string());
    }

    for i in (0..100).step_by(5) {
        assert_eq!(number_to_string_leading_ascii(2, i, true), format!("{:02}", i));
    }
    assert_eq!(number_to_string_leading_ascii(2, 9, true), "09");
    assert_eq!(number_to_string_leading_ascii(2, 99, true), "99");

    for i in (0..1000).step_by(50) {
        assert_eq!(number_to_string_leading_ascii(3, i, true), format!("{:03}", i));
    }
    assert_eq!(number_to_string_leading_ascii(3, 9, true), "009");
    assert_eq!(number_to_string_leading_ascii(3, 99, true), "099");
    assert_eq!(number_to_string_leading_ascii(3, 999, true), "999");

    for i in (0..10000).step_by(500) {
        assert_eq!(number_to_string_leading_ascii(4, i, true), format!("{:04}", i));
    }
    assert_eq!(number_to_string_leading_ascii(4, 9, true), "0009");
    assert_eq!(number_to_string_leading_ascii(4, 99, true), "0099");
    assert_eq!(number_to_string_leading_ascii(4, 999, true), "0999");
    assert_eq!(number_to_string_leading_ascii(4, 9999, true), "9999");
}

#[test]
fn number_with_leading_ascii() {
    for i in 10..36 {
        assert_eq!(number_to_string_leading_ascii(1, i, false), ((b'A' + (i - 10) as u8) as char).to_string());
    }

    for i in (100..360).step_by(10) {
        assert_eq!(number_to_string_leading_ascii(2, i, false), ((b'A' + (i / 10 - 10) as u8) as char).to_string() + "0");
    }

    for i in (109..369).step_by(10) {
        assert_eq!(number_to_string_leading_ascii(2, i, false), ((b'A' + (i / 10 - 10) as u8) as char).to_string() + "9");
    }

    for i in (1000..3600).step_by(100) {
        assert_eq!(number_to_string_leading_ascii(3, i, false), ((b'A' + (i / 100 - 10) as u8) as char).to_string() + "00");
    }

    for i in (1099..3699).step_by(100) {
        assert_eq!(number_to_string_leading_ascii(3, i, false), ((b'A' + (i / 100 - 10) as u8) as char).to_string() + "99");
    }

    for i in (10000..36000).step_by(1000) {
        assert_eq!(number_to_string_leading_ascii(4, i, false), ((b'A' + (i / 1000 - 10) as u8) as char).to_string() + "000");
    }

    for i in (10999..36999).step_by(1000) {
        assert_eq!(number_to_string_leading_ascii(4, i, false), ((b'A' + (i / 1000 - 10) as u8) as char).to_string() + "999");
    }
}
