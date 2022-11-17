use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;
use crate::utils::*;
use smashline::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::hash40;
use smash::phx::{Vector3f, Hash40};
use smash_script::*;

pub const PI : f64 = 3.14159265358979323846264338327950288;
static mut ANGLE : [f32; 8] = [0.0; 8];

static ANGLE_MAX_UP : f32 = 80.0; //#0 Max Upward Angle
static ANGLE_MAX_DOWN : f32 = -70.0; //#1 Max Downward Angle
static V_GLIDE_START : f32 = 0.75; //#2 V Speed added on GlideStart
static GRAVITY_START : f32 = 1.0; //#3 Gravity multiplier on GlideStart
static SPEED_MUL_START : f32 = 1.0 //#4 H speed multiplier on GlideStart
static BASE_SPEED : f32 = 1.7; //#5 Base Power/Speed
static SPEED_CHANGE : f32 = 0.04; //#6 Change of Speed
static MAX_SPEED : f32 = 2.2; //#7 Maximum Speed
static THRESHOLD : f32 = 0.7; //#8 Another speed threshold
static GRAVITY_ACCEL : f32 = 0.03; //#9 Gravity Acceleration
static GRAVITY_SPEED : f32 = 0.6; //#10 Gravity Max Speed
static ANGLE_EXTRA : f32 = 15.0; //#11 Angle stuff but unknown what this is for
static ANGLE_F_SPEED : f32 = -25.0; //#12 Angle to gain forward speed
static DOWN_SPEED_ADD : f32 = 0.03; //#13 Max added H speed gained aiming downward
static UNKNOWN : f32 = 0.15; //#14 Unknown
static RADIAL_STICK : f32 = 0.25; //#15 Radial Stick Sensitivity
static UP_ANGLE_ACCEL : f32 = 0.55; //#16 Upward angular acceleration
static DOWN_ANGLE_ACCEL : f32 = 0.75; //#17 Downward angular acceleration
static MAX_ANGLE_SPEED : f32 = 7.0; //#18 Maximum angular speed
static ADD_ANGLE_SPEED : f32 : 1.0; //#19 Added angular speed for when stick is center

#[status_script(agent = "metaknight", status = FIGHTER_STATUS_KIND_GLIDE_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn glide_start_a(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_start"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GLIDE_START);
    fighter.sub_shift_status_main(L2CValue::Ptr(glide_start_b as *const () as _))
}

unsafe extern "C" fn glide_start_b(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("glide_start") && MotionModule::is_end(fighter.module_accessor) {   
        fighter.change_status(FIGHTER_STATUS_KIND_GLIDE.into(), false.into());
    }
    L2CValue::I32(0)
}

//Init Status stuff from Brawl could go here? WIP
#[status_script(agent = "metaknight", status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn glide_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, BASE_H_SPEED, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_POWER);
    WorkModule::set_float(fighter.module_accessor, GRAVITY_SPEED, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GRAVITY);
    
    KineticEnergy::reset_energy(as *mut KineticEnergy, /*insert lua const here*/, &Vector2f{x: x y: x}, &Vector3f{x: x y: x, z: x}, fighter.module_accessor);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_/*SOMETHING*/ );
    L2CValue::I32(0)
}

//Exec Status stuff from Brawl
#[status_script(agent = "metaknight", status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn glide_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_direction"), 90.0, 0.0, true, 0.0, false, false);
    MotionModule::add_motion_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, Hash40::new("glide_wing"), 0.0, 1.0, true, false, 0.0, false, true, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(glide_exec_main as *const () as _))
}

unsafe extern "C" fn glide_exec_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    /*get_kinetic_energy_descriptor in KineticModule must be called something else in Ultimate since that name doesn't exist. 
    There is KineticModule::get_energy(fighter.module_accessor, "put lua_const here"); but I don't know if that's what we want*/

    let angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
    let angle_speed = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE_SPEED);
    let stick_angle = ControlModule::get_stick_angle(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let stick_magnitude = (stick_x * stick_x + stick_y * stick_y).sqrt();
    let new_angle_speed = angle_speed + ADD_ANGLE_SPEED;
    let angle_accel = ??? //What is this?
    let scaled_angle_accel = angle_accel * (stick_magnitude - RADIAL_STICK) / (1.0 - RADIAL_STICK);
    let new_angle_speed_2nd = angle_speed + scaled_angle_accel;

    if lr <= 0.0 {
        let above_or_below = -1.0
        if stick_angle > 0.0 {
            above_or_below = 1.0;
        }
        stick_angle = (180.0 * above_or_below) - (stick_angle * 180.0 / PI);
    }
    else {
        stick_angle = stick_angle * 180.0 / PI;
    }
    if stick_magnitude <= RADIAL_STICK {
        if WorkModule::on_flag(fighter.module_accessor, FIGHTER_STATUS_GLIDE_FLAG_STOP) {
            if angle_speed < 0.0 {
                angle_speed = 0.0;
            } 
            if new_angle_speed < -MAX_ANGLE_SPEED {
                new_angle_speed = -MAX_ANGLE_SPEED;
            }
            if new_angle_speed > MAX_ANGLE_SPEED {
                new_angle_speed = MAX_ANGLE_SPEED;
            }
            WorkModule::set_float(fighter.module_accessor, new_angle_speed, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE_SPEED);
            angle = angle + new_angle_speed;
        }
        else {
            if stick_angle < 0.0 {
                if stick_angle >= -135.0 {
                    angle_accel = -DOWN_ANGLE_ACCEL; //What is angle_accel here?
                }
                else {
                    angle_accel = UP_ANGLE_ACCEL;
                }   
            }
            else {
                if stick_angle >= 45.0 {
                    angle_accel = UP_ANGLE_ACCEL;
                }
                else {
                    angle_accel = -DOWN_ANGLE_ACCEL;
                }
            }  
            if angle_speed * scaled_angle_accel < 0.0 {
                angle_speed = 0.0;
            }
            if new_angle_speed_2nd < -MAX_ANGLE_SPEED {
                new_angle_speed_2nd = -MAX_ANGLE_SPEED;
            }
            if new_angle_speed_2nd > MAX_ANGLE_SPEED {
                MAX_ANGLE_SPEED = new_angle_speed_2nd;
            }
            WorkModule::set_float(fighter.module_accessor, MAX_ANGLE_SPEED, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE_SPEED);
            angle = angle + MAX_ANGLE_SPEED;
        }
        if angle < ANGLE_MAX_DOWN {
            angle = ANGLE_MAX_DOWN;
        }
        if angle > ANGLE_MAX_UP {
            angle = ANGLE_MAX_UP;
        }
        if WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_STOP) {
            //WIP
        }
        else {
            //WIP
        }
    }
    MotionModule::set_frame(fighter.module_accessor, 90.0 - angle, false);
    if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK) {
        fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_ATTACK.into(), true.into());
    }
    if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_END.into(), true.into());
    }
    if is_grounded(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_LANDING.into(), true.into());
    }
    0.into()
}

#[status_script(agent = "metaknight", status = FIGHTER_STATUS_KIND_GLIDE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn glide_finish(fighter: &mut L2CFighterCommon) -> L2CValue {
    let angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
    angle = 0.0;
    MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, false);
    L2CValue::I32(0)
}

#[status_script(agent = "metaknight", status = FIGHTER_STATUS_KIND_GLIDE_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn glide_attack_a(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_attack"), -1.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(glide_attack_b as *const () as _))
}

unsafe extern "C" fn glide_attack_b(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_fall_common();
    WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_JUMP_FLY_NEXT);
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("glide_attack") && MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    L2CValue::I32(0)
}

#[status_script(agent = "metaknight", status = FIGHTER_STATUS_KIND_GLIDE_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn glide_end_a(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_end"), -1.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(glide_end_b as *const () as _))
}

unsafe extern "C" fn glide_end_b(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_air_check_fall_common();
    WorkModule::enable_transition_term_group(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("glide_end") && MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    L2CValue::I32(0)
}

#[status_script(agent = "metaknight", status = FIGHTER_STATUS_KIND_GLIDE_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn glide_landing_a(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ENTRY_ID = get_entry_id(fighter.module_accessor);
    ANGLE[ENTRY_ID] = 0.0;
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_landing"), -1.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(glide_landing_b as *const () as _))
}

unsafe extern "C" fn glide_landing_b(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("glide_landing") && MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_DOWN_WAIT.into(), false.into());
    }
    L2CValue::I32(0)
}