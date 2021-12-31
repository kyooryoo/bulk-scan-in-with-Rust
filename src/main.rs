use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

#[tokio::main]
 async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_args();

    let apikey = format!("apikey {}", args.apikey);
    let base_url = String::from("https://api-cn.hosted.exlibrisgroup.com.cn/almaws/v1");
    let barcode_url = String::from("/items?item_barcode=");
    let client = reqwest::Client::builder().build()?;

    let file = File::open(PathBuf::from(args.barcodes)).unwrap();
    let reader = BufReader::new(file);
    for (index, barcode) in reader.lines().enumerate() {
        let barcode = String::from(barcode.unwrap());
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
        let item_url = format!("{}/bibs/{}/holdings/{}/items/{}", base_url, mms_id, holding_id, item_id);
    
        let param_op = String::from("op=SCAN");
        let param_lib = String::from("library=LYNN");
        let param_circ = String::from("circ_desk=DEFAULT_CIRC_DESK");
        let param_house = String::from("register_in_house_use=false");
        let item_api = format!("{}?{}&{}&{}&{}", item_url, param_op, param_lib, param_circ, param_house);
    
        let res = client
        .post(item_api)
        .header("accept", "application/json")
        .header("authorization", &apikey)
        .send()
        .await?;
        let post_status = res.status().is_success();
    
        let t = res
        .text()
        .await?;
        let js = json::parse(&t).unwrap();
        let item_in_place = js["item_data"]["base_status"]["value"] == "1";
        
        if  post_status && item_in_place {
            println!("No.{} processed: {}", index + 1, barcode);
        } else {
            println!("Check the item {}!!!", barcode);
        }
    }

    Ok(())
}

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(short = "k", long = "apikey")]
    apikey: String,

    #[structopt(short = "f", long = "filepath")]
    barcodes: PathBuf,
}