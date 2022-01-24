use anyhow::Result;

#[derive(Debug)]
pub struct DataItem {
    timestamp_sec: i64,
    date_str: String,
    date_y: i32,
    date_m: i32,
    date_d: i32,
    price: f32,
    open: f32,
    high: f32,
    low: f32,
    vol: i64,
}

fn get_chrome97_client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent("Chrome/97")
        .build()
        .unwrap()
}

pub async fn fetch_id_by_name(name: &str) -> Result<String> {
    let client = get_chrome97_client();
    // let client = dbg!(client);

    let url = format!("https://www.investing.com/indices/{}", name);
    // let url = dbg!(url);

    let response = client.get(url).send().await?;
    // let response = dbg!(response);

    let response_text = response.text().await?;
    // let response_text = dbg!(response_text);

    let id = response_text
        .split_once("setTargeting(\"pair_id\", \"")
        .unwrap()
        .1
        .split_once("\");googletag.pubads()")
        .unwrap()
        .0;
    // let id = dbg!(id);

    Ok(id.to_string())
}

pub async fn fetch_historical_data(
    id: &str,
    start_date: &str,
    end_date: &str,
    interval: &str,
    sort_col: &str,
    sort_ord: &str,
) -> Result<Vec<DataItem>> {
    let client = get_chrome97_client();
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
        // let timestamp_sec = dbg!(timestamp_sec);

        let naive_date_time = chrono::NaiveDateTime::from_timestamp(timestamp_sec, 0);
        // let naive_date_time = dbg!(naive_date_time);

        let date_str = naive_date_time.format("%Y/%m/%d").to_string();
        // let date_str = dbg!(date_str);

        let date_y = naive_date_time.format("%Y").to_string().parse().unwrap();
        // let date_y = dbg!(date_y);

        let date_m = naive_date_time.format("%m").to_string().parse().unwrap();
        // let date_m = dbg!(date_m);

        let date_d = naive_date_time.format("%d").to_string().parse().unwrap();
        // let date_d = dbg!(date_d);

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
            date_str,
            date_y,
            date_m,
            date_d,
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
    println!("write to {} ...", filename);

    let mut csv_writer = csv::Writer::from_path(filename)?;

    csv_writer.write_record(&[
        "timestamp_sec",
        "date_str",
        "date_y",
        "date_m",
        "date_d",
        "price",
        "open",
        "high",
        "low",
        "vol",
    ])?;

    for data_item in data_item_vec.iter() {
        csv_writer.write_record(&[
            data_item.timestamp_sec.to_string(),
            data_item.date_str.to_string(),
            data_item.date_y.to_string(),
            data_item.date_m.to_string(),
            data_item.date_d.to_string(),
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
async fn test_fetch_id_by_name() {
    let id = fetch_id_by_name("csi1000").await.unwrap();
    assert_eq!(id, "1171911");
}

#[tokio::test]
async fn test_fetch_historical_data() {
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
