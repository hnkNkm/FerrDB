use crate::utils::trim_quotes;

pub fn parse_select_table(command: &str) -> Option<(String, Vec<String>)> {
    // コマンドから末尾のセミコロンを除去し、前後の空白をトリム
    let command = command.trim_end_matches(';').trim();
    // "SELECT " で始まっているか確認
    if !command.to_uppercase().starts_with("SELECT ") {
        return None;
    }
    // "SELECT " を除去して残りを取得
    let without_select = &command[7..]; // "SELECT "は7文字
    // " FROM " で分割（前半がカラム指定、後半がテーブル名）
    let parts: Vec<&str> = without_select.splitn(2, " FROM ").collect();
    if parts.len() != 2 {
        return None;
    }
    let columns_str = parts[0].trim();
    let table_name = parts[1].trim().to_string();
    // カラム指定が "*" の場合はそのまま、それ以外はカンマで分割して各カラム名をトリム
    let columns: Vec<String> = if columns_str == "*" {
        vec!["*".to_string()]
    } else {
        columns_str.split(',')
            .map(|s| s.trim().to_string())
            .collect()
    };
    Some((table_name, columns))
}


/// 例: "SELECT * FROM users WHERE age = '30';" → ("users", "*", "age", "30")
pub fn parse_select_where(command: &str) -> Option<(String, Vec<String>, String, String)> {
    // 末尾のセミコロンを除去しトリム
    let command = command.trim_end_matches(';').trim();
    // " WHERE " で分割（前半が SELECT 部、後半が条件部）
    let parts: Vec<&str> = command.splitn(2, " WHERE ").collect();
    if parts.len() != 2 {
        return None;
    }
    // SELECT 部を解析
    if !parts[0].to_uppercase().starts_with("SELECT ") {
        return None;
    }
    let select_part = &parts[0][7..]; // "SELECT "を除去
    let select_parts: Vec<&str> = select_part.splitn(2, " FROM ").collect();
    if select_parts.len() != 2 {
        return None;
    }
    let columns_str = select_parts[0].trim();
    let table_name = select_parts[1].trim().to_string();
    let columns: Vec<String> = if columns_str == "*" {
        vec!["*".to_string()]
    } else {
        columns_str.split(',')
            .map(|s| s.trim().to_string())
            .collect()
    };
    // WHERE 部の解析（例: "age = '30'" や "id = 10"）
    let cond_parts: Vec<&str> = parts[1].splitn(2, "=").map(|s| s.trim()).collect();
    if cond_parts.len() != 2 {
        return None;
    }
    let condition_column = cond_parts[0].to_string();
    let condition_value = trim_quotes(cond_parts[1]);
    Some((table_name, columns, condition_column, condition_value))
}