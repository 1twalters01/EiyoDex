use std::collections::BTreeSet;

use identity::{entity::Entity, Id, InnerId};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use crate::{
    nutrient_quantity::NutrientQuantity, nutrient_quantity_list::NutrientQuantityList,
    records::nutrient_quantity_record::NutrientQuantityRecord,
};

#[derive(Debug, Clone, PartialEq)]
pub struct NutrientQuantityListRecord {
    id: Vec<u8>,
    name: String,
    description: String,
}

impl NutrientQuantityListRecord {
    pub fn from_value(id: Vec<u8>, name: String, description: String) -> Self {
        Self { id, name, description }
    }

    pub fn from_nutrient_quantity_list_entity(nutrient_quantity_list_entity: Entity<NutrientQuantityList>) -> Self {
        let id = nutrient_quantity_list_entity.get_id().to_bytes().to_vec();
        let nutrient_quantity_list = nutrient_quantity_list_entity.get_inner();
        let name = nutrient_quantity_list.get_name();
        let description = nutrient_quantity_list.get_description();
        Self { id, name, description }
    }

    pub fn to_nutrient_quantity_list(&self) -> NutrientQuantityList {
        let mut nutrient_quantity_list = NutrientQuantityList::new();
        nutrient_quantity_list.set_name(self.name.clone());
        nutrient_quantity_list.set_description(self.description.clone());

        return nutrient_quantity_list;
    }

    pub fn to_nutrient_quantity_list_entity(&self) -> Result<Entity<NutrientQuantityList>, uuid::Error> {
        let mut nutrient_quantity_list = NutrientQuantityList::new();
        nutrient_quantity_list.set_name(self.name.clone());
        nutrient_quantity_list.set_description(self.description.clone());

        let id = Id::from_inner(InnerId::Uuid(Uuid::from_slice(&self.id)?));
        Ok(Entity::new_with_id(id, nutrient_quantity_list))
    }

    pub fn to_nutrient_list_entity(&self) -> Result<Entity<NutrientQuantityList>, uuid::Error> {
        let mut nutrient_quantity_list = NutrientQuantityList::new();
        nutrient_quantity_list.set_name(self.name.clone());
        nutrient_quantity_list.set_description(self.description.clone());

        let id = Id::from_inner(InnerId::Uuid(Uuid::from_slice(&self.id)?));
        Ok(Entity::new_with_id(id, nutrient_quantity_list))
    }

    pub fn get_id(&self) -> Vec<u8> {
        self.id.clone()
    }

    pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO nutrients_nutrient_quantity_list_table (id, name, description)
                VALUES (?, ?, ?)
                ON CONFLICT (id)
                DO NOTHING
            "#,
            self.id,
            self.name,
            self.description,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn save_or_update_to_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO nutrients_nutrient_quantity_list_table (id, name, description)
                VALUES (?, ?, ?)
                ON CONFLICT (id) DO UPDATE SET 
                    name = excluded.name,
                    description = excluded.description
            "#,
            self.id,
            self.name,
            self.description,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn get_all_from_sqlite(pool: &Pool<Sqlite>) -> Result<Vec<Self>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
                Select id, "name", "description" FROM nutrients_nutrient_quantity_list_table
            "#,
        )
            .fetch_all(pool)
            .await?;

        let nutrient_list_record_vec: Vec<NutrientQuantityListRecord> = rows
        .into_iter()
        .map(|row| NutrientQuantityListRecord {
            id: row.id,
            name: row.name,
            description: row.description,
        })
        .collect();
        
        Ok(nutrient_list_record_vec)
    }

    pub async fn delete_from_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM nutrients_nutrient_quantity_list_table WHERE id = ?",
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete_all_items_from_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                DELETE FROM nutrients_nutrient_quantity_list_items
                WHERE
                    nutrient_quantity_list_id = ?
            "#,
            self.id,
        )
        .execute(pool)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NutrientQuantityListItemRecord {
    nutrient_quantity_list_id: Vec<u8>,
    nutrient_quantity_id: Vec<u8>,
}

impl NutrientQuantityListItemRecord {
    pub fn from_value(nutrient_quantity_list_id: Vec<u8>, nutrient_quantity_id: Vec<u8>) -> Self {
        Self { nutrient_quantity_list_id, nutrient_quantity_id }
    }

    pub async fn from_nutrient_quantity_list_entity(
        nutrient_quantity_list_entity: Entity<NutrientQuantityList>,
    ) -> Vec<Self> {
        let nutrient_quantity_list_id = nutrient_quantity_list_entity.get_id().to_bytes().to_vec();
        let nutrient_quantity_list = nutrient_quantity_list_entity.get_inner();
        let mut nutrient_quantity_list_item_vec: Vec<Self> = Vec::new();

        for nutrient_quantity_entity in nutrient_quantity_list.get_nutrient_quantities() {
            let nutrient_quantity_id = nutrient_quantity_entity.get_id().to_bytes().to_vec();
            let item = Self {
                nutrient_quantity_list_id: nutrient_quantity_list_id.clone(),
                nutrient_quantity_id,
            };
            nutrient_quantity_list_item_vec.push(item);
        }

        return nutrient_quantity_list_item_vec
    }

    pub async fn to_nutrient_quantity(self, pool: &Pool<Sqlite>) -> Result<NutrientQuantity, sqlx::Error> {
            let nutrient_quantity_record = sqlx::query_as!(
                NutrientQuantityRecord,
                r#"
                    SELECT
                        id,
                        quantity,
                        nutrient_id,
                        output_unit_id
                    FROM nutrients_nutrient_quantity_table
                    WHERE
                        id = ?
                "#,
                self.nutrient_quantity_id
            )
            .fetch_one(pool)
            .await?;

            Ok(nutrient_quantity_record.to_nutrient_quantity(pool).await?)
    }

    pub async fn to_btree_map_from_vec(
        items: Vec<&Self>, pool: &Pool<Sqlite>,
    ) -> Result<BTreeSet<NutrientQuantity>, sqlx::Error> {
        if let Some(first) = items.first() {
            if !items
                .iter()
                .all(|item| item.nutrient_quantity_list_id == first.nutrient_quantity_list_id)
            {
                panic!("List has elements of a different list")
            };
        }

        let mut tx = pool.begin().await?;

        let mut quantity_tree: BTreeSet<NutrientQuantity> = BTreeSet::new();
        for item in items {
            let nutrient_quantity_record = sqlx::query_as!(
                NutrientQuantityRecord,
                r#"
                    SELECT
                        id,
                        quantity,
                        nutrient_id,
                        output_unit_id
                    FROM nutrients_nutrient_quantity_table
                    WHERE
                        id = ?
                "#,
                item.nutrient_quantity_id
            )
            .fetch_one(&mut *tx)
            .await?;

            let nutrient_quantity = nutrient_quantity_record.to_nutrient_quantity(pool).await?;
            quantity_tree.insert(nutrient_quantity);
        }

        return Ok(quantity_tree);
    }

    pub async fn load_all_from_sqlite(
        nutrient_quantity_list_id: Uuid, pool: &Pool<Sqlite>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        Ok(sqlx::query_as!(
            NutrientQuantityListItemRecord,
            r#"
                    SELECT
                        nutrient_quantity_list_id,
                        nutrient_quantity_id
                    FROM nutrients_nutrient_quantity_list_items
                    WHERE
                        nutrient_quantity_list_id = ?
                "#,
            nutrient_quantity_list_id
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO nutrients_nutrient_quantity_list_items (nutrient_quantity_list_id, nutrient_quantity_id)
                VALUES (?, ?)
                ON CONFLICT(nutrient_quantity_list_id, nutrient_quantity_id) DO NOTHING
            "#,
            self.nutrient_quantity_list_id,
            self.nutrient_quantity_id,
        )
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn save_vec_to_database(items: Vec<&Self>, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        println!("items: {:#?}", items);
        for item in items {
            println!("item: {:?}", item);
            let id = sqlx::query!(
                r#"
                    INSERT INTO nutrients_nutrient_quantity_list_items (nutrient_quantity_list_id, nutrient_quantity_id)
                    VALUES (?, ?)
                    ON CONFLICT(nutrient_quantity_list_id, nutrient_quantity_id) DO NOTHING
                "#,
                item.nutrient_quantity_list_id,
                item.nutrient_quantity_id,
            )
            .execute(&mut *tx)
            .await?;
            println!("id: {:#?}", id);
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn delete_item_from_sqlite(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                DELETE FROM nutrients_nutrient_quantity_list_items 
                WHERE
                    nutrient_quantity_list_id = ?
                    AND nutrient_quantity_id = ?
            "#,
            self.nutrient_quantity_list_id,
            self.nutrient_quantity_id,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete_item_vec_from_sqlite(items: Vec<&Self>, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        for item in items {
            sqlx::query!(
                r#"
                    DELETE FROM nutrients_nutrient_quantity_list_items
                    WHERE
                        nutrient_quantity_list_id = ?
                        AND nutrient_quantity_id = ?
                "#,
                item.nutrient_quantity_list_id,
                item.nutrient_quantity_id,
            )
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;

        Ok(())
    }
}
