use crate::applescript::run_applescript;
use crate::types::{Space, Tab};

pub async fn get_version() -> Result<String, String> {
    let script = r#"
        tell application "Arc"
            return version
        end tell
    "#;

    run_applescript(script).await
}

pub async fn get_spaces() -> Result<Vec<Space>, String> {
    let script = r#"
        set _output to ""

        tell application "Arc"
            set _space_index to 1

            repeat with _space in spaces of front window
                set _title to get title of _space

                set _output to (_output & "{ \"title\": \"" & _title & "\", \"id\": " & _space_index & " }")

                if _space_index < (count spaces of front window) then
                    set _output to (_output & ",\n")
                else
                    set _output to (_output & "\n")
                end if

                set _space_index to _space_index + 1
            end repeat
        end tell

        return "[\n" & _output & "\n]"
    "#;

    let response = run_applescript(script).await?;
    serde_json::from_str(&response).map_err(|e| format!("Failed to parse spaces: {}", e))
}

pub async fn select_space(id: i32) -> Result<(), String> {
    let script = format!(
        r#"
        launch app "Arc"
        delay 1
        tell application "Arc"
            tell front window
                tell space {} to focus
            end tell
        end tell
        "#,
        id
    );

    run_applescript(&script).await.map(|_| ())
}

pub async fn get_tabs() -> Result<Vec<Tab>, String> {
    let script = r#"
        on escape_value(this_text)
            set AppleScript's text item delimiters to the "\""
            set the item_list to every text item of this_text
            set AppleScript's text item delimiters to the "\\\""
            set this_text to the item_list as string
            set AppleScript's text item delimiters to ""
            return this_text
        end escape_value

        set _output to ""

        tell application "Arc"
            set _window_index to 1
            set _tab_index to 1

            repeat with _tab in tabs of first window
                set _title to my escape_value(get title of _tab)
                set _url to get URL of _tab
                set _location to get location of _tab

                set _output to (_output & "{ \"title\": \"" & _title & "\", \"url\": \"" & _url & "\", \"windowId\": " & _window_index & ", \"tabId\": " & _tab_index & " , \"location\": \"" & _location & "\" }")

                if _tab_index < (count tabs of first window) then
                    set _output to (_output & ",\n")
                else
                    set _output to (_output & "\n")
                end if

                set _tab_index to _tab_index + 1
            end repeat
        end tell

        return "[\n" & _output & "\n]"
    "#;

    let response = run_applescript(script).await?;
    serde_json::from_str(&response).map_err(|e| format!("Failed to parse tabs: {}", e))
}

pub async fn new_tab(url: &str) -> Result<(), String> {
    let script = format!(
        r#"
        tell application "Arc"
            tell front window
                make new tab with properties {{URL:"{}"}}
            end tell

            activate
        end tell
        "#,
        url
    );

    run_applescript(&script).await.map(|_| ())
}

pub async fn select_tab(window_id: i32, tab_id: i32) -> Result<(), String> {
    let script = format!(
        r#"
        tell application "Arc"
            tell window {} to tell tab {} to select
            activate
        end tell
        "#,
        window_id, tab_id
    );

    run_applescript(&script).await.map(|_| ())
}

pub async fn reload_tab(window_id: i32, tab_id: i32) -> Result<(), String> {
    let script = format!(
        r#"
        tell application "Arc"
            tell window {} to tell tab {} to reload
        end tell
        "#,
        window_id, tab_id
    );

    run_applescript(&script).await.map(|_| ())
}

pub async fn close_tab(window_id: i32, tab_id: i32) -> Result<(), String> {
    let script = format!(
        r#"
        tell application "Arc"
            tell window {} to tell tab {} to close
        end tell
        "#,
        window_id, tab_id
    );

    run_applescript(&script).await.map(|_| ())
}

pub async fn new_little_arc(url: &str) -> Result<(), String> {
    let script = format!(
        r#"
        tell application "Arc"
            make new tab with properties {{URL:"{}"}}
            activate
        end tell
        "#,
        url
    );

    run_applescript(&script).await.map(|_| ())
}
