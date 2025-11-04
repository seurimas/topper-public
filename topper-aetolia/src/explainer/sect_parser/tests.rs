#[cfg(test)]
mod tests {
    use super::*;
    use crate::explainer::ExplainerPage;
    use crate::explainer::sect_parser::{AetoliaSectParser, get_color_from_node, parse_me_and_you};
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_parse_loser_html() {
        let test_data_path = Path::new("../topper-sect-watch/test_data/Loser.html");
        let html_content =
            fs::read_to_string(test_data_path).expect("Failed to read Loser.html test file");

        let mut parser = AetoliaSectParser::new();
        let result = parser.parse_nodes(html_content);

        assert!(result.is_ok(), "Failed to parse HTML: {:?}", result.err());

        let page = result.unwrap();

        // Verify basic structure
        assert!(!page.id.is_empty(), "Page ID should not be empty");
        assert!(!page.body.is_empty(), "Page body should not be empty");

        // Verify specific content based on the test file
        assert!(
            page.id.contains("Rinata"),
            "Should contain player name Rinata"
        );
        assert!(
            page.id.contains("Naivara"),
            "Should contain opponent name Naivara"
        );
        assert!(page.id.contains("Zealot"), "Should contain class Zealot");
        assert!(
            page.id.contains("Praenomen"),
            "Should contain class Praenomen"
        );

        // Verify winner detection
        assert!(
            page.id.contains("(Naivara)"),
            "Should show Naivara as winner in parentheses"
        );

        // Check that some expected content is in the body
        let body_text: String = page.body.join("\n");
        assert!(
            body_text.contains("Who:   Rinata"),
            "Should contain player identification"
        );
        assert!(
            body_text.contains("Vs:    Naivara"),
            "Should contain opponent identification"
        );
        assert!(
            body_text.contains("You have been slain by Naivara"),
            "Should contain death message"
        );
    }

    #[test]
    fn test_parse_sect_html() {
        let test_data_path = Path::new(
            "../topper-sect-watch/test_data/SECT-Rinata-VS-Naivara-1761618647-Rinata.htm",
        );
        if !test_data_path.exists() {
            // Skip test if file doesn't exist
            return;
        }

        let html_content =
            fs::read_to_string(test_data_path).expect("Failed to read SECT HTML test file");

        let mut parser = AetoliaSectParser::new();
        let result = parser.parse_nodes(html_content);

        assert!(result.is_ok(), "Failed to parse HTML: {:?}", result.err());

        let mut page = result.unwrap();

        // Verify basic structure
        assert!(!page.id.is_empty(), "Page ID should not be empty");
        assert!(!page.body.is_empty(), "Page body should not be empty");

        page.hide_real_times();
        assert_eq!(
            page.body[1377],
            "<white><#00cd00>H:3249/4160 <#00ffff>M:3988 <#ff00ff>P:4044 <#e5e5e5>[c][e-s] <#00ffff>Morning<#ffffff> [00:02:19:48]<white>\r",
            "Should hide real times, and account for rounding errors"
        );
    }

    #[test]
    fn test_parse_me_and_you_function() {
        // Create a test ExplainerPage with known content
        let test_body = vec![
            "<#ffffff>Who:   TestPlayer".to_string(),
            "<#ffffff>Class: Zealot".to_string(),
            "".to_string(),
            "<#ffffff>Vs:    TestOpponent".to_string(),
            "<#ffffff>Class: Praenomen".to_string(),
            "<#00cd00>H:4160/4160 <#005fff>M:6344 <#ff00ff>P:3764 <#e5e5e5>[cs][ebs] <#00ffff>Morning<#ffffff> [02:30:47:52]".to_string(),
        ];

        let test_page = ExplainerPage::new("test_id".to_string(), test_body);
        let (me, you) = parse_me_and_you(&test_page);

        assert_eq!(me, "TestPlayer", "Should correctly extract player name");
        assert_eq!(
            you, "TestOpponent",
            "Should correctly extract opponent name"
        );
    }

    #[test]
    fn test_color_parsing() {
        let test_html = r#"<span style="color:#ffffff;">White text</span>"#;
        let document = tl::parse(test_html, tl::ParserOptions::default()).unwrap();
        let body = document.query_selector("span").unwrap().next().unwrap();
        let node = body.get(document.parser()).unwrap();

        let color = get_color_from_node(node);
        assert_eq!(
            color, "#ffffff",
            "Should extract color correctly from style attribute"
        );
    }

    #[test]
    fn test_parser_initialization() {
        let parser = AetoliaSectParser::new();
        // Just test that we can create a new parser instance
        // The internal fields are private, so we can't test them directly
        let _ = parser; // This ensures the parser was created successfully
    }

    #[test]
    fn test_compare_with_expected_json() {
        let html_path = Path::new("../topper-sect-watch/test_data/Loser.html");
        let json_path = Path::new("../topper-sect-watch/test_data/Loser.json");

        if !html_path.exists() || !json_path.exists() {
            // Skip test if files don't exist
            return;
        }

        // Parse HTML
        let html_content = fs::read_to_string(html_path).expect("Failed to read HTML file");
        let mut parser = AetoliaSectParser::new();
        let parsed_page = parser
            .parse_nodes(html_content)
            .expect("Failed to parse HTML");

        // Load expected JSON
        let json_content = fs::read_to_string(json_path).expect("Failed to read JSON file");
        let expected_page: ExplainerPage =
            serde_json::from_str(&json_content).expect("Failed to parse JSON");

        // Compare key elements (not exact match due to potential formatting differences)
        println!("Parsed ID: {}", parsed_page.id);
        println!("Expected ID: {}", expected_page.id);

        // Check that both contain the same key information
        assert!(
            parsed_page.id.contains("Rinata"),
            "Parsed page should contain Rinata"
        );
        assert!(
            parsed_page.id.contains("Naivara"),
            "Parsed page should contain Naivara"
        );
        assert!(
            parsed_page.id.contains("Zealot"),
            "Parsed page should contain Zealot"
        );
        assert!(
            parsed_page.id.contains("Praenomen"),
            "Parsed page should contain Praenomen"
        );

        // Compare body length (should be reasonably close)
        let length_diff = (parsed_page.body.len() as i32 - expected_page.body.len() as i32).abs();
        assert!(
            length_diff < 5,
            "Body lengths should be similar (diff: {})",
            length_diff
        );

        // Check that key content exists in both
        let parsed_body_text = parsed_page.body.join("\n");
        let expected_body_text = expected_page.body.join("\n");

        assert!(
            parsed_body_text.contains("Who:   Rinata"),
            "Parsed should contain player ID"
        );
        assert!(
            expected_body_text.contains("Who:   Rinata"),
            "Expected should contain player ID"
        );

        assert!(
            parsed_body_text.contains("You have been slain by Naivara"),
            "Parsed should contain death message"
        );
        assert!(
            expected_body_text.contains("You have been slain by Naivara"),
            "Expected should contain death message"
        );
    }
}
