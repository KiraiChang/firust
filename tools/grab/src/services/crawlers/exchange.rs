use scraper::{Html, Selector};
use crate::services::models;
use crate::services::models::enums::ExchangeProps;

pub async fn grab(begin:&str, end:&str) -> Result<Vec::<models::exchange::Exchange>, Box<dyn std::error::Error>> {
    let params = [("queryStartDate", &begin), ("queryEndDate", &end)];
    let client = reqwest::Client::new();
    let resp = client.post("https://www.taifex.com.tw/cht/3/dailyFXRate")
        .header("content-type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?
        .text()
        .await?;
    let document = Html::parse_document(&resp);
    let tr = Selector::parse(".table_c tbody tr").unwrap();
    let td = Selector::parse("td").unwrap();
    let mut result:Vec::<models::exchange::Exchange> = Vec::new();
    for element in document.select(&tr) {
        //println!("{element:#?}");
        let items:Vec<String> = element.select(&td).map(|element| element.inner_html()).collect();
        let mut val = models::exchange::Exchange {
            date: String::from(&items[0]),
            data: Vec::new(),
        } ;
        for x in 1..10 {
            let en =  ExchangeProps::from_usize(x);
            if !en.is_none() {
                val.data.push(models::exchange::ExchangeItem{
                    id:en.unwrap().value().to_string(),
                    value: items[x].parse::<f64>().unwrap(),
                });
            }
        }
        result.push(val);
    }

    Ok(result)
}