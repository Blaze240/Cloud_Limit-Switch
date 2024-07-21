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

unsafe extern "C" fn effect_specialairs2_fusion(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("cloud_kyogiri_stroke1_l"), true, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("cloud_kyogiri_stroke1_r"), true, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.79, 1.1, 0.4);
        macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.4);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1_yellow"), Hash40::new("tex_cloud_sword2"), 3, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT(
                agent,
                Hash40::new("cloud_kyogiri_stroke2_l"),
                Hash40::new("top"),
                0,
                11,
                17,
                0,
                100,
                0,
                1.15,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
        }
    } else {
        if macros::is_excute(agent) {
            macros::EFFECT(
                agent,
                Hash40::new("cloud_kyogiri_stroke2_r"),
                Hash40::new("top"),
                0,
                11,
                17,
                0,
                -100,
                0,
                1.15,
                0,
                0,
                0,
                0,
                0,
                0,
                true,
            );
        }
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, true, true);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(
            agent,
            Hash40::new("cloud_kyogiri_stroke2_l"),
            false,
            false,
        );
        macros::EFFECT_OFF_KIND(
            agent,
            Hash40::new("cloud_kyogiri_stroke2_r"),
            false,
            false,
        );
    }
}

pub fn install() {
    Agent::new("cloud")
        .effect_acmd(
            "effect_specialairs2_fusion",
            effect_specialairs2_fusion,
            Priority::Low,
        )
        .install();
}
