use std::net::SocketAddr;
use std::time::Duration;

use bevy::app::{Events, ManualEventReader, ScheduleRunnerSettings};
use bevy::prelude::*;
use bevy_networking_turbulence::{NetworkEvent, NetworkResource, NetworkingPlugin, Packet};

const SERVER_PORT: u16 = 14191;


pub struct NetPlugin;
impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
                1.0 / 60.0
            )))
            .add_plugin(NetworkingPlugin::default())
            //arg handler?
            .add_startup_system(startup.system())
            .run();
    }
}

fn startup(mut net: ResMut<NetworkResource>) {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            // set the following address to your server address (i.e. local machine)
            // and remove compile_error! line
            let mut server_address: SocketAddr = "127.0.0.1".parse().unwrap();
            server_address.set_port(SERVER_PORT);
        } else {
            let ip_address =
                bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");
            let server_address = SocketAddr::new(ip_address, SERVER_PORT);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
        info!("Starting client");
        net.connect(server_address);
}
