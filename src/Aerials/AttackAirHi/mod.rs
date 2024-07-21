use {
    smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        hash40,
        lib::{lua_const::*, L2CAgent, L2CValue},
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::{Priority::*, *},
};
unsafe extern "C" fn effect_attackairhi_fusion(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(
            agent.module_accessor,
            *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK,
        ) {
            macros::AFTER_IMAGE4_ON_arg29(
                agent,
                Hash40::new("tex_cloud_sword3_limit"),
                Hash40::new("tex_cloud_sword4"),
                3,
                Hash40::new("haver"),
                0,
                1.5,
                -1.2,
                Hash40::new("haver"),
                0,
                20.5,
                -1.2,
                true,
                Hash40::new("null"),
                Hash40::new("haver"),
                0,
                0,
                0,
                0,
                0,
                0,
                1,
                0,
                *EFFECT_AXIS_X,
                0,
                *TRAIL_BLEND_ALPHA,
                101,
                *TRAIL_CULL_NONE,
                1.4,
                0.1,
            );
        } else {
            macros::AFTER_IMAGE4_ON_arg29(
                agent,
                Hash40::new("tex_cloud_sword3"),
                Hash40::new("tex_cloud_sword4"),
                3,
                Hash40::new("haver"),
                0,
                1.5,
                -1.2,
                Hash40::new("haver"),
                0,
                20.5,
                -1.2,
                true,
                Hash40::new("null"),
                Hash40::new("haver"),
                0,
                0,
                0,
                0,
                0,
                0,
                1,
                0,
                *EFFECT_AXIS_X,
                0,
                *TRAIL_BLEND_ALPHA,
                101,
                *TRAIL_CULL_NONE,
                1.4,
                0.1,
            );
        }
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(
            agent.module_accessor,
            *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK,
        ) {
            macros::EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.14);
        } else {
            macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.14);
        }
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND_WORK(
            agent,
            *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB,
            false,
            true,
        );
    }
}

pub fn install() {
    Agent::new("cloud")
        .effect_acmd(
            "effect_attackairhi_fusion",
            effect_attackairhi_fusion,
            Priority::Low,
        )
        .install();
}
