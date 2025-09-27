use foods::price_metadata::Merchant;
use uuid::Uuid;

#[test]
pub fn test_merchant() {
    let name = String::from("Butcher Y");
    let id = None;
    let mut merchant = Merchant::new(name, id);

    assert_eq!(id, None);
    let new_id = Uuid::new_v4();
    merchant.set_id(new_id);
    assert_eq!(merchant.get_id(), new_id);

    assert_eq!(merchant.get_name(), String::from("Butcher Y"));
    merchant.set_name(String::from("Supermarket Z"));
    assert_eq!(merchant.get_name(), String::from("Supermarket Z"));

    assert_eq!(merchant.get_description(), String::from(""));
    let description = String::from("Merchant description");
    merchant.set_description(description);
    assert_eq!(
        merchant.get_description(),
        String::from("Merchant description")
    );

    assert_eq!(merchant.get_website(), String::from(""));
    let website = String::from("merchantsite.com");
    merchant.set_website(website);
    assert_eq!(merchant.get_website(), String::from("merchantsite.com"));
}
