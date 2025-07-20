# Food
Metadata: {
    id: uuid4
    name: String
    subset_of: superset_id (uuid)
}
Data: {
    Source: {
        source_id: uuid
    }
    Tags: Tag_id uuid4
    pral alkalinity: {
        per: {
            units: Unit
            size: f64
        }
        pral alkalinity: f64
    }
    nutrients: {
        per: {
            units: Unit
            size: f64
        }
        nutrients: nutrient_id[]
        nutrient amount: f64
    }
}[]


# Tag
id: uuid
name: String
subset of: tag_id | None