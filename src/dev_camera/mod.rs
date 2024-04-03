use std::f32::consts::PI;

use bevy::prelude::*;
use moonshine_save::prelude::*;

use crate::{save_load::{SaveRequest, LoadRequest}, helpers::easing::ease_in_out_cubic};

const SAVE_FILE: &str = "saves/dev_camera.ron";

pub struct DevCameraPlugin;

impl Plugin for DevCameraPlugin {
    fn build(&self, app: &mut App) {
        app        
            .register_type::<DevCameraTransform>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (calculate_movement, calculate_rotation, update_camera));
    }
}

#[derive(Component)]
struct DevCamera;


#[derive(Component, Reflect)]
#[reflect(Component)]
struct DevCameraTransform {
    pos: Vec3,
    rot: Quat,
    target_rot: i32,
    current_rot: i32,
    time_changed: f32,
}

impl Default for DevCameraTransform {
    fn default() -> Self {
        Self { target_rot: 0, current_rot: 0, time_changed: 0., pos: Vec3::ZERO, rot: Quat::IDENTITY }
    }
}

#[derive(Bundle, Default)]
struct DevCameraBundle {
    dev_cam: DevCameraTransform,
    save: Save,
}

#[derive(Component)]
struct DevCameraParent;

fn spawn_camera(mut commands: Commands){
    commands.spawn(DevCameraBundle::default());
    commands.spawn(TransformBundle::default()).with_children(|builder|{
        builder.spawn(Camera3dBundle {
            transform: Transform {
                translation: Vec3::new(0., 15., 15.),
                rotation: Quat::from_euler(EulerRot::YXZ, 0.*PI, -0.25*PI, 0.0*PI),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(DevCamera);
    }).insert(DevCameraParent);

    commands.insert_resource(LoadRequest(SAVE_FILE.to_string()));
}

fn calculate_movement(
    mut save_qry: Query<&mut DevCameraTransform>,
    mut cam_qry: Query<&mut Transform, With<DevCameraParent>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
){
    let mut detes = save_qry.get_single_mut().unwrap();
    let cam = cam_qry.get_single_mut().unwrap();
    let multip = 2.;
    let speed = 20.;

    let right = (keys.pressed(KeyCode::D) as i32 - keys.pressed(KeyCode::A) as i32) as f32;
    let forward = (keys.pressed(KeyCode::W) as i32 - keys.pressed(KeyCode::S) as i32) as f32;
    let up = (keys.pressed(KeyCode::Space) as i32 - keys.pressed(KeyCode::ControlLeft) as i32) as f32;

    let d = time.delta_seconds();

    let mut fw = cam.forward();
    fw.y = 0.;

    let mut trans = ( fw.normalize() * forward + cam.right() * right + Vec3::Y * up) * d * speed;

    if keys.pressed(KeyCode::ShiftLeft) {
        trans *= multip;
    }

    detes.pos += trans;
}

fn calculate_rotation(
    mut qry: Query<&mut DevCameraTransform>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
){
    const TIME_TO_ROTATE: f32 = 0.2;
    const ROTATION_OFFSET: f32 = 0.25 * PI;
    let mut detes = qry.get_single_mut().unwrap();

    if detes.current_rot == detes.target_rot {
        if keys.just_pressed(KeyCode::E) {
            detes.target_rot += 1;
            detes.time_changed = time.elapsed_seconds();
        }
        if keys.just_pressed(KeyCode::Q) {
            detes.target_rot -= 1;
            detes.time_changed = time.elapsed_seconds();
        }
    } else {
        let dtime = detes.time_changed - time.elapsed_seconds();

        if dtime < -TIME_TO_ROTATE {
            detes.current_rot = detes.target_rot;
            detes.rot = Quat::from_euler(EulerRot::YXZ, ROTATION_OFFSET + detes.target_rot as f32 * PI/2.0, 0. * PI, 0. * PI);
        } else {
            let t = ease_in_out_cubic(-dtime / TIME_TO_ROTATE);
            let current_rot_quat = Quat::from_euler(EulerRot::YXZ, ROTATION_OFFSET + (detes.current_rot as f32) * PI/2.0, 0. * PI, 0. * PI);
            let target_rot_quat = Quat::from_euler(EulerRot::YXZ, ROTATION_OFFSET + (detes.target_rot as f32) * PI/2.0, 0. * PI, 0. * PI);
            detes.rot = current_rot_quat.slerp(target_rot_quat, t);
        }
    }
}

fn update_camera(
    mut commands: Commands,
    qry_devtf: Query<&DevCameraTransform>,
    mut qry_devcam: Query<&mut Transform, With<DevCameraParent>> 
){
    let mut cam_tf = qry_devcam.get_single_mut().unwrap();
    let dev_tf = qry_devtf.get_single().unwrap();

    if (cam_tf.translation == dev_tf.pos) && 
        (cam_tf.rotation == dev_tf.rot) {
        return;
    }

    cam_tf.translation = dev_tf.pos;
    cam_tf.rotation = dev_tf.rot;

    commands.insert_resource(SaveRequest(SAVE_FILE.to_string()));
}