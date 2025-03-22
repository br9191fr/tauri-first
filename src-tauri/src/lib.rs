// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet_cmd(name: &str) -> String {
    let xx = 5;
    let yy = xx*10 +2;
    println!("In greet");
    format!("Hello, {} step {}! You've been greeted from Rust!", name,yy)
}

#[tauri::command]
fn other_cmd(name: &str) -> String {
    let xx = 3;
    let yy = xx*10 +3;
    println!("In other");
    format!("Hello, {} step {}! Next msg from Rust!", name,yy)
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("In run");
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![other_cmd, greet_cmd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    println!("two functions registered");
}
