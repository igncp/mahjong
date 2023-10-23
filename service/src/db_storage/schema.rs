// @generated automatically by Diesel CLI.

diesel::table! {
    auth_info (user_id) {
        provider -> Text,
        role -> Text,
        user_id -> Text,
    }
}

diesel::table! {
    auth_info_email (user_id) {
        hashed_pass -> Text,
        user_id -> Text,
        username -> Text,
    }
}

diesel::table! {
    auth_info_github (user_id) {
        token -> Nullable<Text>,
        user_id -> Text,
        username -> Text,
    }
}

diesel::table! {
    auth_info_providers (id) {
        id -> Text,
    }
}

diesel::table! {
    game (id) {
        created_at -> Timestamp,
        id -> Text,
        name -> Text,
        phase -> Text,
        round_claimed_by -> Nullable<Text>,
        round_claimed_from -> Nullable<Text>,
        round_claimed_id -> Nullable<Int4>,
        round_dealer_index -> Int4,
        round_index -> Int4,
        round_player_index -> Int4,
        round_wall_tile_drawn -> Nullable<Int4>,
        round_wind -> Text,
        updated_at -> Timestamp,
        version -> Text,
    }
}

diesel::table! {
    game_board (game_id, tile_id) {
        game_id -> Text,
        tile_id -> Int4,
        tile_index -> Int4,
    }
}

diesel::table! {
    game_draw_wall (game_id, tile_id) {
        game_id -> Text,
        tile_id -> Int4,
        tile_index -> Int4,
    }
}

diesel::table! {
    game_hand (game_id, tile_id) {
        concealed -> Int4,
        game_id -> Text,
        player_id -> Text,
        set_id -> Nullable<Text>,
        tile_id -> Int4,
        tile_index -> Int4,
    }
}

diesel::table! {
    game_player (game_id, player_id) {
        game_id -> Text,
        player_id -> Text,
        player_index -> Int4,
    }
}

diesel::table! {
    game_score (game_id, player_id) {
        game_id -> Text,
        player_id -> Text,
        score -> Int4,
    }
}

diesel::table! {
    game_settings (game_id) {
        ai_enabled -> Int4,
        auto_sort_players -> Text,
        auto_stop_claim_meld -> Text,
        discard_wait_ms -> Nullable<Int4>,
        fixed_settings -> Int4,
        game_id -> Text,
        last_discard_time -> Int8,
    }
}

diesel::table! {
    player (id) {
        created_at -> Timestamp,
        id -> Text,
        is_ai -> Int4,
        name -> Text,
    }
}

diesel::joinable!(auth_info -> auth_info_providers (provider));
diesel::joinable!(auth_info -> player (user_id));
diesel::joinable!(auth_info_email -> auth_info (user_id));
diesel::joinable!(auth_info_github -> auth_info (user_id));
diesel::joinable!(game_board -> game (game_id));
diesel::joinable!(game_draw_wall -> game (game_id));
diesel::joinable!(game_hand -> game (game_id));
diesel::joinable!(game_hand -> player (player_id));
diesel::joinable!(game_player -> game (game_id));
diesel::joinable!(game_player -> player (player_id));
diesel::joinable!(game_score -> game (game_id));
diesel::joinable!(game_score -> player (player_id));
diesel::joinable!(game_settings -> game (game_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth_info,
    auth_info_email,
    auth_info_github,
    auth_info_providers,
    game,
    game_board,
    game_draw_wall,
    game_hand,
    game_player,
    game_score,
    game_settings,
    player,
);
