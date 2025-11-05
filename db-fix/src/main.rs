fn main() -> Result<(), Box<dyn std::error::Error>> {
    let old = old_sled::open("../topper.db").unwrap();
    let new = sled::open("../topper.db-fixed").unwrap();

    let export = old.export();
    new.import(export);

    assert_eq!(old.checksum()?, new.checksum()?);
    Ok(())
}
