use std::io::Read;

fn get_tailwind_config() -> String {
    let path = "/home/hadson/Documentos/IAgiliza/brokers-frontend/tailwind.config.js".to_string();
    path
}

pub fn update_brokers_color(color: String) {
    let path = get_tailwind_config();
    let file = std::fs::read_to_string(&path).unwrap();

    let new_lines: Vec<String> = file
        .lines()
        .map(|l| {
            for i in 0..7 {
                if l.trim().starts_with(&format!("'purple-{}", i)) {
                    return String::from(format!("'purple-{}': '{}',", i, color));
                }
            }

            l.to_string()
        })
        .collect();

    std::fs::write(path, new_lines.join("\n")).unwrap();
}
