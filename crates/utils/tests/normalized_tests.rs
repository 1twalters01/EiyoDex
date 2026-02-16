use utils::base_types::normalized::Normalized;

#[test]
fn test_normalization() {
    let too_large = Normalized::new(2f64);
    assert!(too_large.is_err());

    let too_low = Normalized::new(-0.5);
    assert!(too_low.is_err());

    let normalized = Normalized::new(0.62);
    assert!(normalized.is_ok());
    assert_eq!(normalized.unwrap().get_value(), 0.62);
}
