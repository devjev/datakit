extern crate iso8601;

fn main() {
    let dtstr = "2020-07-01";
    let parsed_datetime = iso8601::datetime(dtstr);
    println!("{:?}", parsed_datetime);

    let parsed_date = iso8601::date(dtstr);
    println!("{:?}", parsed_date);

    let parsed_time = iso8601::time(dtstr);
    println!("{:?}", parsed_time);
}
