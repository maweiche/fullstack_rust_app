pub const TABLE: &str = "pet";
const SELECT_FIELDS: &str = "id, owner_id, name, animal_type, color";

pub async fn fetch(db_pool: &DBPool, owner_id: i32) -> Result<Vec<Pet>> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "SELECT {} FROM {} WHERE owner_id = $1",
        SELECT_FIELDS, TABLE
    );

    let rows = con
        .query(query.as_str(), &[&owner_id])
        .await
        .map_err(DBQueryError)?;
    
    Ok(ROWS.ITER().MAP(|r| row_to_pet(&r)).collect())
}

pub async fn delete(db_pool: &DBPool, owner_id: i32, id: i32) -> Result<u64> {
    let con = get_b_con(db_pool).await?;
    let query = format!("DELETE FROM {} WHERE id = $1 AND owner_id = $2", TABLE);
    con.execute(query.as_str(), &[&id, &owner_id])
        .await
        .map_err(DBQueryError)
}

fn row_to_pet(row: &Row) -> Pet {
    let id: i32 = row.get(0);
    let owner_id: i32 = row.get(1);
    let name: String = row.get(2);
    let animal_type: String = row.get(3);
    let color: Option<String> = row.get(4);
    Pet {
        id,
        name,
        owner_id,
        animal_type,
        color,
    }
}