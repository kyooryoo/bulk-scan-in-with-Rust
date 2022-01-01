#![allow(dead_code)]
#![allow(unused_variables)]
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_args();
    let apikey = format!("apikey {}", args.apikey);
    let barcode = args.barcode;
    let param_op = String::from("op=SCAN");
    let param_lib = format!("library={}", args.library);
    let param_circ = format!("circ_desk={}", args.circdesk);
    let param_house = format!("register_in_house_use={}", args.inhouseuse);

    let base_url = String::from("https://api-cn.hosted.exlibrisgroup.com.cn/almaws/v1");
    let barcode_url = String::from("/items?item_barcode=");

    if barcode != "NOSUCHCODE" {
        let client = reqwest::Client::builder().build()?;
        let barcode_api = format!("{}{}{}", base_url, barcode_url, barcode);
        let res = client
            .get(barcode_api)
            .header("accept", "application/json")
            .header("authorization", &apikey)
            .send()
            .await?
            .text()
            .await?;
        let js = json::parse(&res).unwrap();
        let mms_id = &js["bib_data"]["mms_id"];
        let holding_id = &js["holding_data"]["holding_id"];
        let item_id = &js["item_data"]["pid"];
        let item_url = format!(
            "{}/bibs/{}/holdings/{}/items/{}",
            base_url, mms_id, holding_id, item_id
        );
        let item_api = format!(
            "{}?{}&{}&{}&{}",
            item_url, param_op, param_lib, param_circ, param_house
        );
        let res = client
            .post(item_api)
            .header("accept", "application/json")
            .header("authorization", &apikey)
            .send()
            .await?;
        let post_status = res.status().is_success();

        let t = res.text().await?;
        let js = json::parse(&t).unwrap();
        let item_in_place = js["item_data"]["base_status"]["value"] == "1";

        if post_status && item_in_place {
            println!("Item {} is processed!", barcode);
        } else {
            println!("Check the item {}!!!", barcode);
        }
    } else {
        let file = File::open(PathBuf::from(args.barcodes)).unwrap();
        let reader = BufReader::new(file);
        for (index, barcode) in reader.lines().enumerate() {
            let barcode = String::from(barcode.unwrap());
            let client = reqwest::Client::builder().build()?;
            let barcode_api = format!("{}{}{}", base_url, barcode_url, barcode);
            let res = client
                .get(barcode_api)
                .header("accept", "application/json")
                .header("authorization", &apikey)
                .send()
                .await?
                .text()
                .await?;
            let js = json::parse(&res).unwrap();
            let mms_id = &js["bib_data"]["mms_id"];
            let holding_id = &js["holding_data"]["holding_id"];
            let item_id = &js["item_data"]["pid"];
            let item_url = format!(
                "{}/bibs/{}/holdings/{}/items/{}",
                base_url, mms_id, holding_id, item_id
            );
            let item_api = format!(
                "{}?{}&{}&{}&{}",
                item_url, param_op, param_lib, param_circ, param_house
            );
            let res = client
                .post(item_api)
                .header("accept", "application/json")
                .header("authorization", &apikey)
                .send()
                .await?;
            let post_status = res.status().is_success();

            let t = res.text().await?;
            let js = json::parse(&t).unwrap();
            let item_in_place = js["item_data"]["base_status"]["value"] == "1";

            if post_status && item_in_place {
                println!("No.{} item processed: {}", index + 1, barcode);
            } else {
                println!("Check the item {}!!!", barcode);
            }
        }
    }

    Ok(())
}

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(short = "k", long = "apikey", default_value = "NOSUCHKEY")]
    apikey: String,

    #[structopt(short = "f", long = "filepath", default_value = "./barcodes.txt")]
    barcodes: PathBuf,

    #[structopt(short = "l", long = "library", default_value = "MAIN")]
    library: String,

    #[structopt(short = "c", long = "circdesk", default_value = "DEFAULT_CIRC_DESK")]
    circdesk: String,

    #[structopt(short = "b", long = "barcode", default_value = "NOSUCHCODE")]
    barcode: String,

    #[structopt(short = "i", long = "inhouseuse", default_value = "false")]
    inhouseuse: String,
}
