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

unsafe extern "C" fn effect_attackdash_fusion(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(
            agent,
            Hash40::new("sys_dash_smoke"),
            Hash40::new("top"),
            -1,
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
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(
            agent,
            Hash40::new("cloud_speedline"),
            Hash40::new("top"),
            0,
            7,
            0,
            0,
            0,
            0,
            0.8,
            true,
            0.627,
            1,
            0.674
        );
    }
    frame(agent.lua_state_agent, 9.0);
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
            macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.3);
        } else {
            macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.3);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(
            agent.module_accessor,
            *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK,
        ) {
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("cloud_dash_slash_limit"),
                Hash40::new("top"),
                1.5,
                1.5,
                6,
                0,
                0,
                0,
                1,
                true,
            );
            macros::FOOT_EFFECT(
                agent,
                Hash40::new("sys_run_smoke"),
                Hash40::new("top"),
                8,
                0,
                0,
                0,
                0,
                0,
                0.9,
                0,
                0,
                0,
                0,
                0,
                0,
                false,
            );
        } else {
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("cloud_dash_slash"),
                Hash40::new("top"),
                1.5,
                1.5,
                6,
                0,
                0,
                0,
                1,
                true,
            );
            macros::FOOT_EFFECT(
                agent,
                Hash40::new("sys_run_smoke"),
                Hash40::new("top"),
                8,
                0,
                0,
                0,
                0,
                0,
                0.9,
                0,
                0,
                0,
                0,
                0,
                0,
                false,
            );
        }
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("cloud_speedline"), false, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("sys_run_smoke"),
            Hash40::new("top"),
            8,
            0,
            0,
            0,
            0,
            0,
            0.9,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(
            agent,
            Hash40::new("sys_run_smoke"),
            Hash40::new("top"),
            8,
            0,
            0,
            0,
            0,
            0,
            0.9,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
    }
    frame(agent.lua_state_agent, 19.0);
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
            "effect_attackdash_fusion",
            effect_attackdash_fusion,
            Priority::Low,
        )
        .install();
}
