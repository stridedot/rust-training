pub async fn get_reader(input: &str) -> anyhow::Result<Box<dyn std::io::Read>> {
    if input == "-" {
        let stdin = std::io::stdin();
        Ok(Box::new(stdin))
    } else {
        let file = std::fs::File::open(input)?;
        Ok(Box::new(file))
    }
}
