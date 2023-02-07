fn main() {
    //     println!("Record = {:?}", save_status_std("Dennis"));
    // println!("Record = {:?}", save_status_concise("Dennis"));
    println!("Record = {:?}", save_status_try("Dennis"));
}

struct StatusRecord {
    id: i64,
    text: String,
    created_at: std::time::Instant,
}

fn save_to_database(text: &str) -> Result<StatusRecord, &'static str> {
    // fake implementation that always fails
    Err("database unavailiable")
}

fn save_status_std(text: &str) -> Result<i64, &'static str> {
    if text.len() > 200 {
        return Err("status text is too long");
    }

    let record = match save_to_database(text) {
        Ok(rec) => rec,
        Err(e) => return Err(e),
    };

    Ok(record.id)
}

fn save_status_concise(text: &str) -> Result<i64, &'static str> {
    if text.len() > 200 {
        return Err("status text is too long");
    }

    let record = save_to_database(text)?;
    // let record = match save_to_database(text) {
    //     Ok(rec) => rec,
    //     Err(e) => return Err(e),
    // };

    Ok(record.id)
}

fn save_status_try(text: &str) -> Result<i64, &'static str> {
    if text.len() > 200 {
        return Err("status text is too long");
    }

    let record = r#try!(save_to_database(text));
    // let record = save_to_database(text)?;
    // let record = match save_to_database(text) {
    //     Ok(rec) => rec,
    //     Err(e) => return Err(e),
    // };

    Ok(record.id)
}
