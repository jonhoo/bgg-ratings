use std::collections::HashMap;

fn main() {
    let game = std::env::args().nth(1).expect("no game id given");
    let mut page = 1;
    let mut total;
    let mut users = HashMap::new();
    loop {
        let body: serde_json::Value =
            reqwest::get(&format!("https://api.geekdo.com/api/collections?ajax=1&objectid={}&objecttype=thing&oneperuser=1&pageid={}&require_review=true&showcount=200&sort=review_tstamp", game, page))
                .unwrap()
                .json()
                .unwrap();
        total = body["config"]["numitems"].as_u64().unwrap() as usize;
        let ratings = body["items"].as_array().unwrap();
        for rating in ratings {
            let r = rating["rating"].as_f64().unwrap();
            if r == 0.0 {
                //eprintln!("skip {} with {}", rating["user"]["username"], r);
                continue;
            }
            users
                .entry(rating["user"]["username"].as_str().unwrap().to_string())
                .or_insert(r);
        }
        if ratings.is_empty() {
            break;
        }
        eprintln!("ratings: {}/{} (page: {})", users.len(), total, page);
        page += 1;
    }

    let out = std::io::stdout();
    let out = out.lock();
    let mut out = csv::Writer::from_writer(out);
    let nusers = users.len();
    for (i, (user, rating)) in users.into_iter().enumerate() {
        let page = reqwest::get(&format!(
            "https://www.boardgamegeek.com/collection/user/{}?rated=1&subtype=boardgame&ff=1",
            user,
        ))
        .unwrap()
        .text()
        .unwrap();

        if i % 25 == 0 {
            eprintln!("users: {}/{}", i + 1, nusers);
            out.flush().unwrap();
        }

        for line in page.lines() {
            let line = line.trim();
            if line.starts_with("1 to ") && line.contains(" of ") {
                let left = line.find(" of ").unwrap();
                let line = &line[left + 4..];
                let right = line.find('&').unwrap_or(line.len() - 1);
                out.write_record(&[&format!("{}", rating), &line[..right], &user])
                    .unwrap();
                break;
            }
        }
    }
}
