const fs = require("fs");
const cheerio = require("cheerio");

// Read the HTML file
const html = fs.readFileSync("aimag.html", "utf-8");

// Load the HTML with cheerio
const $ = cheerio.load(html);

// Extract data
const jsonData = [];
$("tr").each((index, element) => {
  const cells = $(element).find("td");
  
  if (cells.length > 0) {
    jsonData.push({
      aimag_code: $(cells[0]).text().trim(),
      aimag_name_mn: $(cells[1]).text().trim(),
      aimag_name_en: $(cells[2]).text().trim(),
    });
  }
});

// Convert to JSON and save to file
fs.writeFileSync("json/aimag.json", JSON.stringify(jsonData, null, 2), "utf-8");

console.log("JSON data saved to aimag.json");
