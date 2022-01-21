use anyhow::Result;

#[derive(Debug)]
pub struct DataItem {
    timestamp_sec: i64,
    price: f32,
    open: f32,
    high: f32,
    low: f32,
    vol: i64,
}

pub async fn fetch_historical_data(
    id: &str,
    start_date: &str,
    end_date: &str,
    interval: &str,
    sort_col: &str,
    sort_ord: &str,
) -> Result<Vec<DataItem>> {
    let client = reqwest::Client::builder()
        .user_agent("Chrome/97")
        .build()
        .unwrap();
    // let client = dbg!(client);

    let form_data = [
        ("curr_id", id),
        ("st_date", start_date),
        ("end_date", end_date),
        ("interval_sec", interval),
        ("sort_col", sort_col),
        ("sort_ord", sort_ord),
        ("action", "historical_data"),
    ];
    // let form_data = dbg!(form_data);

    let response = client
        .post("https://www.investing.com/instruments/HistoricalDataAjax")
        .header("x-requested-with", "XMLHttpRequest")
        .form(&form_data)
        .send()
        .await?;
    // let response = dbg!(response);

    let response_text = response.text().await?;
    // let response_text = dbg!(response_text);

    let document = scraper::Html::parse_fragment(&response_text);
    // let document = dbg!(document);

    let tr_selector = scraper::Selector::parse("#curr_table tbody tr").unwrap();
    // let tr_selector = dbg!(tr_selector);

    let tr_select = document.select(&tr_selector);
    // let tr_select = dbg!(tr_select);

    let td_selector = scraper::Selector::parse("td").unwrap();
    // let td_selector = dbg!(td_selector);

    let mut data_item_vec = vec![];
    for tr in tr_select {
        let info = tr
            .select(&td_selector)
            .into_iter()
            .filter_map(|td| td.value().attr("data-real-value"))
            .collect::<Vec<&str>>();
        // let info = dbg!(info);

        let timestamp_sec = info.get(0).unwrap();
        let timestamp_sec = timestamp_sec.to_string().parse::<i64>().unwrap();
        // let timestamp_sec = chrono::NaiveDateTime::from_timestamp(timestamp_sec, 0);
        // let timestamp_sec: chrono::DateTime<chrono::Utc> =
        //     chrono::DateTime::from_utc(timestamp_sec, chrono::Utc);
        // let timestamp_sec = timestamp_sec.format("%Y-%m-%d %H:%M:%S");
        // let timestamp_sec = timestamp_sec.to_string();
        // let timestamp_sec = dbg!(timestamp_sec);

        let price = info.get(1).unwrap();
        let price = price.replace(",", "").parse::<f32>().unwrap();
        // let price = dbg!(price);

        let open = info.get(2).unwrap();
        let open = open.replace(",", "").parse::<f32>().unwrap();
        // let open = dbg!(open);

        let high = info.get(3).unwrap();
        let high = high.replace(",", "").parse::<f32>().unwrap();
        // let high = dbg!(high);

        let low = info.get(4).unwrap();
        let low = low.replace(",", "").parse::<f32>().unwrap();
        // let low = dbg!(low);

        let vol = info.get(5).unwrap();
        let vol = vol.replace(",", "").parse::<i64>().unwrap();
        // let vol = dbg!(vol);

        let data_item = DataItem {
            timestamp_sec,
            price,
            open,
            high,
            low,
            vol,
        };
        // let data_item = dbg!(data_item);

        data_item_vec.push(data_item);
    }

    Ok(data_item_vec)
}

pub async fn write_to_csv(data_item_vec: Vec<DataItem>, filename: &str) -> Result<()> {
    println!("write to csv...");

    let mut csv_writer = csv::Writer::from_path(filename)?;

    csv_writer.write_record(&["timestamp_sec", "price", "open", "high", "low", "vol"])?;

    for data_item in data_item_vec.iter() {
        csv_writer.write_record(&[
            data_item.timestamp_sec.to_string(),
            data_item.price.to_string(),
            data_item.open.to_string(),
            data_item.high.to_string(),
            data_item.low.to_string(),
            data_item.vol.to_string(),
        ])?;
    }

    csv_writer.flush()?;

    Ok(())
}

#[tokio::test]
async fn run_fetch_historical_data() {
    let data = fetch_historical_data(
        "1171911",
        "01/01/2022",
        "01/20/2022",
        "Daily",
        "date",
        "DESC",
    )
    .await
    .unwrap();
    assert_eq!(data.len(), 13);
}
