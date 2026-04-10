use std::{cell::RefCell, collections::{BTreeMap, HashMap}, rc::Rc};

use identity::{inner_id::InnerIdType, Id, InnerId};
use sqlx::{Pool, Sqlite};
use utils::dsa::node::GraphNode;
use uuid::Uuid;

use crate::{
    entity::Entity, nutrient::{link_parent_child, Nutrient}, nutrient_list::NutrientList, nutrient_units::NutrientUnit, records::{nutrient_type_record::NutrientTypeRecord, nutrient_unit_record::NutrientUnitRecord}
};

#[derive(Debug, PartialEq)]
pub struct NutrientRecord {
    pub nutrient_id: Vec<u8>,
    pub name: String,
    pub description: String,
    pub main_unit_id: i64,
    pub essentiality_type_id: i64,
    pub quantity_type_id: i64,
    pub chemical_id: i64,
}

impl NutrientRecord {
    pub fn from_values(
        nutrient_id: Vec<u8>,
        name: String,
        description: String,
        main_unit_id: i64,
        essentiality_type_id: i64,
        quantity_type_id: i64,
        chemical_id: i64,
    ) -> Self {
        Self {
            nutrient_id,
            name,
            description,
            main_unit_id,
            essentiality_type_id,
            quantity_type_id,
            chemical_id,
        }
    }

    pub async fn from_nutrient(nutrient: Nutrient, pool: &Pool<Sqlite>) -> Result<Self, sqlx::Error> {
        let nutrient_record_option = Self::load_from_database_using_name(nutrient.get_name(), &pool).await;
        let nutrient_id = match nutrient_record_option {
            Ok(record) => record.nutrient_id,
            Err(sqlx::Error::RowNotFound) => Id::<Nutrient>::new(InnerIdType::Uuid).to_bytes().to_vec(),
            Err(err) => return Err(err),
        };

        let name = nutrient.get_name();
        let description = nutrient.get_description();
        let nutrient_type_record = NutrientTypeRecord::from_nutrient_type(nutrient.get_nutrient_type());
        let main_unit_id = NutrientUnitRecord::from_nutrient_unit(nutrient.get_main_unit(), pool)
            .await
            .get_database_id(&pool)
            .await?;

        let essentiality_type_id = nutrient_type_record.get_essentiality_type_id();
        let quantity_type_id = nutrient_type_record.get_quantity_type_id();
        let chemical_id = nutrient_type_record.get_chemical_id_from_database(&pool).await?;

        Ok(Self {
            nutrient_id,
            name,
            description,
            essentiality_type_id,
            quantity_type_id,
            chemical_id,
            main_unit_id,
        })
    }

    pub async fn from_nutrient_entity(nutrient_entity: Entity<Nutrient>, pool: &Pool<Sqlite>) -> Result<Self, &'static str> {
        let nutrient_id = nutrient_entity.get_id().get_inner().to_bytes().to_vec();

        let nutrient = nutrient_entity.get_inner();
        let name = nutrient.get_name();
        let description = nutrient.get_description();
        let nutrient_type_record = NutrientTypeRecord::from_nutrient_type(nutrient.get_nutrient_type());
        let main_unit_id = NutrientUnitRecord::from_nutrient_unit(nutrient.get_main_unit(), pool)
            .await
            .get_database_id(&pool)
            .await
            .unwrap();

        let essentiality_type_id = nutrient_type_record.get_essentiality_type_id();
        let quantity_type_id = nutrient_type_record.get_quantity_type_id();
        let chemical_id = nutrient_type_record.get_chemical_id_from_database(&pool).await.unwrap();

        Ok(Self {
            nutrient_id,
            name,
            description,
            essentiality_type_id,
            quantity_type_id,
            chemical_id,
            main_unit_id,
        })
    }

    pub async fn to_nutrient(&self, pool: &Pool<Sqlite>) -> Result<Nutrient, sqlx::Error> {
        let name = self.name.clone();
        let description = self.description.clone();
        let nutrient_type_record =
            NutrientTypeRecord::load_from_database_from_nutrient_composite_id(
                self.essentiality_type_id,
                self.quantity_type_id,
                self.chemical_id,
                pool,
            )
            .await?;
        let nutrient_type = nutrient_type_record.to_nutrient_type();
        let main_unit_id = NutrientUnitRecord::load_from_database(self.main_unit_id, pool)
            .await?
            .to_nutrient_unit(pool)
            .await;

        let mut nutrient = Nutrient::new(name, nutrient_type, main_unit_id);
        nutrient.set_description(description);

        Ok(nutrient)
    }

    pub async fn to_nutrient_entity(&self, pool: &Pool<Sqlite>) -> Entity<Nutrient> {
        let name = self.name.clone();
        let description = self.description.clone();
        let nutrient_type_record =
            NutrientTypeRecord::load_from_database_from_nutrient_composite_id(
                self.essentiality_type_id,
                self.quantity_type_id,
                self.chemical_id,
                pool,
            )
            .await
            .unwrap();
        let nutrient_type = nutrient_type_record.to_nutrient_type();
        let main_unit_id = NutrientUnitRecord::load_from_database(self.main_unit_id, pool)
            .await
            .unwrap()
            .to_nutrient_unit(pool)
            .await;

        let mut nutrient = Nutrient::new(name, nutrient_type, main_unit_id);
        nutrient.set_description(description);

        // let id = Uuid::from_slice(&self.nutrient_id).unwrap();
        let id = Id::<Nutrient>::from_slice(InnerIdType::Uuid, &self.nutrient_id).unwrap();
        let nutrient_entity = Entity::<Nutrient>::new_with_id(id, nutrient);

        return nutrient_entity;
    }

    pub async fn select_nutrient_from_nutrient_list(&self, nutrient_list: NutrientList, pool: &Pool<Sqlite>) -> Option<Rc<RefCell<Nutrient>>> {
        let nutrient_name = self.to_nutrient(pool).await.unwrap().get_name();

        let nutrients: Vec<Rc<RefCell<Nutrient>>> = nutrient_list
            .get_nutrients()
            .iter()
            .filter(|nutrient| nutrient.borrow().get_name() == nutrient_name)
            .map(|nutrient| nutrient.clone())
            .collect();

        if nutrients.len() == 0 { panic!("No name found") }
        let nutrient: Option<Rc<RefCell<Nutrient>>> = match nutrients.first() {
            Some(nutrient) => {
                if nutrients.iter().all(|n| Rc::ptr_eq(n, &nutrient)) == false { panic!("Different nutrient definitions found") };
                Some(nutrient.clone())
            },
            None => None,
        };
        return nutrient;
    }

    pub async fn load_from_database_using_name(name: String, pool: &Pool<Sqlite>) -> Result<Self, sqlx::Error> {
        Ok(sqlx::query_as!(
            NutrientRecord,
            r#"
                    SELECT
                        id as nutrient_id,
                        name,
                        description,
                        main_unit_id,
                        quantity_type_id,
                        essentiality_type_id,
                        chemical_id
                    FROM nutrients_nutrient_table
                    WHERE
                        name = ?
                "#,
            name
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn load_from_database_using_id(nutrient_id: Id<Nutrient>, pool: &Pool<Sqlite>) -> Result<Self, sqlx::Error> {
        let id = nutrient_id.get_inner().to_bytes().to_vec();

        Ok(sqlx::query_as!(
            NutrientRecord,
            r#"
                    SELECT
                        id as nutrient_id,
                        name,
                        description,
                        main_unit_id,
                        quantity_type_id,
                        essentiality_type_id,
                        chemical_id
                    FROM nutrients_nutrient_table
                    WHERE
                        id = ?
                "#,
            id
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<Vec<u8>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
                INSERT INTO nutrients_nutrient_table (id, name, description, main_unit_id, quantity_type_id, essentiality_type_id, chemical_id)
                VALUES (?, ?, ?, ?, ?, ?, ?)
                ON CONFLICT (name)
                DO UPDATE SET
                    description = excluded.description,
                    main_unit_id = excluded.main_unit_id,
                    quantity_type_id = excluded.quantity_type_id,
                    essentiality_type_id = excluded.essentiality_type_id,
                    chemical_id = excluded.chemical_id
                RETURNING id
            "#,
            self.nutrient_id,
            self.name,
            self.description,
            self.main_unit_id,
            self.quantity_type_id,
            self.essentiality_type_id,
            self.chemical_id,
        )
            .fetch_one(pool)
            .await?;
        Ok(row.id)
    }

    pub async fn delete_nutrient_from_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM nutrients_nutrient_table WHERE id = ?",
            self.nutrient_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct NutrientConversionRecord {
    pub(crate) nutrient_id: Vec<u8>,
    pub(crate) unit_id: i64,
    pub(crate) factor: f64,
}

impl NutrientConversionRecord {
    pub fn from_values(nutrient_id: Vec<u8>, unit_id: i64, factor: f64) -> Self {
        Self { nutrient_id, unit_id, factor }
    }

    pub async fn from_nutrient(nutrient: Nutrient, pool: &Pool<Sqlite>) -> Result<Vec<Self>, &'static str> {
        let nutrient_id = Id::<Nutrient>::new(InnerIdType::Uuid).to_bytes().to_vec();
        let mut conversion_vec = Vec::new();

        for (unit, factor) in nutrient.get_unit_conversions().iter() {
            let unit_id = NutrientUnitRecord::from_nutrient_unit(*unit, pool)
                .await
                .get_database_id(&pool)
                .await
                .expect("invalid nutrient");
            let factor = *factor;
            let conversion = Self {
                nutrient_id: nutrient_id.clone(),
                unit_id,
                factor,
            };
            conversion_vec.push(conversion);
        }

        Ok(conversion_vec)
    }

    pub async fn from_nutrient_entity(nutrient_entity: Entity<Nutrient>, pool: &Pool<Sqlite>) -> Result<Vec<Self>, &'static str> {
        let nutrient_id = nutrient_entity.get_id().get_inner().to_bytes().to_vec();
        let nutrient = nutrient_entity.get_inner();

        let mut conversion_vec = Vec::new();

        for (unit, factor) in nutrient.get_unit_conversions().iter() {
            println!("unit: {:#?}", unit);
            let unit_id = NutrientUnitRecord::from_nutrient_unit(*unit, pool)
                .await
                .get_database_id(&pool)
                .await
                .expect("invalid nutrient");
            let factor = *factor;
            let conversion = Self {
                nutrient_id: nutrient_id.clone(),
                unit_id,
                factor,
            };
            conversion_vec.push(conversion);
        }

        Ok(conversion_vec)
    }

    pub async fn to_btree_map_from_vec(items: Vec<Self>, pool: &Pool<Sqlite>) -> Result<BTreeMap<NutrientUnit, f64>, sqlx::Error> {
        let mut map: BTreeMap<NutrientUnit, f64> = BTreeMap::new();
        for conversion in items.iter() {
            let unit = NutrientUnitRecord::load_from_database(conversion.unit_id, pool)
                .await?
                .to_nutrient_unit(pool)
                .await;
            let factor = conversion.factor;
            map.insert(unit, factor);
        }

        return Ok(map);
    }

    pub fn sort_records(items: &mut Vec<Self>) {
        items.sort_by(|a, b| {
            a.nutrient_id
                .cmp(&b.nutrient_id)
                .then(a.unit_id.cmp(&b.unit_id))
                .then(a.factor.partial_cmp(&b.factor).unwrap())
        });
    }

    pub async fn load_from_database(nutrient_id: Id<Nutrient>, pool: &Pool<Sqlite>) -> Result<Vec<Self>, sqlx::Error> {
        let id = nutrient_id.get_inner().to_bytes().to_vec();

        Ok(sqlx::query_as!(
            NutrientConversionRecord,
            r#"
                    SELECT
                        nutrient_id,
                        unit_id,
                        factor
                    FROM nutrients_unit_conversions
                    WHERE
                        nutrient_id = ?
                "#,
            id
        )
        .fetch_all(pool)
        .await?)
    }

    pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO nutrients_unit_conversions (nutrient_id, unit_id, factor)
                VALUES (?, ?, ?)
                ON CONFLICT (nutrient_id, unit_id)
                DO UPDATE SET
                    nutrient_id = excluded.nutrient_id,
                    unit_id = excluded.unit_id,
                    factor = excluded.factor
            "#,
            self.nutrient_id,
            self.unit_id,
            self.factor
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn save_vec_to_database(items: &Vec<Self>, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        for item in items {
            sqlx::query!(
                r#"
                    INSERT INTO nutrients_unit_conversions (nutrient_id, unit_id, factor)
                    VALUES (?, ?, ?)
                    ON CONFLICT (nutrient_id, unit_id)
                    DO UPDATE SET
                        unit_id = excluded.unit_id,
                        factor = excluded.factor
                "#,
                item.nutrient_id,
                item.unit_id,
                item.factor
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn delete_conversion_from_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                DELETE FROM nutrients_unit_conversions 
                WHERE
                    nutrient_id = ?
                    AND unit_id = ?
            "#,
            self.nutrient_id,
            self.unit_id,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete_all_conversions_from_database(nutrient_id: Id<Nutrient>, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let id = nutrient_id.get_inner().to_bytes().to_vec();

        sqlx::query!(
            r#"
                DELETE FROM nutrients_unit_conversions 
                WHERE
                    nutrient_id = ?
            "#,
            id,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete_conversion_vec_from_database(items: Vec<&Self>, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        for item in items {
            sqlx::query!(
                r#"
                DELETE FROM nutrients_unit_conversions 
                WHERE
                nutrient_id = ?
                    AND unit_id = ?
                    "#,
                item.nutrient_id,
                item.unit_id,
            )
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct NutrientLinkRecord {
    pub nutrient_id: Vec<u8>,
    pub parent_ids: Vec<Vec<u8>>,
    pub child_ids: Vec<Vec<u8>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NutrientLinkHashes {
    pub nutrient_map: HashMap<Vec<u8>, String>,
    pub parent_map: HashMap<Vec<u8>, String>,
    pub child_map: HashMap<Vec<u8>, String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NutrientLinkNames {
    pub parent_names: Vec<String>,
    pub child_names: Vec<String>,
}

impl NutrientLinkRecord {
    pub fn from_values(nutrient_id: Vec<u8>, parent_ids: Vec<Vec<u8>>, child_ids: Vec<Vec<u8>>) -> Self {
        Self {
            nutrient_id,
            parent_ids,
            child_ids,
        }
    }

    pub async fn from_nutrient(nutrient: Nutrient, pool: &Pool<Sqlite>) -> Result<(Self, NutrientLinkHashes), &'static str> {
        let nutrient_id = Id::<Nutrient>::new(InnerIdType::Uuid).to_bytes().to_vec();
        let nutrient_name = nutrient.get_name();
        let nutrient_map = HashMap::from([(nutrient_id.clone(), nutrient_name)]);

        let mut child_ids: Vec<Vec<u8>> = Vec::new();
        let mut child_names: Vec<String> = Vec::new();
        let mut child_map: HashMap<Vec<u8>, String> = HashMap::new();
        for child in nutrient.get_children() {
            let child_name = child.borrow().clone().get_name();
            let child_id = NutrientRecord::load_from_database_using_name(child.borrow().get_name(), pool).await.unwrap().nutrient_id;
            child_names.push(child_name.clone());
            child_ids.push(child_id.clone());
            child_map.insert(child_id, child_name);
        }

        let mut parent_ids: Vec<Vec<u8>> = Vec::new();
        let mut parent_names: Vec<String> = Vec::new();
        let mut parent_map: HashMap<Vec<u8>, String> = HashMap::new();
        for parent_weak in nutrient.get_parents() {
            if let Some(parent) = parent_weak.upgrade() {
                let parent_name = parent.borrow().clone().get_name();
                let parent_id = NutrientRecord::load_from_database_using_name(parent.borrow().get_name(), pool).await.unwrap().nutrient_id;
                parent_names.push(parent_name.clone());
                parent_ids.push(parent_id.clone());
                parent_map.insert(parent_id, parent_name);
            } else {
                return Err("Weak parent");
            }
        }

 
        let nutrient_link_record = NutrientLinkRecord {
            nutrient_id,
            parent_ids,
            child_ids,
        };
        let nutrient_link_names = NutrientLinkHashes {
            nutrient_map,
            parent_map,
            child_map,
        };

        Ok((nutrient_link_record, nutrient_link_names))
    }

    pub async fn from_nutrient_entity(nutrient_entity: Entity<Nutrient>, pool: &Pool<Sqlite>) -> Result<(Self, NutrientLinkHashes), &'static str> {
        let nutrient_id = nutrient_entity.get_id().get_inner().to_bytes().to_vec();
        let nutrient = nutrient_entity.get_inner();
        let nutrient_name = nutrient.get_name();
        let nutrient_map = HashMap::from([(nutrient_id.clone(), nutrient_name)]);

        let mut child_ids: Vec<Vec<u8>> = Vec::new();
        let mut child_names: Vec<String> = Vec::new();
        let mut child_map: HashMap<Vec<u8>, String> = HashMap::new();
        for child in nutrient.get_children() {
            let child_name = child.borrow().clone().get_name();
            let child_id = NutrientRecord::from_nutrient(child.borrow().clone(), pool)
                .await
                .unwrap()
                .nutrient_id;
            child_names.push(child_name.clone());
            child_ids.push(child_id.clone());
            child_map.insert(child_id, child_name);
        }

        let mut parent_ids: Vec<Vec<u8>> = Vec::new();
        let mut parent_names: Vec<String> = Vec::new();
        let mut parent_map: HashMap<Vec<u8>, String> = HashMap::new();
        for parent_weak in nutrient.get_parents() {
            if let Some(parent) = parent_weak.upgrade() {
                let parent_name = parent.borrow().clone().get_name();
                let parent_id = NutrientRecord::from_nutrient(parent.borrow().clone(), pool)
                    .await
                    .unwrap()
                    .nutrient_id;
                parent_names.push(parent_name.clone());
                parent_ids.push(parent_id.clone());
                parent_map.insert(parent_id, parent_name);
            } else {
                return Err("Weak parent");
            }
        }

        let nutrient_link_record = NutrientLinkRecord {
            nutrient_id,
            parent_ids,
            child_ids,
        };
        let nutrient_link_names = NutrientLinkHashes {
            nutrient_map,
            parent_map,
            child_map,
        };

        Ok((nutrient_link_record, nutrient_link_names))
    }

    pub fn update_nutrient_link_ids(&self, nutrient_link_hashes: &NutrientLinkHashes, nutrient_name_new_id_map: &HashMap<String, Vec<u8>>) -> NutrientLinkRecord {
        let mut new_nutrient_link_record = self.clone();

        for id in &mut new_nutrient_link_record.child_ids {
            if let Some(name) = nutrient_link_hashes.child_map.get(id) {
                if let Some(new_id) = nutrient_name_new_id_map.get(name) {
                    *id = new_id.clone();
                }
            }
        }

        for id in &mut new_nutrient_link_record.parent_ids {
            if let Some(name) = nutrient_link_hashes.parent_map.get(id) {
                if let Some(new_id) = nutrient_name_new_id_map.get(name) {
                    *id = new_id.clone();
                }
            }
        }

        return new_nutrient_link_record
    }

    pub fn sort(&mut self) {
        self.child_ids.sort();
        self.parent_ids.sort();
    }

    pub async fn get_nutrient_link_names(&self, pool: &Pool<Sqlite>) -> Result<NutrientLinkNames, sqlx::Error> {
        let mut parent_names = Vec::new();
        let mut child_names = Vec::new();

        for parent_id in &self.parent_ids {
            let parent_record = NutrientRecord::load_from_database_using_id(Id::from_bytes(InnerIdType::Uuid, parent_id.clone().try_into().unwrap()), pool).await?;
            let parent = parent_record.to_nutrient(&pool).await?;
            parent_names.push(parent.get_name());
        }

        for child_id in &self.child_ids {
            println!("child id: {:#?}", child_id);
            let child_record = NutrientRecord::load_from_database_using_id(Id::from_bytes(InnerIdType::Uuid, child_id.clone().try_into().unwrap()), pool).await?;
            let child = child_record.to_nutrient(&pool).await?;
            child_names.push(child.get_name());
        }

        let nutrient_links = NutrientLinkNames {
            parent_names,
            child_names,
        };

        Ok(nutrient_links)
    }

    pub async fn to_hydrated_nutrient(&self, nutrient_list: NutrientList, pool: &Pool<Sqlite>) -> Result<Rc<RefCell<Nutrient>>, sqlx::Error> {
        let nutrient_record = NutrientRecord::load_from_database_using_id(Id::from_inner(InnerId::Uuid(Uuid::from_slice(&self.nutrient_id).unwrap())), pool).await?;

        let nutrient_name = nutrient_record.to_nutrient(pool).await.unwrap().get_name();
        let nutrients: Vec<Rc<RefCell<Nutrient>>> = nutrient_list
            .get_nutrients()
            .iter()
            .filter(|nutrient| nutrient.borrow().get_name() == nutrient_name)
            .map(|nutrient| nutrient.clone())
            .collect();

        if nutrients.len() == 0 { panic!("No name found") }
        let nutrient: Rc<RefCell<Nutrient>> = match nutrients.first() {
            Some(n) => n.clone(),
            None => panic!("Nutrient is empty"),
        };
        if nutrients.iter().all(|n| Rc::ptr_eq(n, &nutrient)) == false { panic!("Different nutrient definitions found") };

        for parent_id in &self.parent_ids {
            let parent_record = NutrientRecord::load_from_database_using_id(Id::from_bytes(InnerIdType::Uuid, parent_id.clone().try_into().unwrap()), pool).await?;
            let parent_name = parent_record.to_nutrient(pool).await?.get_name();
            let parents: Vec<Rc<RefCell<Nutrient>>> = nutrient_list
                .get_nutrients()
                .iter()
                .filter(|nutrient| nutrient.borrow().get_name() == parent_name)
                .map(|nutrient| nutrient.clone())
                .collect();

            if parents.len() == 0  { panic!("No name found") }
            let parent: Rc<RefCell<Nutrient>> = match parents.first() {
                Some(p) => p.clone(),
                None => panic!("Parent is empty"),
            };
            if parents.iter().all(|p| Rc::ptr_eq(p, &parent)) == false { panic!("Different nutrient definitions found") };

            link_parent_child(&parent, &nutrient).unwrap();
        }

        for child_id in &self.child_ids {
            let child_record = NutrientRecord::load_from_database_using_id(Id::from_bytes(InnerIdType::Uuid, child_id.clone().try_into().unwrap()), pool).await?;
            let child_name = child_record.to_nutrient(pool).await?.get_name();
            let children: Vec<Rc<RefCell<Nutrient>>> = nutrient_list
                .get_nutrients()
                .iter()
                .filter(|nutrient| nutrient.borrow().get_name() == child_name)
                .map(|nutrient| nutrient.clone())
                .collect();

            if children.len() == 0  { panic!("No name found") }
            let child: Rc<RefCell<Nutrient>> = match children.first() {
                Some(c) => c.clone(),
                None => panic!("Child is empty"),
            };
            if children.iter().all(|c| Rc::ptr_eq(c, &child)) == false { panic!("Different nutrient definitions found") };

            link_parent_child(&nutrient, &child).unwrap();
        }
        
        return Ok(nutrient);
    }

    pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        for child_id in &self.child_ids {
            sqlx::query!(
                r#"
                    INSERT INTO nutrients_nutrient_relationships (parent_id, child_id)
                    VALUES (?, ?)
                    ON CONFLICT (parent_id, child_id)
                    DO NOTHING
                "#,
                self.nutrient_id,
                child_id,
            )
            .execute(&mut *tx)
            .await?;
        }

        for parent_id in &self.parent_ids {
            sqlx::query!(
                r#"
                    INSERT INTO nutrients_nutrient_relationships (parent_id, child_id)
                    VALUES (?, ?)
                    ON CONFLICT (parent_id, child_id)
                    DO NOTHING
                "#,
                parent_id,
                self.nutrient_id,
            )
            .execute(&mut *tx)
            .await?;
        }
        let _ = tx.commit().await;

        Ok(())
    }

    pub async fn load_from_sqlite(nutrient_id: Uuid, pool: &Pool<Sqlite>) -> Result<Self, sqlx::Error> {
        let parent_rows = sqlx::query!(
            r#"
                SELECT
                    parent_id
                FROM nutrients_nutrient_relationships
                WHERE
                    child_id = ?
            "#,
            nutrient_id
        )
        .fetch_all(pool)
        .await?;
        let parent_ids: Vec<Vec<u8>> = parent_rows
            .iter()
            .map(|row| row.parent_id.clone())
            .collect();

        let child_rows = sqlx::query!(
            r#"
                SELECT
                    child_id
                FROM nutrients_nutrient_relationships
                WHERE
                    parent_id = ?
            "#,
            nutrient_id
        )
        .fetch_all(pool)
        .await?;
        let child_ids: Vec<Vec<u8>> = child_rows.iter().map(|row| row.child_id.clone()).collect();

        let nutrient_link_record = Self {
            nutrient_id: nutrient_id.as_bytes().to_vec(),
            parent_ids,
            child_ids,
        };
        Ok(nutrient_link_record)
    }

    pub async fn delete_nutrient_link(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        sqlx::query!(
            r#"
                DELETE FROM nutrients_nutrient_relationships
                WHERE
                    parent_id = ?
            "#,
            self.nutrient_id,
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
                DELETE FROM nutrients_nutrient_relationships
                WHERE
                    child_id = ?
            "#,
            self.nutrient_id,
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

}

pub struct NutrientLinkRecordUuid {
    pub nutrient_id: Uuid,
    pub parent_id_vec: Vec<Uuid>,
    pub child_id_vec: Vec<Uuid>,
}
