// It could be added to the CI but in some cases it will not be needed to clean
// the database, so running it manually. It can be moved to the scripts folder.

use crate::utils::Shell;
use argon2::{self, Config};
use uuid::Uuid;

use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_timestamp() -> u128 {
    let since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_millis()
}

pub fn run_sync_prod(shell: &mut Shell) {
    shell.run_status("cd service && rm -rf mahjong_prod.db");
    shell.run_status("cd service && DATABASE_URL=sqlite://mahjong_prod.db diesel setup");

    let admin_pass = shell
        .run_output("ssh mahjong-rust.com \"cat .env | grep MAHJONG_ADMIN_PASS | cut -d '=' -f2\"");
    let admin_pass = admin_pass.replace('\n', "");

    let salt = Uuid::new_v4().to_string();
    let config = Config::default();
    let hash = argon2::hash_encoded(admin_pass.as_bytes(), salt.as_bytes(), &config).unwrap();
    let user_id = Uuid::new_v4().to_string();
    let role = "\"Admin\"";

    let auth_sql_query = format!(
        "INSERT INTO auth_info (user_id, username, role, hashed_pass) VALUES ('{user_id}', '{username}', '{role}', '{hash}');",
        user_id = user_id,
        username = "admin",
        role = role,
        hash = hash
    );
    std::fs::write("/tmp/mahjong_query.sql", auth_sql_query).expect("Unable to write file");
    shell.run_status("cd service && sqlite3 mahjong_prod.db < /tmp/mahjong_query.sql");

    let player_sql_query = format!(
        "INSERT INTO player (id, is_ai, name, created_at) VALUES ('{id}', '{is_ai}', '{name}', '{created_at}');",
        id = user_id,
        is_ai = 0,
        name = "Admin",
        created_at = get_timestamp()
    );
    std::fs::write("/tmp/mahjong_query.sql", player_sql_query).expect("Unable to write file");
    shell.run_status("cd service && sqlite3 mahjong_prod.db < /tmp/mahjong_query.sql");

    std::fs::remove_file("/tmp/mahjong_query.sql").expect("Unable to delete file");

    shell.run_status("cd service && scp mahjong_prod.db mahjong-rust.com:data/mahjong_prod.db");
    shell.run_status("cd scripts && scp docker-compose.yml mahjong-rust.com:");
    shell.run_status("cd scripts && scp -r sql-queries mahjong-rust.com:");

    let ssh_cmd = vec![
        "docker compose down",
        "rm -rf data/mahjong.db",
        "cp data/mahjong_prod.db data/mahjong.db",
        "docker compose pull",
        "docker compose up -d --quiet-pull",
    ];

    shell.run_status(format!("ssh mahjong-rust.com \"{}\"", ssh_cmd.join("; ")).as_str());

    println!("Synced prod");
}
