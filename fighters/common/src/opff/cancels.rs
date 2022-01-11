use crate::opff_import::*;
use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::hash40;


//=================================================================
//== JUMP CANCEL GRABS
//=================================================================
unsafe fn jump_cancel_grab(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, fighter_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
        if boma.is_cat_flag(Cat1::WallJumpRight) {
            if fighter_kind == *FIGHTER_KIND_POPO {
                VarModule::on_flag(get_battle_object_from_accessor(boma), vars::common::POPO_JC_GRAB);
            }
            WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH, true);
        }
    }
}

//=================================================================
//== AIRDODGE CANCEL ZAIR AND ITEM TOSS
//=================================================================
unsafe fn airdodge_cancels(boma: &mut BattleObjectModuleAccessor, cat2: i32, cat3: i32, status_kind: i32, fighter_kind: i32, facing: f32, stick_x: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
        if MotionModule::frame(boma) > 3.0 && MotionModule::frame(boma) < 41.0 {
            // Throw item
            if ItemModule::is_have_item(boma, 0) {
                if boma.is_cat_flag(Cat3::ItemLightThrowAirAll) {
                    if facing * stick_x < 0.0 {
                        PostureModule::reverse_lr(boma);
                    }
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ITEM_THROW, false);
                }
            } else { // Zair if no item toss
                if [*FIGHTER_KIND_LUCAS,
                    *FIGHTER_KIND_YOUNGLINK,
                    *FIGHTER_KIND_TOONLINK,
                    *FIGHTER_KIND_SAMUS,
                    *FIGHTER_KIND_SAMUSD,
                    *FIGHTER_KIND_SZEROSUIT,
                    *FIGHTER_KIND_LUIGI].contains(&fighter_kind) {
                    if !ItemModule::is_have_item(boma, 0) {
                       if boma.is_cat_flag(Cat2::AirLasso) {
                           StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_AIR_LASSO, true);
                       }
                    }
                }
            }
        }
    }
}

//=================================================================
//== DITCIT
//=================================================================
unsafe fn ditcit(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, facing: f32) {
    let player_number = hdr::get_player_number(boma);
    let mut motion_value: f32 = 0.0;
    let mut motion_vec = Vector3f {x: 0.0, y: 0.0, z: 0.0};

    if status_kind != *FIGHTER_STATUS_KIND_ITEM_THROW {
        VarModule::off_flag(get_battle_object_from_accessor(boma), vars::common::DITCIT_SLIDING);
    }

    if status_kind == *FIGHTER_STATUS_KIND_ITEM_THROW_DASH {
        if MotionModule::frame(boma) > 2.0 && MotionModule::frame(boma) < 6.0
            && ((boma.is_cat_flag(Cat1::AttackHi4))
             || (boma.is_cat_flag(Cat1::AttackLw4))
             || (boma.is_cat_flag(Cat1::AttackS4))
             || (boma.is_cat_flag(Cat1::AttackHi3))
             || (boma.is_cat_flag(Cat1::AttackLw3))
             || (boma.is_cat_flag(Cat1::AttackS3))) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ITEM_THROW, false);
            VarModule::on_flag(get_battle_object_from_accessor(boma), vars::common::DITCIT_SLIDING);
        }
    } else {
        if VarModule::is_flag(get_battle_object_from_accessor(boma), vars::common::DITCIT_SLIDING) {  // status_kind == ITEM_THROWN, coming from THROW_DASH
            motion_value = 2.8 * (MotionModule::end_frame(boma) - MotionModule::frame(boma)) / MotionModule::end_frame(boma);
            motion_vec.x = motion_value * facing;
            motion_vec.y = 0.0;
            motion_vec.z = 0.0;
            KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        }
    }
}

//=================================================================
//== DACUS
//=================================================================
unsafe fn dacus(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, stick_y: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
        if MotionModule::frame(boma) < 10.0 {
            let is_catch = boma.is_cat_flag(Cat1::WallJumpRight) || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_CATCH);

            // Normal smash input or Z with left stick
            if boma.is_cat_flag(Cat1::AttackHi4) || (stick_y >= 0.7 && is_catch) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
            }

            if boma.is_cat_flag(Cat1::AttackLw4) || (stick_y <= -0.7 && is_catch) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
            }

            // Adjust input window of tilts to prevent accidental smashes
            if MotionModule::frame(boma) > 2.0 {
                if boma.is_cat_flag(Cat1::AttackHi3) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
                }
                if boma.is_cat_flag(Cat1::AttackLw3) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
                }
            }
        }
    }
}

//=================================================================
//== JUMP CANCEL AIRDODGE
//=================================================================
unsafe fn jump_cancel_airdodge(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, fighter_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
        if boma.is_cat_flag(Cat1::JumpButton) && !boma.is_cat_flag(Cat1::WallJumpRight) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
        }
    }
}

//=================================================================
//== ANTI-FOOTSTOOL DEGENERACY TECH
//=================================================================
unsafe fn footstool_defense(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    // Shield cancel grounded footstool recoil after being ground footstooled and then receiving
    // histun beforehand
    let prev_status_0 = StatusModule::prev_status_kind(boma, 0);
    let prev_status_1 = StatusModule::prev_status_kind(boma, 1);
    let prev_status_2 = StatusModule::prev_status_kind(boma, 2);
    let prev_status_3 = StatusModule::prev_status_kind(boma, 3);
    if (status_kind == *FIGHTER_STATUS_KIND_TREAD_DAMAGE_RV && situation_kind == *SITUATION_KIND_GROUND)
        && (prev_status_1 == *FIGHTER_STATUS_KIND_DAMAGE)
          || (prev_status_2 == *FIGHTER_STATUS_KIND_DAMAGE_AIR && prev_status_1 == *FIGHTER_STATUS_KIND_DAMAGE)
          || (prev_status_3 == *FIGHTER_STATUS_KIND_DAMAGE && prev_status_2 == *FIGHTER_STATUS_KIND_DAMAGE_AIR && prev_status_1 == *FIGHTER_STATUS_KIND_DAMAGE) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
            if situation_kind == *SITUATION_KIND_GROUND {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
            }
        }
    }

    if status_kind == *FIGHTER_STATUS_KIND_TREAD_DAMAGE_RV {
        // DamageModule::add_damage(boma, 100.0, 0);
    }

    let player_number = hdr::get_player_number(boma);

    // Prevent airdodging after a footstool until after F20
    if (status_kind == *FIGHTER_STATUS_KIND_JUMP && prev_status_0 == *FIGHTER_STATUS_KIND_TREAD_JUMP)
        || (status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL && prev_status_0 == *FIGHTER_STATUS_KIND_JUMP && prev_status_1 == *FIGHTER_STATUS_KIND_TREAD_JUMP)
        && MotionModule::frame(boma) < 20.0 {
        VarModule::on_flag(get_battle_object_from_accessor(boma), vars::common::FOOTSTOOL_AIRDODGE_LOCKOUT);
    } else if VarModule::is_flag(get_battle_object_from_accessor(boma), vars::common::FOOTSTOOL_AIRDODGE_LOCKOUT) {
        VarModule::off_flag(get_battle_object_from_accessor(boma), vars::common::FOOTSTOOL_AIRDODGE_LOCKOUT);
    }
}



pub unsafe fn run(boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    //jump_cancel_airdodge(boma, cat[0], status_kind, fighter_kind); // experimental, must be called before jcgrab
    // jump_cancel_grab(boma, cat[0], status_kind, fighter_kind);
    // airdodge_cancels(boma, cat[1], cat[2], status_kind, fighter_kind, facing, stick_x);
    ditcit(boma, cat[0], status_kind, facing); // original = ditcit(boma, cat1, status_kind, motion_value, motion_vec, facing);
    //dacus(boma, cat[0], status_kind, stick_y);
    footstool_defense(boma, status_kind, situation_kind);
}
