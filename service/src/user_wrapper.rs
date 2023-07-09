use crate::http_server::StorageData;
use actix_web::{web, HttpResponse};
use service_contracts::{ServicePlayer, UserGetInfoResponse, UserPatchInfoRequest};

pub struct UserWrapper<'a> {
    storage: &'a StorageData,
    player: ServicePlayer,
}

impl<'a> UserWrapper<'a> {
    pub async fn from_storage(
        storage: &'a StorageData,
        player_id: &web::Path<String>,
    ) -> Result<UserWrapper<'a>, HttpResponse> {
        let user = storage.get_player(&player_id.to_string()).await;

        if user.is_err() {
            return Err(HttpResponse::InternalServerError().body("Error loading player"));
        }

        let user_content = user.unwrap();

        if user_content.is_none() {
            return Err(HttpResponse::BadRequest().body("No user found"));
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

    pub async fn get_info(&self) -> HttpResponse {
        let info = self.get_info_data().await;

        HttpResponse::Ok().json(info)
    }

    pub async fn update_info(&mut self, new_data: &UserPatchInfoRequest) -> HttpResponse {
        self.player.name = new_data.name.clone();

        let save_result = self.storage.save_player(&self.player).await;
        let info = self.get_info_data().await;

        match save_result {
            Ok(_) => HttpResponse::Ok().json(info),
            Err(_) => HttpResponse::InternalServerError().body("Error saving player"),
        }
    }
}
