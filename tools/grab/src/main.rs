use scraper::{Html, Selector};

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
    for element in document.select(&tr) {
        println!("{element:#?}");
        let items:Vec<String> = element.select(&td).map(|element| element.inner_html()).collect();
        for x in 0..10 {
            println!("{}", items[x]); // x: i32
        }
        // for item in element.select(&td){
        //     let text = item.inner_html();
        //     println!("{text:#?}");
        // }
    }


    Ok(())
}