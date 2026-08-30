/*
in this file we can construct the query the user wants to send which could include areas to search, icao24 codes etc

Usage:
let url_builder =
    URLBuilder
        ::new()
        ::filter_area(some_sort_of_shape)

request::find_aircraft(&token_manager, &client,)

for complex shapes the user might draw on the map, im not sure the min/max lat/long approach works
and so we might have to find another approach
although we need min/max values to put in the url
we could scan all aircraft and check each one to see if they are in the area but this is a terrible idea, very inefficient and expensive
on tokens and slow

lets next work on sending notifications to users
*/

#[derive(Debug)]
pub struct URLBuilder {
    url: String,
}

impl URLBuilder {
    pub fn new() -> Self {
        Self {
            url: "https://opensky-network.org/api/states/all?".to_string(),
        }
    }

    pub fn filter_area(area: i32) {
        todo!()
    }
}
