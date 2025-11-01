use crate::errors::macro_error::MacroError;
use std::collections::HashMap;

pub fn generate_create_table_sql(
    table_name: &str,
    properties: &HashMap<String, String>,
) -> Result<String, MacroError> {
    let mut columns = Vec::new();

    columns.push("identifier UUID PRIMARY KEY".to_string());
    columns.push("created_at TIMESTAMP NOT NULL DEFAULT NOW()".to_string());
    columns.push("updated_at TIMESTAMP NOT NULL DEFAULT NOW()".to_string());

    for (key, value) in properties {
        columns.push(format!("{key} {kind}", kind = map_field_type(value)))
    }

    let sql = columns.join(",");

    Ok(sql)
}

fn map_field_type(field_type: &str) -> String {
    let lower = field_type.to_lowercase();

    if lower.starts_with("enum:") {
        field_type.to_string()
    } else {
        match lower.as_str() {
            "string" => "TEXT".to_string(),
            "integer" | "int" => "INTEGER".to_string(),
            "float" => "REAL".to_string(),
            "bool" | "boolean" => "BOOLEAN".to_string(),
            "uuid" => "UUID".to_string(),
            "date" => "TIMESTAMP".to_string(),
            _ => "TEXT".to_string(),
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_sql_generation() {
        let properties = HashMap::from([
            ("age".to_string(), "integer".to_string()),
            ("country".to_string(), "string".to_string()),
        ]);

        let sql = generate_create_table_sql("users", &properties).expect("SQL generation failed");

        // Check key parts of the SQL
        assert!(sql.contains("identifier UUID PRIMARY KEY"));
        assert!(sql.contains("created_at TIMESTAMP NOT NULL DEFAULT NOW()"));
        assert!(sql.contains("updated_at TIMESTAMP NOT NULL DEFAULT NOW()"));
        assert!(sql.contains("age INTEGER"));
        assert!(sql.contains("country TEXT"));

        // Optionally check the full output structure
        let expected_parts = vec![
            "identifier UUID PRIMARY KEY",
            "created_at TIMESTAMP NOT NULL DEFAULT NOW()",
            "updated_at TIMESTAMP NOT NULL DEFAULT NOW()",
            "age INTEGER",
            "country TEXT",
        ];

        for part in expected_parts {
            assert!(
                sql.contains(part),
                "Expected part `{part}` missing in SQL: {sql}"
            );
        }
    }
}

