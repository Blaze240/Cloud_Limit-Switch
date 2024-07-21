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

unsafe extern "C" fn effect_attacks4_limit(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_smash_flash"),
            Hash40::new("haver"),
            -2,
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
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(
            agent.module_accessor,
            *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK,
        ) {
            macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.5);
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("cloud_smash_slash_limit"),
                Hash40::new("top"),
                0,
                0,
                0,
                0,
                0,
                0,
                1,
                true,
            );
            EffectModule::set_disable_render_offset_last(agent.module_accessor);
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
            macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.5);
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("cloud_smash_slash"),
                Hash40::new("top"),
                0,
                0,
                0,
                0,
                0,
                0,
                1,
                true,
            );
            EffectModule::set_disable_render_offset_last(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 20.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(
                agent,
                Hash40::new("sys_atk_smoke"),
                Hash40::new("top"),
                0,
                0,
                0,
                0,
                0,
                0,
                0.8,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
        }
        frame(agent.lua_state_agent, 21.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND_WORK(
                agent,
                *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE,
                true,
                true,
            );
        }
        frame(agent.lua_state_agent, 24.0);
        if macros::is_excute(agent) {
            if WorkModule::is_flag(
                agent.module_accessor,
                *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK,
            ) {
                macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 1.5);
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
                macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 1.5);
            }
        }
        frame(agent.lua_state_agent, 27.0);
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND_WORK(
                agent,
                *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE,
                true,
                true,
            );
        }
        frame(agent.lua_state_agent, 28.0);
        if macros::is_excute(agent) {
            if WorkModule::is_flag(
                agent.module_accessor,
                *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK,
            ) {
                macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.2);
                macros::LANDING_EFFECT(
                    agent,
                    Hash40::new("sys_atk_smoke"),
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
                macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.2);
                macros::LANDING_EFFECT(
                    agent,
                    Hash40::new("sys_atk_smoke"),
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
            frame(agent.lua_state_agent, 31.0);
            if macros::is_excute(agent) {
                macros::EFFECT_OFF_KIND_WORK(
                    agent,
                    *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE,
                    false,
                    true,
                );
            }
        }
    }
}

pub fn install() {
    Agent::new("cloud")
        .effect_acmd("effect_attacks4_limit", effect_attacks4_limit, Priority::Low)
        .install();
}
