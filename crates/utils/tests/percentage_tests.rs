use utils::base_types::percentage::Percentage;

#[test]
fn test_percentages() {
    let large = Percentage::new(100.5f64);
    assert!(!large.is_proportion());

    let low = Percentage::new(-0.5);
    assert!(!low.is_proportion());

    let proportion_1 = Percentage::new(62.8);
    assert!(proportion_1.is_proportion());

    let proportion_2 = Percentage::new(0f64);
    assert!(proportion_2.is_proportion());

    let proportion_3 = Percentage::new(0.1);
    assert!(proportion_3.is_proportion());

    let proportion_4 = Percentage::new(100f64);
    assert!(proportion_4.is_proportion());

    let proportion_5 = Percentage::new(99.99999);
    assert!(proportion_5.is_proportion());

    assert_eq!(proportion_1 + proportion_3, Percentage::new(62.9));
    assert!((proportion_1 - proportion_3).get_value() - 62.7 < 0.000001);
}
