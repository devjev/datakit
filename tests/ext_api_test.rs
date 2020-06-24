pub mod ext_api_serde_json {
    /// One Two Three
    #[test]
    fn parse_string_to_jsvalue_noquotes() {
        let err = serde_json::from_str::<String>("abc").unwrap_err();
    }
}
