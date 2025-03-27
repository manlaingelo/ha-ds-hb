use scraper::{Html, Selector};
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(serde::Serialize)]
struct AimagData {
    aimag_code: String,
    aimag_name_mn: String,
    aimag_name_en: String,
}

#[derive(serde::Serialize)]
struct BagKhorooData {
    bag_code: String,
    bag_name_mn: String,
    bag_name_en: String,
    aimag_code: String,
    duureg_code: String,
}

#[derive(serde::Serialize)]
struct SumDuuregData {
    duureg_code: String,
    duureg_name: String,
    aimag_name_mn: String,
}

fn extract_and_save<T: serde::Serialize>(
    file_path: &str,
    selector_str: &str,
    data_mapper: fn(&scraper::ElementRef) -> Option<T>,
    output_file_name: &str,
) -> Result<(), Box<dyn Error>> {
    let html_content = fs::read_to_string(file_path)?;
    let document = Html::parse_document(&html_content);
    let selector = Selector::parse(selector_str).unwrap();

    let json_data: Vec<T> = document
        .select(&selector)
        .filter_map(|element| data_mapper(&element))
        .collect();

    let output_dir = Path::new("out");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }

    let json_string = serde_json::to_string_pretty(&json_data)?;
    fs::write(format!("out/{}.json", output_file_name), json_string)?;

    println!("JSON data saved to {}.json", output_file_name);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let aimag_data_mapper = |element: &scraper::ElementRef| {
        let cells = element
            .select(&Selector::parse("td").unwrap())
            .collect::<Vec<_>>();
        if cells.len() >= 3 {
            Some(AimagData {
                aimag_code: cells[0].text().collect::<String>().trim().to_string(),
                aimag_name_mn: cells[1].text().collect::<String>().trim().to_string(),
                aimag_name_en: cells[2].text().collect::<String>().trim().to_string(),
            })
        } else {
            None
        }
    };

    let bag_khoroo_data_mapper = |element: &scraper::ElementRef| {
        let cells = element
            .select(&Selector::parse("td").unwrap())
            .collect::<Vec<_>>();
        if cells.len() >= 5 {
            Some(BagKhorooData {
                bag_code: cells[0].text().collect::<String>().trim().to_string(),
                bag_name_mn: cells[1].text().collect::<String>().trim().to_string(),
                bag_name_en: cells[4].text().collect::<String>().trim().to_string(),
                aimag_code: cells[2].text().collect::<String>().trim().to_string(),
                duureg_code: cells[3].text().collect::<String>().trim().to_string(),
            })
        } else {
            None
        }
    };

    let sum_duureg_data_mapper = |element: &scraper::ElementRef| {
        let cells = element
            .select(&Selector::parse("td").unwrap())
            .collect::<Vec<_>>();
        if cells.len() >= 4 {
            Some(SumDuuregData {
                duureg_code: cells[1].text().collect::<String>().trim().to_string(),
                duureg_name: cells[2].text().collect::<String>().trim().to_string(),
                aimag_name_mn: cells[3].text().collect::<String>().trim().to_string(),
            })
        } else {
            None
        }
    };

    extract_and_save("html/aimag.html", "tr", aimag_data_mapper, "aimag")?;
    extract_and_save(
        "html/bag-khoroo.html",
        "tr",
        bag_khoroo_data_mapper,
        "bag-khoroo",
    )?;
    extract_and_save(
        "html/sum-duureg.html",
        "tr",
        sum_duureg_data_mapper,
        "sum-duureg",
    )?;

    Ok(())
}
