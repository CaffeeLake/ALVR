mod props;
use alvr_common::{once_cell::sync::Lazy, parking_lot::RwLock, warn};
use alvr_packets::Haptics;
pub use props::*;

use crate::{
    input_mapping, logging_backend, FfiFov, FfiViewsConfig, ServerCoreContext, ServerCoreEvent,
    SERVER_DATA_MANAGER,
};
use std::{
    ffi::{c_char, c_void},
    thread,
    time::{Duration, Instant},
};

static SERVER_CORE_CONTEXT: Lazy<RwLock<Option<ServerCoreContext>>> = Lazy::new(|| {
    logging_backend::init_logging();

    RwLock::new(Some(ServerCoreContext::new()))
});

extern "C" fn driver_ready_idle(set_default_chap: bool) {
    thread::spawn(move || {
        unsafe { crate::InitOpenvrClient() };

        if set_default_chap {
            // call this when inside a new thread. Calling this on the parent thread will crash
            // SteamVR
            unsafe {
                crate::SetChaperoneArea(2.0, 2.0);
            }
        }

        if let Some(context) = &*SERVER_CORE_CONTEXT.read() {
            context.start_connection();
        }

        let mut last_resync = Instant::now();
        loop {
            let event = if let Some(context) = &*SERVER_CORE_CONTEXT.read() {
                match context.poll_event() {
                    Some(event) => event,
                    None => {
                        thread::sleep(Duration::from_millis(5));
                        continue;
                    }
                }
            } else {
                break;
            };

            match event {
                ServerCoreEvent::ClientConnected => {
                    unsafe {
                        crate::InitializeStreaming();
                        crate::RequestDriverResync();
                    };
                }
                ServerCoreEvent::ClientDisconnected => unsafe { crate::DeinitializeStreaming() },
                ServerCoreEvent::Battery(info) => unsafe {
                    crate::SetBattery(info.device_id, info.gauge_value, info.is_plugged)
                },
                ServerCoreEvent::PlayspaceSync(bounds) => unsafe {
                    crate::SetChaperoneArea(bounds.x, bounds.y)
                },
                ServerCoreEvent::ViewsConfig(config) => unsafe {
                    crate::SetViewsConfig(FfiViewsConfig {
                        fov: [
                            FfiFov {
                                left: config.fov[0].left,
                                right: config.fov[0].right,
                                up: config.fov[0].up,
                                down: config.fov[0].down,
                            },
                            FfiFov {
                                left: config.fov[1].left,
                                right: config.fov[1].right,
                                up: config.fov[1].up,
                                down: config.fov[1].down,
                            },
                        ],
                        // todo: send full matrix to steamvr
                        ipd_m: config.local_view_transforms[1].position.x
                            - config.local_view_transforms[0].position.x,
                    });
                },
                ServerCoreEvent::RequestIDR => unsafe { crate::RequestIDR() },
                ServerCoreEvent::GameRenderLatencyFeedback(game_latency) => {
                    if cfg!(target_os = "linux") && game_latency.as_secs_f32() > 0.25 {
                        let now = Instant::now();
                        if now.saturating_duration_since(last_resync).as_secs_f32() > 0.1 {
                            last_resync = now;
                            warn!("Desync detected. Attempting recovery.");
                            unsafe {
                                crate::RequestDriverResync();
                            }
                        }
                    }
                }
                ServerCoreEvent::ShutdownPending => {
                    SERVER_CORE_CONTEXT.write().take();

                    unsafe { crate::ShutdownSteamvr() };
                }
                ServerCoreEvent::RestartPending => {
                    if let Some(context) = SERVER_CORE_CONTEXT.write().take() {
                        context.restart();
                    }

                    unsafe { crate::ShutdownSteamvr() };
                }
            }
        }

        unsafe { crate::ShutdownOpenvrClient() };
    });
}

extern "C" fn send_haptics(device_id: u64, duration_s: f32, frequency: f32, amplitude: f32) {
    if let Some(context) = &*SERVER_CORE_CONTEXT.read() {
        let haptics = Haptics {
            device_id,
            duration: Duration::from_secs_f32(f32::max(duration_s, 0.0)),
            frequency,
            amplitude,
        };

        context.send_haptics(haptics);
    }
}

extern "C" fn wait_for_vsync() {
    // NB: don't sleep while locking SERVER_DATA_MANAGER or SERVER_CORE_CONTEXT
    let sleep_duration = if SERVER_DATA_MANAGER
        .read()
        .settings()
        .video
        .optimize_game_render_latency
    {
        SERVER_CORE_CONTEXT
            .read()
            .as_ref()
            .and_then(|ctx| ctx.duration_until_next_vsync())
    } else {
        None
    };

    if let Some(duration) = sleep_duration {
        thread::sleep(duration);
    }
}

pub extern "C" fn shutdown_driver() {
    SERVER_CORE_CONTEXT.write().take();
}

/// This is the SteamVR/OpenVR entry point
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn HmdDriverFactory(
    interface_name: *const c_char,
    return_code: *mut i32,
) -> *mut c_void {
    // Make sure the context is initialized, and initialize logging
    SERVER_CORE_CONTEXT.read().as_ref();

    crate::DriverReadyIdle = Some(driver_ready_idle);
    crate::GetSerialNumber = Some(get_serial_number);
    crate::SetOpenvrProps = Some(set_device_openvr_props);
    crate::RegisterButtons = Some(input_mapping::register_buttons);
    crate::HapticsSend = Some(send_haptics);
    crate::ShutdownRuntime = Some(shutdown_driver);
    crate::WaitForVSync = Some(wait_for_vsync);

    crate::CppOpenvrEntryPoint(interface_name, return_code)
}