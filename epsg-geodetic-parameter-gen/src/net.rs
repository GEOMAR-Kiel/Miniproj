//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

use reqwest::blocking::*;
use std::collections::HashMap;
use std::sync::Mutex;
use std::ops::DerefMut;

use lazy_static;

lazy_static::lazy_static! {
    static ref NET_CACHE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

const RETRY_LIMIT: usize = 8;

pub fn get_cached(url: &str) -> String {
    NET_CACHE.lock().map(|mut e| {
        let hm = e.deref_mut();
        if !hm.contains_key(url) {
            eprint!("Requesting `{}`...", url);
            let tmp = 
                std::iter::from_fn(|| Some(get(url)))
                    .enumerate()
                    .map(|(i, e)| {
                        std::thread::sleep(std::time::Duration::new((i * i) as u64, 5_000_000));
                        e})
                    .take(RETRY_LIMIT)
                    .find_map(|e| 
                        e.ok()
                        .filter(|r| r.status().as_u16() == 200)
                        .and_then(|r| r.text().ok()))
                    .expect("Too many retries.");
            hm.insert(url.into(), tmp);
            eprintln!(" Done.");
        }
        hm.get(url).unwrap().into()
    }).unwrap()
}