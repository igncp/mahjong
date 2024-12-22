use crate::{
    http_server::DataStorage,
    service_error::{ResponseCommon, ServiceError},
};
use actix_web::HttpResponse;
use service_contracts::{
    AuthInfoSummary, DashboardGame, DashboardPlayer, ServicePlayer, UserGetDashboardResponse,
    UserGetInfoResponse, UserPatchInfoRequest,
};

pub struct UserWrapper<'a> {
    storage: &'a DataStorage,
    player: ServicePlayer,
}

impl<'a> UserWrapper<'a> {
    pub async fn from_storage(
        storage: &'a DataStorage,
        player_id: &String,
    ) -> Result<UserWrapper<'a>, ServiceError> {
        let user = storage.get_player(&player_id.to_string()).await;

        if user.is_err() {
            return Err(ServiceError::Custom("Error loading the user"));
        }

        let user_content = user.unwrap();

        if user_content.is_none() {
            return Err(ServiceError::Custom("User not found"));
        }

        let player = user_content.unwrap();

        Ok(Self { storage, player })
    }

    async fn get_info_data(&self) -> Option<UserGetInfoResponse> {
        let total_score = self.storage.get_player_total_score(&self.player.id).await;

        if total_score.is_err() {
            return None;
        }

        let info = UserGetInfoResponse {
            name: self.player.name.clone(),
            total_score: total_score.unwrap(),
        };

        Some(info)
    }

    pub async fn get_info(&self) -> ResponseCommon {
        let info = self.get_info_data().await;

        Ok(HttpResponse::Ok().json(info))
    }

    pub async fn get_dashboard(
        &self,
        auth_info_summary: &AuthInfoSummary,
    ) -> Result<UserGetDashboardResponse, ServiceError> {
        let player = DashboardPlayer {
            created_at: self.player.created_at.clone(),
            id: self.player.id.clone(),
            name: self.player.name.clone(),
        };

        let info = self.get_info_data().await;

        let games = self
            .storage
            .get_player_games(&Some(self.player.id.clone()))
            .await;

        if games.is_err() {
            return Err(ServiceError::Custom("Error loading player games"));
        }

        let games = games.unwrap();

        let player_games = games
            .into_iter()
            .map(|game| DashboardGame {
                created_at: game.created_at,
                id: game.id,
                updated_at: game.updated_at,
            })
            .collect();

        Ok(UserGetDashboardResponse {
            auth_info: auth_info_summary.clone(),
            player,
            player_games,
            player_total_score: info.unwrap().total_score,
        })
    }

    pub async fn update_info(&mut self, new_data: &UserPatchInfoRequest) -> ResponseCommon {
        self.player.name.clone_from(&new_data.name);

        let info = self.get_info_data().await;

        self.storage.save_player(&self.player).await.map_or(
            Err(ServiceError::Custom("Error saving player").into()),
            |_| Ok(HttpResponse::Ok().json(info)),
        )
    }
}
