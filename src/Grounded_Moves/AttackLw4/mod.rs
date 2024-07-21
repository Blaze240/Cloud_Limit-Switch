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

unsafe extern "C" fn effect_attacklw4_limit(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_smash_flash"),
            Hash40::new("haver"),
            0,
            10,
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
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("sys_atk_smoke"),
            Hash40::new("top"),
            4,
            0,
            0,
            0,
            0,
            0,
            0.55,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
        agent.clear_lua_stack();
        lua_args!(
            agent,
            Hash40::new("cloud_speedline"),
            Hash40::new("top"),
            -1,
            3.5,
            2,
            0,
            0,
            0,
            0.8,
            true,
            0.396,
            0.475,
            2,
        );
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
        sv_animcmd::EFFECT_OFF(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_attack_impact"),
            Hash40::new("top"),
            0,
            4.5,
            15,
            0,
            0,
            0,
            1.2,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(
            agent.module_accessor,
            *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK,
        ) {
            macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.1);
            agent.clear_lua_stack();
            lua_args!(
                agent,
                Hash40::new("cloud_speedline"),
                Hash40::new("top"),
                0,
                4.5,
                -2,
                0,
                180,
                4,
                0.85,
                true,
                0.492,
                0.869,
                0.669,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 1.2);
            sv_animcmd::EFFECT_OFF(agent.lua_state_agent);
        } else {
            macros::EFFECT_FOLLOW_WORK(
                agent,
                *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE,
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
            macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.1);
            agent.clear_lua_stack();
        lua_args!(
                agent,
                Hash40::new("cloud_speedline"),
                Hash40::new("top"),
                0,
                4.5,
                -2,
                0,
                180,
                4,
                0.85,
                true,
                0.492,
                0.869,
                0.669,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 1.2);
        }
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("sys_atk_smoke"),
            Hash40::new("top"),
            4,
            0,
            0,
            0,
            180,
            0,
            0.55,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND_WORK(
            agent,
            *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE,
            false,
            true,
        );
    }
}

pub fn install() {
    Agent::new("cloud")
        .effect_acmd("effect_attacklw4_limit", effect_attacklw4_limit, Priority::Low)
        .install();
}
