use super::error::BoursoScrapeError;
use log::warn;

pub async fn scrape_isin(isin: &str) -> Result<String, BoursoScrapeError> {
    let url = format!("https://www.boursorama.com/recherche/{}?searchId=", isin);
    let response = reqwest::get(url).await?.text().await?;
    let document = scraper::Html::parse_document(&response);
    let quote_selector = match scraper::Selector::parse("#main-content > div > section.l-quotepage > header > div > div > div.c-faceplate__company > div.c-faceplate__info > div > div.c-faceplate__price.c-faceplate__price--inline > span.c-instrument.c-instrument--last") {
        Ok(elem) => elem,
        Err(_) => { warn!("Element not found");
        return Err(BoursoScrapeError::ParseError);},
    };
    let mut quote = document.select(&quote_selector).map(|x| x.inner_html());
    match quote.next() {
        Some(quote) => return Ok(quote),
        None => {
            // Didn't find value with previous selector, falling back to another one based on diffrent page layout
            let quote_bis_selector = match scraper::Selector::parse("#main-content > div > section.l-quotepage > header > div > div > div.c-faceplate__company > div.c-faceplate__info > div > div.c-faceplate__price > span.c-instrument.c-instrument--last") {
                Ok(elem) => elem,
                Err(_) => { warn!("Element not found");
                return Err(BoursoScrapeError::ParseError);},
            };
            let mut quote_bis = document.select(&quote_bis_selector).map(|x| x.inner_html());
            match quote_bis.next() {
                Some(q) => return Ok(q),
                None => return Err(BoursoScrapeError::ParseError),
            }
        }
    }
    // quote.for_each(|x| println!("Quote: {}", x));
}
