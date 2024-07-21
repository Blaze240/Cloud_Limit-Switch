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

unsafe extern "C" fn effect_attackhi3_fusion(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(
            agent.module_accessor,
            *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK,
        ) {
            macros::EFFECT_FOLLOW_WORK(
                agent,
                *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB,
                Hash40::new("haver"),
                0,
                0,
                0,
                0,
                0,
                0,
                1,
                true,
            );
            macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.39);
            macros::AFTER_IMAGE4_ON_arg29(
                agent,
                Hash40::new("tex_cloud_sword1_blue"),
                Hash40::new("tex_cloud_sword2"),
                5,
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
            macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.39);
            macros::AFTER_IMAGE4_ON_arg29(
                agent,
                Hash40::new("tex_cloud_sword1"),
                Hash40::new("tex_cloud_sword2"),
                5,
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
        macros::FOOT_EFFECT(
            agent,
            Hash40::new("sys_run_smoke"),
            Hash40::new("top"),
            -4,
            0,
            0,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 11.0);
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
            "effect_attackhi3_fusion",
            effect_attackhi3_fusion,
            Priority::Low,
        )
        .install();
}
