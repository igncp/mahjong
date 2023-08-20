// @generated automatically by Diesel CLI.

diesel::table! {
    auth_info (username) {
        hashed_pass -> Text,
        role -> Text,
        user_id -> Text,
        username -> Text,
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
        round_claimed_id -> Nullable<Integer>,
        round_dealer_index -> Integer,
        round_index -> Integer,
        round_player_index -> Integer,
        round_wall_tile_drawn -> Nullable<Integer>,
        round_wind -> Text,
        updated_at -> Timestamp,
        version -> Text,
    }
}

diesel::table! {
    game_board (game_id, tile_id) {
        game_id -> Text,
        tile_id -> Integer,
        tile_index -> Integer,
    }
}

diesel::table! {
    game_draw_wall (game_id, tile_id) {
        game_id -> Text,
        tile_id -> Integer,
        tile_index -> Integer,
    }
}

diesel::table! {
    game_hand (game_id, tile_id) {
        concealed -> Integer,
        game_id -> Text,
        player_id -> Text,
        set_id -> Nullable<Text>,
        tile_id -> Integer,
        tile_index -> Integer,
    }
}

diesel::table! {
    game_player (game_id, player_id) {
        game_id -> Text,
        player_id -> Text,
        player_index -> Integer,
    }
}

diesel::table! {
    game_score (game_id, player_id) {
        game_id -> Text,
        player_id -> Text,
        score -> Integer,
    }
}

diesel::table! {
    game_settings (game_id) {
        ai_enabled -> Integer,
        auto_sort_players -> Text,
        discard_wait_ms -> Nullable<Integer>,
        fixed_settings -> Integer,
        game_id -> Text,
        last_discard_time -> BigInt,
    }
}

diesel::table! {
    player (id) {
        created_at -> Timestamp,
        id -> Text,
        is_ai -> Integer,
        name -> Text,
    }
}

diesel::joinable!(game_player -> game (game_id));
diesel::joinable!(game_player -> player (player_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth_info,
    game,
    game_board,
    game_draw_wall,
    game_hand,
    game_player,
    game_score,
    game_settings,
    player,
);
