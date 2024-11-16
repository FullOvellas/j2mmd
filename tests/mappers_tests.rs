use assert_unordered::assert_eq_unordered;
use j2mmd::{domain::Mapper, get_mappers};

#[test]
fn get_mappers_successfully() {
    let mut actual = Vec::new();
    get_mappers(
        format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/tests/test_data/mappers"
        )
        .as_str(),
        &mut actual,
    );
    assert_eq_unordered!(
        actual,
        vec![
            Mapper {
                name: String::from("StandaloneMapper"),
                used_sources: Vec::new()
            },
            Mapper {
                name: String::from("AddressMapper"),
                used_sources: Vec::new(),
            },
            Mapper {
                name: String::from("CustomerMapper"),
                used_sources: vec![String::from("AddressMapper")],
            },
            Mapper {
                name: String::from("UserMapper"),
                used_sources: vec![String::from("CustomerMapper")]
            },
            Mapper {
                name: String::from("OrderMapper"),
                used_sources: vec![String::from("ProductMapper"), String::from("AddressMapper")]
            },
            Mapper {
                name: String::from("ProductMapper"),
                used_sources: Vec::new(),
            }
        ]
    );
}
