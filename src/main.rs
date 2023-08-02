use std::collections::HashMap;
use std::fs;

fn update_env(env_path: &str, env_sample_path: &str) -> std::io::Result<()> {
    // Read the .env.sample and .env files
    let sample_content = fs::read_to_string(env_sample_path)?;
    let env_content = fs::read_to_string(env_path)?;

    // Split the contents into lines
    let sample_lines: Vec<&str> = sample_content.lines().collect();
    let env_lines: Vec<&str> = env_content.lines().collect();

    let mut env_map = HashMap::new();

    for line in env_lines {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let key_value: Vec<&str> = line.splitn(2, '=').collect();
        env_map.insert(key_value[0], line);
    }

    // Write the keys and values to the .env in the order they appear in the .env.sample
    let mut output = String::new();
    for line in sample_lines {
        if line.is_empty() {
            output.push('\n');
        } else if line.starts_with('#') {
            output.push_str(line);
            output.push('\n');
        } else {
            let key: &str = line.split('=').collect::<Vec<&str>>()[0];
            if let Some(env_line) = env_map.remove(key) {
                output.push_str(env_line);
            } else {
                output.push_str(line);
            }
            output.push('\n');
        }
    }

    // Overwrite the .env file with the new content
    fs::write(env_path, output)?;
    Ok(())
}

// Usage
fn main() -> std::io::Result<()> {
    update_env(".env", ".env.sample")?;
    Ok(())
}
