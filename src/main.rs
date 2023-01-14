use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short = 'n', long)]
    name: String,
    #[clap(short = 's', long, help = "%m/%d/%Y")]
    start_date: String,
    #[clap(short = 'e', long, help = "%m/%d/%Y")]
    end_date: String,
    #[clap(short = 'i', long, default_value = "Daily")]
    interval: String,
    #[clap(short = 'c', long, default_value = "date")]
    sort_col: String,
    #[clap(short = 'o', long, default_value = "DESC")]
    sort_ord: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let id = investing_api::fetch_id_by_name(&args.name).await?;
    // let id = dbg!(id);

    let data_item_vec = investing_api::fetch_historical_data(
        &id,
        &args.start_date,
        &args.end_date,
        &args.interval,
        &args.sort_col,
        &args.sort_ord,
    )
    .await?;
    println!("{data_item_vec:#?}");

    let filename = format!("{}.csv", args.name);
    investing_api::write_to_csv(&data_item_vec, &filename).await?;

    Ok(())
}
