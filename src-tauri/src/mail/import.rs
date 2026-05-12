use std::collections::HashSet;

use crate::models::account::{AccountImportInput, InvalidImportLine};

pub fn parse_account_import(input: &str) -> (Vec<AccountImportInput>, Vec<InvalidImportLine>) {
    let mut accounts = Vec::new();
    let mut invalid = Vec::new();
    let mut seen = HashSet::new();

    for (index, raw_line) in input.lines().enumerate() {
        let line_number = index + 1;
        let value = raw_line.trim();

        if value.is_empty() {
            continue;
        }

        let parts = value.split("----").map(str::trim).collect::<Vec<_>>();
        if parts.len() != 4 {
            invalid.push(InvalidImportLine {
                line_number,
                reason: "格式必须是 email----password----client_id----refresh_token".to_string(),
                value: value.to_string(),
            });
            continue;
        }

        if parts.iter().any(|part| part.is_empty()) {
            invalid.push(InvalidImportLine {
                line_number,
                reason: "四个字段都不能为空".to_string(),
                value: value.to_string(),
            });
            continue;
        }

        let email = parts[0].to_lowercase();
        if !email.contains('@') || !email.contains('.') {
            invalid.push(InvalidImportLine {
                line_number,
                reason: "邮箱格式无效".to_string(),
                value: value.to_string(),
            });
            continue;
        }

        if !seen.insert(email.clone()) {
            invalid.push(InvalidImportLine {
                line_number,
                reason: "本次导入中邮箱重复".to_string(),
                value: value.to_string(),
            });
            continue;
        }

        accounts.push(AccountImportInput {
            email,
            password: parts[1].to_string(),
            client_id: parts[2].to_string(),
            refresh_token: parts[3].to_string(),
        });
    }

    (accounts, invalid)
}

#[cfg(test)]
mod tests {
    use super::parse_account_import;

    #[test]
    fn parses_valid_account_line() {
        let (accounts, invalid) = parse_account_import(
            "KatieFerguson7034@outlook.com----fjdr3810----client_id----refresh_token",
        );

        assert!(invalid.is_empty());
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].email, "katieferguson7034@outlook.com");
        assert_eq!(accounts[0].password, "fjdr3810");
        assert_eq!(accounts[0].client_id, "client_id");
        assert_eq!(accounts[0].refresh_token, "refresh_token");
    }

    #[test]
    fn ignores_empty_lines() {
        let (accounts, invalid) = parse_account_import("\n\n  \n");

        assert!(accounts.is_empty());
        assert!(invalid.is_empty());
    }

    #[test]
    fn reports_malformed_line() {
        let (accounts, invalid) = parse_account_import("bad----line");

        assert!(accounts.is_empty());
        assert_eq!(invalid.len(), 1);
        assert_eq!(invalid[0].line_number, 1);
    }

    #[test]
    fn trims_whitespace() {
        let (accounts, invalid) =
            parse_account_import("  user@outlook.com ---- pass ---- client ---- token  ");

        assert!(invalid.is_empty());
        assert_eq!(accounts[0].email, "user@outlook.com");
        assert_eq!(accounts[0].password, "pass");
    }
}
