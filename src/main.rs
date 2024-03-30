struct Game {
    home: String,
    away: String,
    time: Option<String>,
    score_home: Option<u8>,
    score_away: Option<u8>,
    competition: String,
}

fn get_games(url: &str) -> Vec<Game> {
    let res = reqwest::blocking::get(url);
    let html_content = res.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&html_content);
    let html_product_selector = scraper::Selector::parse("tr.football-match").unwrap();
    let html_products = document.select(&html_product_selector);

    let mut games: Vec<Game> = Vec::new();

    for html_product in html_products {
        let time = html_product
            .select(&scraper::Selector::parse("td time").unwrap())
            .next()
            .map(|time| time.attr("data-timestamp").unwrap());
        let home = html_product
            .select(
                &scraper::Selector::parse(".football-match__team--home .team-name__long").unwrap(),
            )
            .next()
            .map(|price| price.text().collect::<String>())
            .unwrap();
        let away = html_product
            .select(
                &scraper::Selector::parse(".football-match__team--away .team-name__long").unwrap(),
            )
            .next()
            .map(|price| price.text().collect::<String>())
            .unwrap();
        let score_home = html_product.attr("data-score-home").unwrap();
        let score_away = html_product.attr("data-score-away").unwrap();
        let game = Game {
            home,
            away,
            time: Some(match time {
                Some(time) => time.to_string(),
                None => "".to_string(),
            }),
            score_home: match score_home.parse::<u8>() {
                Ok(v) => Some(v),
                Err(_) => None,
            },
            score_away: match score_away.parse::<u8>() {
                Ok(v) => Some(v),
                Err(_) => None,
            },
            competition: String::from("Premier League"),
        };
        games.push(game);
    }
    games
}

fn write_csv(games: &Vec<Game>) {
    // create the CSV output file
    let path = std::path::Path::new("products.csv");
    let mut writer = csv::Writer::from_path(path).unwrap();

    // append the header to the CSV
    writer
        .write_record(&[
            "home",
            "away",
            "time",
            "score_home",
            "score_away",
            "competition",
        ])
        .unwrap();
    // populate the output file
    for game in games {
        writer
            .write_record(&[
                game.home.clone(),
                game.away.clone(),
                match &game.time {
                    Some(v) => v.to_string(),
                    None => String::from(""),
                },
                match game.score_home {
                    Some(v) => v.to_string(),
                    None => String::from(""),
                },
                match game.score_away {
                    Some(v) => v.to_string(),
                    None => String::from(""),
                },
                game.competition.clone(),
            ])
            .unwrap();
    }

    // free up the resources
    writer.flush().unwrap();
}

fn main() {
    let urls = [
        "https://www.theguardian.com/football/premierleague/results",
        "https://www.theguardian.com/football/premierleague/fixtures",
    ];
    let mut games: Vec<Game> = vec![];
    for url in urls {
        games.extend(get_games(url));
    }
    write_csv(&games);
}
