Potentially move to a file with rows of the form:
    {
        symbol: "£",
        code: "GBP"
        unit_type: "pound",
        unit_type_plural: "pounds"
    }

Then have a function to just read from the file?
Is this better than hard coding everything in rust?
Not if only having GBP/USD but yes if having lots of currencies
I would still need the CurrencyUnit enum and it would get big