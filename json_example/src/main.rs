#[macro_use]
extern crate json;

fn main(){
    let parsed_data = json::parse(r#"

    {
        "userid": 103609,
        "verified": true,
        "access_privelages": [
            "user",
            "admin"
        ]
    }

    "#).unwrap();

    let instantiated_data = object!{
        "userid" => 103609,
        "verified" => true,
        "access_privelages" => array![
            "user",
            "admin"
        ]
    };

    assert_eq!(parsed_data, instantiated_data);
}
