use juniper::{EmptySubscription, FieldResult, RootNode};
use service_contracts::ServicePlayer;

use crate::{auth::GetAuthInfo, http_server::StorageData};

pub struct GraphQLContext {
    pub user_id: String,
    pub storage: StorageData,
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

    pub async fn player_games_ids(ctx: &GraphQLContext) -> FieldResult<Vec<String>> {
        let games = ctx
            .storage
            .get_games_ids(&Some(ctx.user_id.clone()))
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

        if auth_info.username != "test" {
            return Err("Player not test".into());
        }

        let games_ids = ctx
            .storage
            .get_games_ids(&Some(ctx.user_id.clone()))
            .await?;

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
