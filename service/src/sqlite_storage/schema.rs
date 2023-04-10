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
        id -> Text,
        name -> Text,
        phase -> Text,
        round_claimed_by -> Nullable<Text>,
        round_claimed_from -> Nullable<Text>,
        round_claimed_id -> Nullable<Integer>,
        round_dealer_index -> Integer,
        round_player_index -> Integer,
        round_wall_tile_drawn -> Nullable<Integer>,
        round_wind -> Text,
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
    player (id) {
        ai_enabled -> Integer,
        id -> Text,
        is_ai -> Integer,
        name -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    auth_info,
    game,
    game_board,
    game_draw_wall,
    game_hand,
    game_player,
    game_score,
    player,
);
