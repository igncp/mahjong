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

// This script clears the database completely in prod, it is a temporaly approach until the whole
// project is stable
pub fn run_sync_prod(shell: &mut Shell) {
    shell.run_status(
        "docker exec mahjong_db psql -U postgres -c 'DROP DATABASE IF EXISTS mahjong_prod'",
    );
    shell.run_status("docker exec mahjong_db psql -U postgres -c 'CREATE DATABASE mahjong_prod'");
    shell.run_status("cd service && DATABASE_URL='postgres://postgres:postgres@localhost/mahjong_prod' diesel setup");

    let admin_pass = shell
        .run_output("ssh mahjong-rust.com \"cat .env | grep MAHJONG_ADMIN_PASS | cut -d '=' -f2\"");
    let admin_pass = admin_pass.replace('\n', "");

    let salt = Uuid::new_v4().to_string();
    let config = Config::default();
    let hash = argon2::hash_encoded(admin_pass.as_bytes(), salt.as_bytes(), &config).unwrap();
    let user_id = Uuid::new_v4().to_string();
    let role = "\"Admin\"";

    let auth_sql_query = format!(
        "INSERT INTO auth_info (user_id, role, provider) VALUES ('{user_id}', '{role}', 'email');",
        user_id = user_id,
        role = role,
    );
    std::fs::write("/tmp/mahjong_query.sql", auth_sql_query).expect("Unable to write file");
    shell.run_status(
        "docker exec mahjong_db psql -U postgres -d mahjong_prod < /tmp/mahjong_query.sql",
    );

    let auth_email_sql_query = format!(
        "INSERT INTO auth_info_email (user_id, username, hashed_pass) VALUES ('{user_id}', '{username}',  '{hash}');",
        user_id = user_id,
        username = "admin",
        hash = hash
    );
    std::fs::write("/tmp/mahjong_query.sql", auth_email_sql_query).expect("Unable to write file");
    shell.run_status(
        "docker exec mahjong_db psql -U postgres -d mahjong_prod < /tmp/mahjong_query.sql",
    );

    let player_sql_query = format!(
        "INSERT INTO player (id, is_ai, name, created_at) VALUES ('{id}', '{is_ai}', '{name}', '{created_at}');",
        id = user_id,
        is_ai = 0,
        name = "Admin",
        created_at = get_timestamp()
    );
    std::fs::write("/tmp/mahjong_query.sql", player_sql_query).expect("Unable to write file");
    shell.run_status(
        "docker exec mahjong_db psql -U postgres -d mahjong_prod < /tmp/mahjong_query.sql",
    );

    std::fs::remove_file("/tmp/mahjong_query.sql").expect("Unable to delete file");

    shell.run_status(
        "docker exec mahjong_db pg_dump -U postgres -d mahjong_prod > /tmp/mahjong_prod.sql",
    );
    shell.run_status("scp /tmp/mahjong_prod.sql mahjong-rust.com:data/");
    shell.run_status("cd scripts && scp docker-compose.yml mahjong-rust.com:");
    shell.run_status("cd scripts && scp -r sql-queries mahjong-rust.com:");

    let ssh_cmd = [
        "docker compose pull",
        "docker compose down",
        // This needs an entry in sudoers (and special permissions to the file)
        // - sudoers: mahjong ALL=(ALL) NOPASSWD: /home/mahjong/rm_volume.sh
        // - script: chown root:root /home/mahjong/rm_volume.sh
        "sudo /home/mahjong/rm_volume.sh",
        "docker compose up -d db",
        "sleep 5",
        "docker compose exec db psql -U postgres -c 'DROP DATABASE IF EXISTS mahjong'",
        "docker compose exec db psql -U postgres -c 'CREATE DATABASE mahjong'",
        "docker compose exec db psql -U postgres -d mahjong < ./data/mahjong_prod.sql",
        "docker compose up -d --quiet-pull",
        "docker system prune -fa",
        "rm -rf data/mahjong_prod.sql",
    ];

    shell.run_status(format!("ssh mahjong-rust.com \"{}\"", ssh_cmd.join(" && ")).as_str());

    println!("Synced prod");
}
