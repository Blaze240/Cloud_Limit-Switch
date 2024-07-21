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

unsafe extern "C" fn effect_attackairlw_fusion(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_smash_flash"),
            Hash40::new("top"),
            0,
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
            true,
        );
    }
    frame(agent.lua_state_agent, 10.0);
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
            macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.5);
        } else {
            macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.5);
        }
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(
            agent,
            Hash40::new("cloud_speedline"),
            Hash40::new("haver"),
            0,
            4,
            0,
            -90,
            0,
            0,
            0.8,
            true,
            0.627,
            1,
            0.674,
        );
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND_WORK(
            agent,
            *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE_LB,
            true,
            true,
        );
    }
}

pub fn install() {
    Agent::new("cloud")
        .effect_acmd(
            "effect_attackairlw_fusion",
            effect_attackairlw_fusion,
            Priority::Low,
        )
        .install();
}
