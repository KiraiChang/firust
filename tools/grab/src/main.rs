mod models;

use scraper::{Html, Selector};
use crate::models::enums::ExchangeProps;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let params = [("queryStartDate", "2024/08/01"), ("queryEndDate", "2024/10/30")];
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
            if let en =  ExchangeProps::from_usize(x){
                val.data.push(models::exchange::ExchangeItem{
                    id:en.unwrap().value().to_string(),
                    value: items[x].parse::<f64>().unwrap(),
                });
            }
            // println!("{}", items[x]); // x: i32
        }
        result.push(val);
        // for item in element.select(&td){
        //     let text = item.inner_html();
        //     println!("{text:#?}");
        // }
    }
    println!("{:?}", result); // x: i32


    Ok(())
}