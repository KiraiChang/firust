#[derive(Debug)]
pub struct ExchangeItem {
    pub id: String,
    pub value: f64,
}

#[derive(Debug)]
pub struct Exchange{
    pub date : String,
    pub data: Vec<ExchangeItem>
}
