use std::collections::HashSet;

use mail_parser::MessageParser;
use regex::Regex;

use crate::models::email::StoredEmail;

pub fn extract_codes(text: &str) -> Vec<String> {
    let regex = Regex::new(r"\b\d{4,8}\b").expect("verification code regex must compile");
    let mut seen = HashSet::new();
    let mut codes = Vec::new();

    for matched in regex.find_iter(text) {
        if !has_code_context(text, matched.start(), matched.end()) {
            continue;
        }
        let value = matched.as_str().to_string();
        if seen.insert(value.clone()) {
            codes.push(value);
        }
    }

    codes
}

pub fn parse_message(account_id: &str, uid: i64, raw_message: &[u8]) -> StoredEmail {
    if let Some(email) = parse_message_with_mime_parser(account_id, uid, raw_message) {
        return email;
    }

    parse_message_fallback(account_id, uid, raw_message)
}

fn parse_message_with_mime_parser(
    account_id: &str,
    uid: i64,
    raw_message: &[u8],
) -> Option<StoredEmail> {
    let message = MessageParser::default().parse(raw_message)?;
    let subject = message.subject().unwrap_or("Untitled").to_string();
    let from = message.from().and_then(|address| address.first());
    let sender_name = from
        .and_then(|address| address.name.as_deref())
        .map(str::to_string);
    let sender_email = from
        .and_then(|address| address.address.as_deref())
        .map(str::to_string);
    let received_at = message.date().map(|date| date.to_rfc3339());
    let message_id = message.message_id().map(str::to_string);
    let body_text = message
        .body_text(0)
        .map(|body| clean_display_body(body.as_ref()))
        .or_else(|| message.body_html(0).map(|body| clean_display_body(body.as_ref())))
        .unwrap_or_else(|| clean_display_body(String::from_utf8_lossy(raw_message).as_ref()));
    let body_html = message.body_html(0).map(|body| body.into_owned());
    let combined = format!("{}\n{}", subject, body_text);

    Some(StoredEmail {
        id: uuid::Uuid::new_v4().to_string(),
        account_id: account_id.to_string(),
        uid,
        message_id,
        subject,
        sender_name,
        sender_email,
        received_at,
        body_text: Some(body_text),
        body_html,
        codes: extract_codes(&combined),
    })
}

fn parse_message_fallback(account_id: &str, uid: i64, raw_message: &[u8]) -> StoredEmail {
    let raw = String::from_utf8_lossy(raw_message).to_string();
    let (headers, body) = split_headers_body(&raw);
    let subject = header_value(headers, "subject").unwrap_or_else(|| "Untitled".to_string());
    let from = header_value(headers, "from");
    let (sender_name, sender_email) = parse_sender(from.as_deref());
    let received_at = header_value(headers, "date");
    let message_id = header_value(headers, "message-id");
    let body_text = extract_plain_text_body(body);
    let combined = format!("{}\n{}", subject, body_text);

    StoredEmail {
        id: uuid::Uuid::new_v4().to_string(),
        account_id: account_id.to_string(),
        uid,
        message_id,
        subject,
        sender_name,
        sender_email,
        received_at,
        body_text: Some(body_text),
        body_html: None,
        codes: extract_codes(&combined),
    }
}

fn split_headers_body(raw: &str) -> (&str, &str) {
    raw.split_once("\r\n\r\n")
        .or_else(|| raw.split_once("\n\n"))
        .unwrap_or((raw, ""))
}

fn header_value(headers: &str, name: &str) -> Option<String> {
    let mut current_name = String::new();
    let mut current_value = String::new();

    for line in headers.lines() {
        if line.starts_with(' ') || line.starts_with('\t') {
            current_value.push(' ');
            current_value.push_str(line.trim());
            continue;
        }

        if !current_name.is_empty() && current_name.eq_ignore_ascii_case(name) {
            return normalize_header_value(&current_value);
        }

        if let Some((key, value)) = line.split_once(':') {
            current_name = key.trim().to_string();
            current_value = value.trim().to_string();
        }
    }

    if !current_name.is_empty() && current_name.eq_ignore_ascii_case(name) {
        return normalize_header_value(&current_value);
    }

    None
}

fn normalize_header_value(value: &str) -> Option<String> {
    Some(value.trim().trim_matches('"').to_string()).filter(|value| !value.is_empty())
}

fn parse_sender(value: Option<&str>) -> (Option<String>, Option<String>) {
    let Some(value) = value else {
        return (None, None);
    };

    if let Some((name, email)) = value.rsplit_once('<') {
        return (
            Some(name.trim().trim_matches('"').to_string()).filter(|name| !name.is_empty()),
            Some(email.trim().trim_end_matches('>').to_string()).filter(|email| !email.is_empty()),
        );
    }

    (
        None,
        Some(value.to_string()).filter(|email| email.contains('@')),
    )
}

fn extract_plain_text_body(body: &str) -> String {
    let normalized = body.replace("\r\n", "\n");
    let decoded = decode_quoted_printable(&normalized);
    let visible_source = preferred_visible_body_source(&decoded);
    clean_display_body(visible_source)
}

fn clean_display_body(body: &str) -> String {
    let cleaned = body
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.starts_with("--")
                && !trimmed.to_ascii_lowercase().starts_with("content-")
                && !trimmed.eq_ignore_ascii_case("quoted-printable")
        })
        .collect::<Vec<_>>()
        .join("\n");

    normalize_display_lines(&strip_html(&cleaned))
}

fn normalize_display_lines(text: &str) -> String {
    let mut output = Vec::new();
    let mut seen = HashSet::new();
    let mut blank_count = 0;

    for line in text.lines() {
        let normalized = normalize_display_line(line);
        let trimmed = normalized.trim();

        if should_drop_display_line(trimmed) {
            continue;
        }

        if trimmed.is_empty() {
            blank_count += 1;
            if blank_count <= 1 && !output.is_empty() {
                output.push(String::new());
            }
            continue;
        }

        blank_count = 0;
        let key = trimmed.to_ascii_lowercase();
        if !seen.insert(key) {
            continue;
        }
        output.push(trimmed.to_string());
    }

    output.join("\n").trim().to_string()
}

fn normalize_display_line(line: &str) -> String {
    line.chars()
        .filter(|character| !matches!(character, '\u{200c}' | '\u{200d}' | '\u{034f}'))
        .collect::<String>()
        .replace('\u{00a0}', " ")
}

fn should_drop_display_line(line: &str) -> bool {
    if line.is_empty() {
        return false;
    }

    let lower = line.to_ascii_lowercase();
    lower.contains("source_code=")
        || lower.starts_with("mi cuenta http")
        || lower.starts_with("mi biblioteca http")
        || lower.starts_with("lista de deseos http")
        || lower.starts_with("centro de ayuda http")
        || lower.starts_with("política de privacidad http")
        || lower.starts_with("requisitos de la aplicación audible http")
        || lower == "membership now"
        || lower == "audible - una empresa de amazon"
        || lower.starts_with('©')
}

fn preferred_visible_body_source(body: &str) -> &str {
    let html_start = body
        .to_ascii_lowercase()
        .find("<!doctype html")
        .or_else(|| body.to_ascii_lowercase().find("<html"));

    if let Some(index) = html_start {
        let plain_candidate = body[..index].trim();
        if !plain_candidate.is_empty() {
            return plain_candidate;
        }
    }

    body
}

fn has_code_context(text: &str, start: usize, end: usize) -> bool {
    let before_start = previous_char_boundary(text, start.saturating_sub(40));
    let before = text[before_start..start].to_ascii_lowercase();
    let after_end = next_char_boundary(text, (end + 24).min(text.len()));
    let after = text[end..after_end].to_ascii_lowercase();

    let before_patterns = [
        "code",
        "otp",
        "one time password",
        "verification",
        "verify",
        "验证码",
    ];
    let after_patterns = ["code", "otp", "验证码"];

    before_patterns
        .iter()
        .any(|keyword| has_before_code_context(&before, keyword))
        || after_patterns
            .iter()
            .any(|keyword| contains_context_keyword(&after, keyword))
}

fn has_before_code_context(text: &str, keyword: &str) -> bool {
    context_keyword_ranges(text, keyword).any(|(index, length)| {
        let suffix = &text[index + length..];
        !suffix
            .chars()
            .any(|character| matches!(character, '.' | '!' | '?' | '。' | '！' | '？'))
            && !contains_code_candidate(suffix)
    })
}

fn contains_code_candidate(text: &str) -> bool {
    let mut digits = 0;
    for byte in text.bytes() {
        if byte.is_ascii_digit() {
            digits += 1;
            continue;
        }
        if (4..=8).contains(&digits) {
            return true;
        }
        digits = 0;
    }
    (4..=8).contains(&digits)
}

fn previous_char_boundary(text: &str, mut index: usize) -> usize {
    while index > 0 && !text.is_char_boundary(index) {
        index -= 1;
    }
    index
}

fn next_char_boundary(text: &str, mut index: usize) -> usize {
    while index < text.len() && !text.is_char_boundary(index) {
        index += 1;
    }
    index
}

fn contains_context_keyword(text: &str, keyword: &str) -> bool {
    context_keyword_ranges(text, keyword).next().is_some()
}

fn context_keyword_ranges<'a>(
    text: &'a str,
    keyword: &'a str,
) -> impl Iterator<Item = (usize, usize)> + 'a {
    text.match_indices(keyword).filter_map(move |(index, value)| {
        if keyword == "验证码" {
            return Some((index, value.len()));
        }

        let before = index
            .checked_sub(1)
            .and_then(|offset| text.as_bytes().get(offset))
            .copied();
        let after = text.as_bytes().get(index + value.len()).copied();
        (!is_identifier_byte(before) && !is_identifier_byte(after)).then_some((index, value.len()))
    })
}

fn is_identifier_byte(value: Option<u8>) -> bool {
    value.is_some_and(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
}

fn decode_quoted_printable(text: &str) -> String {
    let bytes = text.replace("=\n", "").into_bytes();
    let mut output = Vec::new();
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] == b'=' && index + 2 < bytes.len() {
            if let Ok(hex) = std::str::from_utf8(&bytes[index + 1..index + 3]) {
                if let Ok(value) = u8::from_str_radix(hex, 16) {
                    output.push(value);
                    index += 3;
                    continue;
                }
            }
        }
        output.push(bytes[index]);
        index += 1;
    }

    String::from_utf8_lossy(&output).to_string()
}

fn strip_html(text: &str) -> String {
    let style_regex = Regex::new(r"(?is)<style[\s\S]*?</style>").expect("style regex must compile");
    let script_regex = Regex::new(r"(?is)<script[\s\S]*?</script>").expect("script regex must compile");
    let tag_regex = Regex::new(r"(?is)<[^>]+>").expect("tag regex must compile");
    let without_style = style_regex.replace_all(text, " ");
    let without_script = script_regex.replace_all(&without_style, " ");
    tag_regex.replace_all(&without_script, " ").to_string()
}

#[cfg(test)]
mod tests {
    use super::{extract_codes, parse_message};

    #[test]
    fn extracts_four_to_eight_digit_codes() {
        assert_eq!(
            extract_codes("OpenAI code 349000, Microsoft code 1234."),
            vec!["349000", "1234"]
        );
    }

    #[test]
    fn ignores_short_and_long_numbers() {
        assert_eq!(extract_codes("12 123456789"), Vec::<String>::new());
    }

    #[test]
    fn ignores_tracking_source_code_numbers() {
        assert_eq!(
            extract_codes("Manage your account: https://example.com/account?source_code=AUDOREM1216179MBK&color=000000"),
            Vec::<String>::new()
        );
    }

    #[test]
    fn extracts_codes_without_panicking_near_unicode_boundaries() {
        assert_eq!(
            extract_codes("verification \u{200c}\u{200c}\u{200c}\u{200c}\u{200c}\u{200c}\u{200c}\u{200c}\u{200c}\u{200c}\u{200c}\u{200c} code 123456"),
            vec!["123456"]
        );
    }

    #[test]
    fn deduplicates_codes_preserving_order() {
        assert_eq!(
            extract_codes("OpenAI code 349000, Microsoft code 1234, OpenAI code 349000"),
            vec!["349000", "1234"]
        );
    }

    #[test]
    fn parses_basic_message_fields() {
        let message = b"Subject: OpenAI code\r\nFrom: OpenAI <noreply@openai.com>\r\nMessage-ID: <1>\r\n\r\nYour code is 349000";
        let parsed = parse_message("account", 7, message);

        assert_eq!(parsed.subject, "OpenAI code");
        assert_eq!(parsed.sender_email.as_deref(), Some("noreply@openai.com"));
        assert_eq!(parsed.codes, vec!["349000"]);
    }

    #[test]
    fn extracts_only_contextual_verification_codes() {
        let text = "Your verification code is 194152. Address 410 Terry Ave, Seattle WA 98109. Color #303333 and year 2026.";

        assert_eq!(extract_codes(text), vec!["194152"]);
    }

    #[test]
    fn decodes_quoted_printable_text_body() {
        let message = b"Subject: Amazon OTP\r\nFrom: Amazon <account-update@amazon.com>\r\nContent-Transfer-Encoding: quoted-printable\r\n\r\nDon=E2=80=99t share this OTP=2E\r\nCode: 194152=\r\n\r\n";
        let parsed = parse_message("account", 8, message);

        assert_eq!(parsed.body_text.as_deref(), Some("Don’t share this OTP.\nCode: 194152"));
        assert_eq!(parsed.codes, vec!["194152"]);
    }

    #[test]
    fn strips_html_from_mixed_email_body() {
        let message = b"Subject: Verify your new Amazon account\r\nFrom: Amazon <account-update@amazon.com>\r\n\r\nTo verify your email address, please use the following One Time Password (OTP):\r\n\r\n194152\r\n\r\n<!doctype html><html><head><style>.footer{color:#303333}</style></head><body><p>Address 98109</p></body></html>";
        let parsed = parse_message("account", 9, message);
        let body = parsed.body_text.unwrap();

        assert!(body.contains("One Time Password"));
        assert!(!body.contains("<!doctype html>"));
        assert!(!body.contains(".footer"));
        assert_eq!(parsed.codes, vec!["194152"]);
    }

    #[test]
    fn extracts_text_from_pure_html_document_body() {
        let message = b"Subject: Microsoft update\r\nFrom: Microsoft <account-security-noreply@accountprotection.microsoft.com>\r\n\r\n<!DOCTYPE HTML><html><head><style>.button{color:#505050;background:#000000}</style></head><body><p>Get the mobile apps available with your Microsoft account.</p><p>1999 Microsoft Way, Redmond WA 98052</p></body></html>";
        let parsed = parse_message("account", 10, message);
        let body = parsed.body_text.unwrap();

        assert!(body.contains("Get the mobile apps available"));
        assert!(!body.contains("<!DOCTYPE HTML>"));
        assert!(!body.contains(".button"));
        assert_eq!(parsed.codes, Vec::<String>::new());
    }

    #[test]
    fn decodes_quoted_printable_html_before_extracting_codes() {
        let message = b"Subject: Amazon notice\r\nFrom: Amazon <account-update@amazon.com>\r\nContent-Transfer-Encoding: quoted-printable\r\n\r\n=20\r\n<html><head><style>.footer{color:#303333;background:#000000}</style></head><body><p>1999 Amazon Way, Seattle WA 98109</p></body></html>";
        let parsed = parse_message("account", 11, message);
        let body = parsed.body_text.unwrap();

        assert!(!body.contains("=20"));
        assert!(!body.contains(".footer"));
        assert_eq!(parsed.codes, Vec::<String>::new());
    }

    #[test]
    fn extracts_body_from_multipart_alternative_html() {
        let message = b"Subject: TikTok code\r\nFrom: TikTok <noreply@account.tiktok.com>\r\nContent-Type: multipart/alternative; boundary=mailboxhub-boundary\r\n\r\n--mailboxhub-boundary\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Transfer-Encoding: quoted-printable\r\n\r\n<html><body><p>Your verification code is <b>927723</b>.</p><p>Do not share it.</p></body></html>\r\n--mailboxhub-boundary--\r\n";
        let parsed = parse_message("account", 12, message);
        let body = parsed.body_text.unwrap();

        assert!(body.contains("Your verification code is 927723"));
        assert!(!body.contains("Content-Type"));
        assert!(!body.contains("<html"));
        assert_eq!(parsed.codes, vec!["927723"]);
    }

    #[test]
    fn removes_repeated_navigation_and_tracking_links_from_marketing_html() {
        let message = b"Subject: Tu pedido est\xC3\xA1 confirmado\r\nFrom: Audible <donotreply@audible.es>\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body><p>Membership Now</p><p>Membership Now</p><p>audible - una empresa de amazon</p><p>Mi Cuenta https://www.audible.es/account/overview/?source_code=AUDOREM1216179MBK</p><h1>\xC2\xA1Bienvenido a Audible!</h1><p>Aqu\xC3\xAD tienes la confirmaci\xC3\xB3n de tu pedido.</p><p>N\xC3\xBAmero de pedido: D01-4346203-6664655</p><p>Mi biblioteca https://www.audible.es/lib?source_code=AUDOREM1216179MBK</p><p>\xC2\xA91\xE2\x80\x8C9\xE2\x80\x8C9\xE2\x80\x8C7\xE2\x80\x8C-2026 Audible GmbH, 10117 Berl\xC3\xADn</p></body></html>";
        let parsed = parse_message("account", 13, message);
        let body = parsed.body_text.unwrap();

        assert!(body.starts_with("¡Bienvenido a Audible!"));
        assert!(body.contains("Número de pedido: D01-4346203-6664655"));
        assert!(!body.contains("Membership Now"));
        assert!(!body.contains("source_code="));
        assert!(!body.contains("©1\u{200c}9\u{200c}9\u{200c}7"));
        assert!(!body.contains("\n\n\n"));
    }
}
