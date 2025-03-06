const fs = require("fs");
const cheerio = require("cheerio");

// Read the HTML file
const html = fs.readFileSync("sum-duureg.html", "utf-8");

// Load the HTML with cheerio
const $ = cheerio.load(html);

// Extract data
const jsonData = [];
$("tr").each((index, element) => {
  const cells = $(element).find("td");

  if (cells.length > 0) {
    jsonData.push({
      district_code: $(cells[1]).text().trim(),
      district_name: $(cells[2]).text().trim(),
      aimag_name_mn: $(cells[3]).text().trim(),
    });
  }
});

// Convert to JSON and save to file
fs.writeFileSync("sum-duureg.json", JSON.stringify(jsonData, null, 2), "utf-8");

console.log("JSON data saved to sum-duureg.json");
