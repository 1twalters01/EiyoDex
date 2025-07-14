use crate::define_currencies;

define_currencies! {
    USD => {
        symbol: "$",
        code: "USD",
        unit_type: "dollar",
        unit_type_plural: "dollars"
    },
    EUR => {
        symbol: "€",
        code: "EUR",
        unit_type: "euro",
        unit_type_plural: "euros"
    },
    JPY => {
        symbol: "¥",
        code: "JPY",
        unit_type: "yen",
        unit_type_plural: "yen"
    },
    GBP => {
        symbol: "£",
        code: "GBP",
        unit_type: "pound",
        unit_type_plural: "pounds"
    }
}
