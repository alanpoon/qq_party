use bevy::prelude::*;
use bevy::utils::Duration;
use qq_party_shared::*;
use crate::PlayerHealthCheckTimer;
use crate::c_;
pub fn player_health_check(mut health_check_timer:Query<&mut PlayerHealthCheckTimer>,mut commands: ResMut<protocol::Commands>,time:Res<Time>,local_user_info:Res<LocalUserInfo>){
    
    for mut timer in health_check_timer.iter_mut(){
        //info!("has health_check_timer");
        let ball_id = local_user_info.0.ball_id;
        if ball_id.0!=0{
            (*timer).0.tick(time.delta());
            if (*timer).0.just_finished() {
                info!("has ping {:?}",ball_id.0);
                let c = c_::ping(ball_id.0);
                commands.push(c);
            }
        }
       
    }
}