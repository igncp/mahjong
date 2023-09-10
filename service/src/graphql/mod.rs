use crate::{
    auth::{AuthInfoData, GetAuthInfo},
    graphql::gql_game::GraphQLServiceGameSummary,
    http_server::DataStorage,
};
use juniper::{EmptySubscription, FieldResult, RootNode};
use mahjong_core::GameId;
use service_contracts::{ServiceGameSummary, ServicePlayer, ServicePlayerGame};

mod gql_game;

pub struct GraphQLContext {
    pub user_id: String,
    pub storage: DataStorage,
}

impl juniper::Context for GraphQLContext {}

#[derive(Clone, Copy, Debug)]
pub struct GraphQLQuery;

#[derive(Clone, Copy, Debug)]
pub struct GraphQLMutation;

#[juniper::graphql_object(context = GraphQLContext)]
impl GraphQLQuery {
    pub async fn player(ctx: &GraphQLContext) -> FieldResult<ServicePlayer> {
        let player = ctx.storage.get_player(&ctx.user_id).await?;

        if player.is_none() {
            return Err("Player not found".into());
        }

        Ok(player.unwrap())
    }

    pub async fn game(ctx: &GraphQLContext, id: GameId) -> FieldResult<GraphQLServiceGameSummary> {
        let game = ctx.storage.get_game(&id).await?;

        if game.is_none() {
            return Err("Game not found".into());
        }

        let game = game.unwrap();

        let game_summary = ServiceGameSummary::from_service_game(&game, &ctx.user_id);

        if game_summary.is_none() {
            return Err("Invalid game".into());
        }

        let service_game_summary = game_summary.unwrap();
        let gql_game = GraphQLServiceGameSummary::from_service_game_summary(&service_game_summary);

        Ok(gql_game)
    }

    pub async fn player_games(ctx: &GraphQLContext) -> FieldResult<Vec<ServicePlayerGame>> {
        let games = ctx
            .storage
            .get_player_games(&Some(ctx.user_id.clone()))
            .await?;

        Ok(games)
    }

    pub async fn player_total_score(ctx: &GraphQLContext) -> FieldResult<i32> {
        let total_score = ctx.storage.get_player_total_score(&ctx.user_id).await;

        if total_score.is_err() {
            return Err("Player not found".into());
        }

        Ok(total_score.unwrap())
    }
}

#[juniper::graphql_object(context = GraphQLContext)]
impl GraphQLMutation {
    pub async fn test_delete_games(ctx: &GraphQLContext) -> FieldResult<bool> {
        // If deleting for normal users, should check if any active running at the moment by using
        // the web socket

        let auth_info = ctx
            .storage
            .get_auth_info(GetAuthInfo::PlayerId(ctx.user_id.clone()))
            .await?;

        if auth_info.is_none() {
            return Err("Player not found".into());
        }

        let auth_info = auth_info.unwrap();

        if let AuthInfoData::Email(auth_info_email) = auth_info.data {
            if auth_info_email.username != "test" {
                return Err("Player not test".into());
            }
        } else {
            return Err("Player not found".into());
        }

        let games = ctx
            .storage
            .get_player_games(&Some(ctx.user_id.clone()))
            .await?;
        let games_ids: Vec<_> = games.iter().map(|g| g.id.clone()).collect();

        let result = ctx.storage.delete_games(&games_ids).await;

        Ok(result.is_ok())
    }
}

pub type Schema =
    RootNode<'static, GraphQLQuery, GraphQLMutation, EmptySubscription<GraphQLContext>>;

pub fn create_schema() -> Schema {
    Schema::new(
        GraphQLQuery,
        GraphQLMutation,
        EmptySubscription::<GraphQLContext>::new(),
    )
}
